use super::environment::{Environment, EnvironmentRef};
use crate::evaluator::{EnvironmentErr, Evaluation, Evaluator, Frame, Object, RuntimeErr};
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Statement};
use im_rc::Vector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

pub type Arguments = HashMap<String, Rc<Object>>;
pub type ExternalFnDef = (String, Vec<ExpressionKind>, ExternalFn);

type BuiltinFn = fn(&mut Evaluator, Arguments, Location) -> Evaluation;
type ExternalFn = Rc<dyn Fn(Arguments, Location) -> Evaluation>;
type MemoizedCache = Rc<RefCell<HashMap<Vec<Rc<Object>>, Rc<Object>>>>;

#[derive(Clone)]
pub enum Function {
    Closure {
        parameters: Vec<Expression>,
        body: Statement,
        environment: EnvironmentRef,
    },
    MemoizedClosure {
        parameters: Vec<Expression>,
        body: Statement,
        environment: EnvironmentRef,
        cache: MemoizedCache,
    },
    Builtin {
        parameters: Vec<ExpressionKind>,
        body: BuiltinFn,
        partial: Option<Arguments>,
    },
    External {
        parameters: Vec<ExpressionKind>,
        body: ExternalFn,
        partial: Option<Arguments>,
    },
    Composition {
        functions: Vec<Function>,
    },
    Continuation {
        arguments: Vec<Rc<Object>>,
    },
}

impl Function {
    pub fn apply(&self, evaluator: &mut Evaluator, arguments: Vec<Rc<Object>>, source: Location) -> Evaluation {
        match self {
            Self::Closure {
                parameters,
                body,
                environment,
            } => {
                let enclosed_enviornment = Environment::from(Rc::clone(environment));
                let remaining_parameters =
                    self.assign_closure_parameters(Rc::clone(&enclosed_enviornment), parameters, &arguments)?;

                if !remaining_parameters.is_empty() {
                    return Ok(Rc::new(Object::Function(Self::Closure {
                        parameters: remaining_parameters,
                        body: body.clone(),
                        environment: enclosed_enviornment,
                    })));
                }

                evaluator.push_frame(Frame::ClosureCall {
                    source,
                    environment: Rc::clone(&enclosed_enviornment),
                });

                let mut result = evaluator.eval_statement(body)?;

                loop {
                    if let Object::Function(Function::Continuation { arguments }) = &*result {
                        let remaining_parameters =
                            self.assign_closure_parameters(Rc::clone(&enclosed_enviornment), parameters, arguments)?;

                        if !remaining_parameters.is_empty() {
                            result = Rc::new(Object::Function(Self::Closure {
                                parameters: remaining_parameters,
                                body: body.clone(),
                                environment: enclosed_enviornment,
                            }));
                            break;
                        }

                        result = evaluator.eval_statement(body)?;
                        continue;
                    }

                    break;
                }

                evaluator.pop_frame();

                Ok(result)
            }
            Self::MemoizedClosure {
                parameters,
                body,
                environment,
                cache,
            } => {
                if let Some(result) = cache.borrow().get(&arguments) {
                    return Ok(Rc::clone(result));
                }

                let enclosed_enviornment = Environment::from(Rc::clone(environment));
                let remaining_parameters =
                    self.assign_closure_parameters(Rc::clone(&enclosed_enviornment), parameters, &arguments)?;

                if !remaining_parameters.is_empty() {
                    return Ok(Rc::new(Object::Function(Self::Closure {
                        parameters: remaining_parameters,
                        body: body.clone(),
                        environment: enclosed_enviornment,
                    })));
                }

                evaluator.push_frame(Frame::ClosureCall {
                    source,
                    environment: Rc::clone(&enclosed_enviornment),
                });

                let mut result = evaluator.eval_statement(body)?;

                loop {
                    if let Object::Function(Function::Continuation { arguments }) = &*result {
                        let remaining_parameters =
                            self.assign_closure_parameters(Rc::clone(&enclosed_enviornment), parameters, arguments)?;

                        if !remaining_parameters.is_empty() {
                            result = Rc::new(Object::Function(Self::Closure {
                                parameters: remaining_parameters,
                                body: body.clone(),
                                environment: enclosed_enviornment,
                            }));
                            break;
                        }

                        result = evaluator.eval_statement(body)?;
                        continue;
                    }

                    break;
                }

                evaluator.pop_frame();

                cache.borrow_mut().insert(arguments, Rc::clone(&result));

                Ok(result)
            }
            Self::Builtin {
                parameters,
                body,
                partial,
            } => {
                let mut evaluated_arguments = match partial {
                    Some(args) => args.clone(),
                    None => HashMap::new(),
                };

                let remaining_parameters =
                    self.assign_interal_parameters(&mut evaluated_arguments, parameters, &arguments)?;

                if !remaining_parameters.is_empty() {
                    return Ok(Rc::new(Object::Function(Self::Builtin {
                        parameters: remaining_parameters,
                        body: *body,
                        partial: Some(evaluated_arguments),
                    })));
                }

                evaluator.push_frame(Frame::BuiltinCall { source });

                let result = body(evaluator, evaluated_arguments, source)?;

                evaluator.pop_frame();

                Ok(result)
            }
            Self::External {
                parameters,
                body,
                partial,
            } => {
                let mut evaluated_arguments = match partial {
                    Some(args) => args.clone(),
                    None => HashMap::new(),
                };

                let remaining_parameters =
                    self.assign_interal_parameters(&mut evaluated_arguments, parameters, &arguments)?;

                if !remaining_parameters.is_empty() {
                    return Ok(Rc::new(Object::Function(Self::External {
                        parameters: remaining_parameters,
                        body: Rc::clone(body),
                        partial: Some(evaluated_arguments),
                    })));
                }

                evaluator.push_frame(Frame::ExternalCall { source });

                let result = body(evaluated_arguments, source)?;

                evaluator.pop_frame();

                Ok(result)
            }
            Self::Composition { functions } => {
                let mut result = Rc::clone(&arguments[0]);

                for function in functions {
                    result = function.apply(evaluator, vec![result], source)?;
                }

                Ok(result)
            }
            Self::Continuation { .. } => unreachable!(),
        }
    }

    fn assign_closure_parameters(
        &self,
        enviornment: EnvironmentRef,
        #[allow(clippy::ptr_arg)] parameters: &Vec<Expression>,
        arguments: &Vec<Rc<Object>>,
    ) -> Result<Vec<Expression>, RuntimeErr> {
        let mut remaining_parameters = vec![];

        for (position, (parameter, argument)) in parameters.iter().zip(arguments.iter()).enumerate() {
            if let Object::Placeholder = **argument {
                remaining_parameters.push(parameter.clone());
                continue;
            }

            match &parameter.kind {
                ExpressionKind::Identifier(name) => {
                    enviornment.borrow_mut().set_variable(name, Rc::clone(argument));
                }
                ExpressionKind::RestIdentifier(name) => {
                    enviornment.borrow_mut().set_variable(
                        name,
                        Rc::new(Object::List(arguments.clone().into_iter().skip(position).collect())),
                    );
                    break;
                }
                ExpressionKind::IdentifierListPattern(pattern) => {
                    Self::destructure_list_pattern_parameter(
                        Rc::clone(&enviornment),
                        pattern,
                        Rc::clone(argument),
                        parameter.source,
                    )?;
                }
                ExpressionKind::Placeholder => {
                    continue;
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unexpected parameter, found: {}", parameter.kind),
                        source: parameter.source,
                        trace: vec![],
                    })
                }
            }
        }

        remaining_parameters.append(&mut parameters.clone().into_iter().skip(arguments.len()).collect());

        Ok(remaining_parameters)
    }

    fn assign_interal_parameters(
        &self,
        evaluated_arguments: &mut Arguments,
        #[allow(clippy::ptr_arg)] parameters: &Vec<ExpressionKind>,
        arguments: &Vec<Rc<Object>>,
    ) -> Result<Vec<ExpressionKind>, RuntimeErr> {
        let mut remaining_parameters = vec![];

        for (position, (parameter, argument)) in parameters.iter().zip(arguments.iter()).enumerate() {
            if let Object::Placeholder = **argument {
                remaining_parameters.push(parameter.clone());
                continue;
            }

            match &parameter {
                ExpressionKind::Identifier(name) => {
                    evaluated_arguments.insert(name.to_owned(), Rc::clone(argument));
                }
                ExpressionKind::RestIdentifier(name) => {
                    evaluated_arguments.insert(
                        name.to_owned(),
                        Rc::new(Object::List(arguments.clone().into_iter().skip(position).collect())),
                    );
                    break;
                }
                ExpressionKind::Placeholder => {
                    continue;
                }
                _ => {
                    unreachable!()
                }
            }
        }

        remaining_parameters.append(&mut parameters.clone().into_iter().skip(arguments.len()).collect());

        Ok(remaining_parameters)
    }

    fn destructure_list_pattern_parameter(
        environment: EnvironmentRef,
        parameter: &[Expression],
        argument: Rc<Object>,
        source: Location,
    ) -> Evaluation {
        let list = match &*argument {
            Object::List(list) => list,
            _ => {
                return Err(RuntimeErr {
                    message: format!("Expected a List argument to destructure, found: {}", argument.name()),
                    source,
                    trace: vec![],
                })
            }
        };

        for (position, parameter) in parameter.iter().enumerate() {
            match &parameter.kind {
                ExpressionKind::Identifier(name) => {
                    match environment.borrow_mut().declare_variable(
                        name,
                        Rc::clone(list.iter().nth(position).unwrap_or(&Rc::new(Object::Nil))),
                        false,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: parameter.source,
                                trace: vec![],
                            })
                        }
                    }
                }
                ExpressionKind::RestIdentifier(name) => {
                    match environment.borrow_mut().declare_variable(
                        name,
                        Rc::new(Object::List(list.clone().into_iter().skip(position).collect())),
                        false,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: parameter.source,
                                trace: vec![],
                            })
                        }
                    }
                    break;
                }
                ExpressionKind::Placeholder => {
                    continue;
                }
                ExpressionKind::IdentifierListPattern(next_parameter) => {
                    Self::destructure_list_pattern_parameter(
                        Rc::clone(&environment),
                        next_parameter,
                        Rc::clone(
                            list.iter()
                                .nth(position)
                                .unwrap_or(&Rc::new(Object::List(Vector::new()))),
                        ),
                        parameter.source,
                    )?;
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unexpected List destructing pattern, found: {}", parameter.kind),
                        source: parameter.source,
                        trace: vec![],
                    })
                }
            }
        }

        Ok(argument)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Function::Closure { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [closure] }}", formatted.join(", "))
            }
            Function::MemoizedClosure { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [memoized] }}", formatted.join(", "))
            }
            Function::Builtin { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [builtin] }}", formatted.join(", "))
            }
            Function::External { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [external] }}", formatted.join(", "))
            }
            Function::Composition { .. } => "|a| { [composed] }".to_owned(),
            Function::Continuation { .. } => unreachable!(),
        };
        write!(f, "{}", s)
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Function {}

impl Ord for Function {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unreachable!()
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        unreachable!()
    }
}

impl Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        unreachable!()
    }
}
