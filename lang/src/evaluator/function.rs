use super::environment::{Environment, EnvironmentRef};
use crate::evaluator::{Evaluation, Evaluator, Frame, Object, RuntimeErr};
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Statement};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

pub type Arguments = HashMap<String, Rc<Object>>;
type BuiltinFn = fn(&mut Evaluator, Arguments, Location) -> Evaluation;
type ExternalFn = Rc<dyn Fn(&mut Evaluator, Arguments, Location) -> Evaluation>;

#[derive(Clone)]
pub enum Function {
    Closure {
        parameters: Vec<Expression>,
        body: Statement,
        environment: EnvironmentRef,
    },
    Composition {
        functions: Vec<Function>,
    },
    // MemoizedClosure {
    //     parameters: Vec<Expression>,
    //     body: Statement,
    //     environment: EnvironmentRef,
    //     cache: HashMap<Vec<Rc<Object>>, Rc<Object>>
    // },
    // ContinuationClosure {
    //     arguments: Vec<Rc<Object>>,
    //     body: Statement,
    //     environment: EnvironmentRef
    // },
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
                let mut remaining_parameters = vec![];

                for (position, (parameter, argument)) in parameters.iter().zip(arguments.iter()).enumerate() {
                    if let Object::Placeholder = **argument {
                        remaining_parameters.push(parameter.clone());
                        continue;
                    }

                    match &parameter.kind {
                        ExpressionKind::Identifier(name) => {
                            enclosed_enviornment
                                .borrow_mut()
                                .set_variable(name, Rc::clone(argument));
                        }
                        ExpressionKind::RestIdentifier(name) => {
                            enclosed_enviornment.borrow_mut().set_variable(
                                name,
                                Rc::new(Object::List(arguments.clone().into_iter().skip(position).collect())),
                            );
                            break;
                        }
                        ExpressionKind::Placeholder => {
                            continue;
                        }
                        _ => {
                            return Err(RuntimeErr {
                                message: "Unknown parameter type".to_owned(),
                                source: parameter.source,
                            })
                        }
                    }
                }

                remaining_parameters.append(&mut parameters.clone().into_iter().skip(arguments.len()).collect());

                if remaining_parameters.is_empty() {
                    evaluator.push_frame(Frame::ClosureCall {
                        source,
                        environment: enclosed_enviornment,
                    });
                    let result = evaluator.eval_statement(body)?;
                    evaluator.pop_frame();
                    return Ok(result);
                }

                Ok(Rc::new(Object::Function(Self::Closure {
                    parameters: remaining_parameters,
                    body: body.clone(),
                    environment: enclosed_enviornment,
                })))
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
                            panic!()
                        }
                    }
                }

                remaining_parameters.append(&mut parameters.clone().into_iter().skip(arguments.len()).collect());

                if remaining_parameters.is_empty() {
                    evaluator.push_frame(Frame::BuiltinCall { source });
                    let result = body(evaluator, evaluated_arguments, source)?;
                    evaluator.pop_frame();
                    return Ok(result);
                }

                Ok(Rc::new(Object::Function(Self::Builtin {
                    parameters: remaining_parameters,
                    body: *body,
                    partial: Some(evaluated_arguments),
                })))
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
                            panic!()
                        }
                    }
                }

                remaining_parameters.append(&mut parameters.clone().into_iter().skip(arguments.len()).collect());

                if remaining_parameters.is_empty() {
                    evaluator.push_frame(Frame::ExternalCall { source });
                    let result = body(evaluator, evaluated_arguments, source)?;
                    evaluator.pop_frame();
                    return Ok(result);
                }

                Ok(Rc::new(Object::Function(Self::External {
                    parameters: remaining_parameters,
                    body: Rc::clone(body),
                    partial: Some(evaluated_arguments),
                })))
            }
            Self::Composition { functions } => {
                let mut result = Rc::clone(&arguments[0]);

                for function in functions {
                    result = function.apply(evaluator, vec![result], source)?;
                }

                Ok(result)
            }
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Function::Closure { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [closure] }}", formatted.join(", "))
            }
            Function::Builtin { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [builtin] }}", formatted.join(", "))
            }
            Function::External { parameters, .. } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {{ [external] }}", formatted.join(", "))
            }
            Function::Composition { .. } => "|a| { [compose] }".to_owned(),
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
