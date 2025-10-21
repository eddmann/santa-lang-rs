mod builtins;
mod environment;
mod function;
mod index;
mod infix;
mod lazy_sequence;
mod matcher;
mod object;

#[cfg(test)]
mod tests;

pub use crate::evaluator::environment::{Environment, EnvironmentErr, EnvironmentRef};
use crate::evaluator::function::Function;
pub use crate::evaluator::function::{Arguments, ExternalFnDef};
use crate::evaluator::lazy_sequence::LazySequence;
pub use crate::evaluator::object::Object;
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Prefix, Program, Statement, StatementKind};
use im_rc::{HashMap, HashSet, Vector};
use std::rc::Rc;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RuntimeErr {
    pub message: String,
    pub source: Location,
    pub trace: Vec<Location>,
}

pub type Evaluation = Result<Rc<Object>, RuntimeErr>;
type ExternalFnLookup = std::collections::HashMap<String, Rc<Object>>;

#[derive(Debug)]
pub struct Evaluator {
    frames: Vec<Frame>,
    external_functions: Option<ExternalFnLookup>,
}

#[derive(Debug)]
pub enum Frame {
    Program {
        environment: EnvironmentRef,
    },
    Block {
        source: Location,
        environment: EnvironmentRef,
    },
    ClosureCall {
        source: Location,
        environment: EnvironmentRef,
    },
    BuiltinCall {
        source: Location,
    },
    ExternalCall {
        source: Location,
    },
}

impl Evaluator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            frames: vec![],
            external_functions: None,
        }
    }

    pub fn new_with_external_functions(external_function_defs: &[ExternalFnDef]) -> Self {
        let external_functions: ExternalFnLookup = external_function_defs
            .iter()
            .map(|(name, parameters, body)| {
                (
                    name.to_owned(),
                    Rc::new(Object::Function(Function::External {
                        parameters: parameters.clone(),
                        body: Rc::clone(body),
                        partial: None,
                    })),
                )
            })
            .collect();

        Self {
            frames: vec![],
            external_functions: Some(external_functions),
        }
    }

    pub fn evaluate(&mut self, program: &Program) -> Evaluation {
        self.push_frame(Frame::Program {
            environment: Environment::new(),
        });
        let result = self.eval_statement_block(&program.statements)?;
        self.pop_frame();
        Ok(result)
    }

    pub fn evaluate_with_environment(&mut self, program: &Program, environment: EnvironmentRef) -> Evaluation {
        self.push_frame(Frame::Program { environment });
        let result = self.eval_statement_block(&program.statements)?;
        self.pop_frame();
        Ok(result)
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) {
        self.frames.pop();
    }

    fn environment(&self) -> EnvironmentRef {
        // Walk up the frame stack to find the first frame with an environment.
        // BuiltinCall and ExternalCall frames don't have environments, so we skip them.
        for frame in self.frames.iter().rev() {
            match frame {
                Frame::Program { environment } => return Rc::clone(environment),
                Frame::Block { environment, .. } => return Rc::clone(environment),
                Frame::ClosureCall { environment, .. } => return Rc::clone(environment),
                Frame::BuiltinCall { .. } | Frame::ExternalCall { .. } => continue,
            }
        }

        // This should never happen as the bottom frame is always Program with an environment
        panic!("No frame with environment found in stack - this indicates a bug in the evaluator");
    }

    fn eval_statement_block(&mut self, block: &[Statement]) -> Evaluation {
        let mut result = Rc::new(Object::Nil);

        for (index, statement) in block.iter().enumerate() {
            if let StatementKind::Comment(_) = statement.kind {
                continue;
            }

            if let StatementKind::Return(value) = &statement.kind {
                if let ExpressionKind::Call { function, arguments } = &value.kind {
                    for frame in self.frames.iter().rev() {
                        if let Frame::ClosureCall { source, .. } = &frame {
                            if function.source == *source {
                                return Ok(Rc::new(Object::Function(Function::Continuation {
                                    arguments: self.eval_expressions(arguments)?,
                                })));
                            }
                            break;
                        }
                    }
                }
            }

            if index == block.len() - 1 {
                if let StatementKind::Expression(expression) = &statement.kind {
                    if let ExpressionKind::Call { function, arguments } = &expression.kind {
                        for frame in self.frames.iter().rev() {
                            if let Frame::ClosureCall { source, .. } = &frame {
                                if function.source == *source {
                                    return Ok(Rc::new(Object::Function(Function::Continuation {
                                        arguments: self.eval_expressions(arguments)?,
                                    })));
                                }
                                break;
                            }
                        }
                    }
                }
            }

            result = self.eval_statement(statement)?;

            if matches!(&*result, Object::Return(_) | Object::Break(_)) {
                return Ok(result);
            }
        }

        Ok(Rc::clone(&result))
    }

    fn eval_statement(&mut self, statement: &Statement) -> Evaluation {
        match &statement.kind {
            StatementKind::Return(value) => Ok(Rc::new(Object::Return(self.eval_expression(value)?))),
            StatementKind::Break(value) => Ok(Rc::new(Object::Break(self.eval_expression(value)?))),
            StatementKind::Comment(_) => Ok(Rc::new(Object::Nil)),
            StatementKind::Section { name, body } => {
                self.environment()
                    .borrow_mut()
                    .add_section(name, Rc::new(*body.clone()));
                Ok(Rc::new(Object::Nil))
            }
            StatementKind::Expression(expression) => self.eval_expression(expression),
            StatementKind::Block(statements) => {
                self.push_frame(Frame::Block {
                    source: statement.source,
                    environment: Environment::from(self.environment()),
                });
                let result = self.eval_statement_block(statements)?;
                self.pop_frame();
                Ok(result)
            }
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Evaluation {
        match &expression.kind {
            ExpressionKind::Let { name, value } => self.eval_let_expression(name, value),
            ExpressionKind::MutableLet { name, value } => self.eval_mutable_let_expression(name, value),
            ExpressionKind::Assign { name, value } => self.eval_assign_expression(name, value),
            ExpressionKind::Identifier(name) => {
                if let Some(value) = self.environment().borrow().get_variable(name) {
                    return Ok(value);
                }

                if let Some(builtin) = crate::evaluator::builtins::builtins(name) {
                    return Ok(builtin);
                }

                if let Some(builtin) = crate::evaluator::builtins::builtin_aliases(name) {
                    return Ok(builtin);
                }

                if let Some(external_functions) = &self.external_functions {
                    if let Some(external) = external_functions.get(name) {
                        return Ok(Rc::clone(external));
                    }
                }

                Err(RuntimeErr {
                    message: format!("Identifier can not be found: {}", name),
                    source: expression.source,
                    trace: self.get_trace(),
                })
            }
            ExpressionKind::Integer(value) => Ok(Rc::new(Object::Integer(*value))),
            ExpressionKind::Decimal(value) => Ok(Rc::new(Object::Decimal(*value))),
            ExpressionKind::String(value) => Ok(Rc::new(Object::String(value.to_owned()))),
            ExpressionKind::Boolean(value) => Ok(Rc::new(Object::Boolean(*value))),
            ExpressionKind::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(condition, consequence, alternative),
            ExpressionKind::Match { subject, cases } => crate::evaluator::matcher::matcher(self, subject, cases),
            ExpressionKind::Function { parameters, body } => Ok(Rc::new(Object::Function(Function::Closure {
                parameters: parameters.clone(),
                body: Rc::clone(body),
                environment: Rc::clone(&self.environment()),
            }))),
            ExpressionKind::Call { function, arguments } => {
                let evaluated_function = self.eval_expression(function)?;

                if let Object::Function(func) = &*evaluated_function {
                    let evaluated_arguments = self.eval_expressions(arguments)?;
                    return func.apply(self, evaluated_arguments, function.source);
                }

                Err(RuntimeErr {
                    message: format!("Expected a Function, found: {}", evaluated_function.name()),
                    source: function.source,
                    trace: self.get_trace(),
                })
            }
            ExpressionKind::List(list) => Ok(Rc::new(Object::List(Vector::from(
                self.eval_expressions(list)?
                    .into_iter()
                    .map(|obj| (*obj).clone())
                    .collect::<Vec<_>>()
            )))),
            ExpressionKind::Set(set) => {
                let mut elements = HashSet::default();
                for element in self.eval_expressions(set)? {
                    if !element.is_hashable() {
                        return Err(RuntimeErr {
                            message: format!("Unable to include a {} within an Set", element.name()),
                            source: expression.source,
                            trace: self.get_trace(),
                        });
                    }
                    elements.insert((*element).clone());
                }
                Ok(Rc::new(Object::Set(elements)))
            }
            ExpressionKind::Dictionary(map) => {
                let mut elements = HashMap::default();
                for (key, value) in map {
                    let evaluated_key = self.eval_expression(key)?;
                    if !evaluated_key.is_hashable() {
                        return Err(RuntimeErr {
                            message: format!("Unable to use a {} as a Dictionary key", evaluated_key.name()),
                            source: key.source,
                            trace: self.get_trace(),
                        });
                    }
                    elements.insert((*evaluated_key).clone(), (*self.eval_expression(value)?).clone());
                }
                Ok(Rc::new(Object::Dictionary(elements)))
            }
            ExpressionKind::Index { left, index } => {
                let evaluated_left = self.eval_expression(left)?;
                let evaluated_index = self.eval_expression(index)?;
                crate::evaluator::index::lookup(self, evaluated_left, evaluated_index, index.source)
            }
            ExpressionKind::FunctionThread { initial, functions } => {
                let mut result = self.eval_expression(initial)?;

                for function in functions {
                    let evaluated_function = self.eval_expression(function)?;

                    if let Object::Function(f) = &*evaluated_function {
                        result = f.apply(self, vec![result], function.source)?;
                        continue;
                    }

                    return Err(RuntimeErr {
                        message: format!("Expected a Function, found: {}", evaluated_function.name()),
                        source: function.source,
                        trace: self.get_trace(),
                    });
                }

                Ok(result)
            }
            ExpressionKind::FunctionComposition(functions) => {
                let mut evaluated_functions = Vec::with_capacity(functions.len());

                for function in functions {
                    let evaluated_function = self.eval_expression(function)?;

                    if let Object::Function(f) = &*evaluated_function {
                        evaluated_functions.push(f.clone());
                        continue;
                    }

                    return Err(RuntimeErr {
                        message: format!("Expected a Function, found: {}", evaluated_function.name()),
                        source: function.source,
                        trace: self.get_trace(),
                    });
                }

                Ok(Rc::new(Object::Function(Function::Composition {
                    functions: evaluated_functions,
                })))
            }
            ExpressionKind::InclusiveRange { from, to } => {
                match (&*self.eval_expression(from)?, &*self.eval_expression(to)?) {
                    (Object::Integer(from), Object::Integer(to)) => {
                        Ok(Rc::new(Object::LazySequence(LazySequence::inclusive_range(*from, *to))))
                    }
                    (from, to) => Err(RuntimeErr {
                        message: format!(
                            "Expected Integer inclusive range, found: {}..={}",
                            from.name(),
                            to.name()
                        ),
                        source: expression.source,
                        trace: self.get_trace(),
                    }),
                }
            }
            ExpressionKind::ExclusiveRange { from, until } => {
                match (&*self.eval_expression(from)?, &*self.eval_expression(until)?) {
                    (Object::Integer(from), Object::Integer(until)) => Ok(Rc::new(Object::LazySequence(
                        LazySequence::exclusive_range(*from, *until),
                    ))),
                    (from, until) => Err(RuntimeErr {
                        message: format!(
                            "Expected Integer inclusive range, found: {}..{}",
                            from.name(),
                            until.name()
                        ),
                        source: expression.source,
                        trace: self.get_trace(),
                    }),
                }
            }
            ExpressionKind::UnboundedRange { from } => match &*self.eval_expression(from)? {
                Object::Integer(from) => Ok(Rc::new(Object::LazySequence(LazySequence::unbounded_range(*from)))),
                from => Err(RuntimeErr {
                    message: format!("Expected Integer unbounded range, found: {}..", from.name()),
                    source: expression.source,
                    trace: self.get_trace(),
                }),
            },
            ExpressionKind::Infix { left, operator, right } => {
                crate::evaluator::infix::apply(self, left, operator, right, expression.source)
            }
            ExpressionKind::Prefix { operator, right } => match (&operator, &*self.eval_expression(right)?) {
                (Prefix::Bang, object) => Ok(Rc::new(Object::Boolean(!object.is_truthy()))),
                (Prefix::Minus, Object::Integer(v)) => Ok(Rc::new(Object::Integer(-v))),
                (Prefix::Minus, Object::Decimal(v)) => Ok(Rc::new(Object::Decimal(-v))),
                (Prefix::Minus, object) => Err(RuntimeErr {
                    message: format!("Unexpected prefix operation: -{}", object.name()),
                    source: right.source,
                    trace: self.get_trace(),
                }),
            },
            ExpressionKind::Nil => Ok(Rc::new(Object::Nil)),
            ExpressionKind::Placeholder => Ok(Rc::new(Object::Placeholder)),
            ExpressionKind::Spread(_) => Err(RuntimeErr {
                message: "Unable to spread within this context".to_owned(),
                source: expression.source,
                trace: self.get_trace(),
            }),
            _ => Err(RuntimeErr {
                message: format!("Unimplemented expression: {}", expression),
                source: expression.source,
                trace: self.get_trace(),
            }),
        }
    }

    fn eval_let_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .environment()
                    .borrow_mut()
                    .declare_variable(id, Rc::clone(&evaluated_value), false)
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                        trace: self.get_trace(),
                    }),
                }
            }
            ExpressionKind::IdentifierListPattern(pattern) => {
                self.destructure_let_list_pattern(pattern, evaluated_value, false, name.source)
            }
            _ => Err(RuntimeErr {
                message: format!("Unexpected Let identifier, found: {}", name.kind),
                source: name.source,
                trace: self.get_trace(),
            }),
        }
    }

    fn eval_mutable_let_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .environment()
                    .borrow_mut()
                    .declare_variable(id, Rc::clone(&evaluated_value), true)
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                        trace: self.get_trace(),
                    }),
                }
            }
            ExpressionKind::IdentifierListPattern(pattern) => {
                self.destructure_let_list_pattern(pattern, evaluated_value, true, name.source)
            }
            _ => Err(RuntimeErr {
                message: format!("Unexpected Let identifier, found: {}", name.kind),
                source: name.source,
                trace: self.get_trace(),
            }),
        }
    }

    fn eval_assign_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .environment()
                    .borrow_mut()
                    .assign_variable(id, Rc::clone(&evaluated_value))
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                        trace: self.get_trace(),
                    }),
                }
            }
            _ => Ok(Rc::new(Object::Nil)),
        }
    }

    fn eval_if_expression(
        &mut self,
        condition: &Expression,
        consequence: &Statement,
        alternative: &Option<Box<Statement>>,
    ) -> Evaluation {
        let evaluated_condition = self.eval_expression(condition)?;

        if evaluated_condition.is_truthy() {
            self.eval_statement(consequence)
        } else if let Some(alternative) = alternative {
            self.eval_statement(alternative)
        } else {
            Ok(Rc::new(Object::Nil))
        }
    }

    fn eval_expressions(&mut self, expressions: &[Expression]) -> Result<Vec<Rc<Object>>, RuntimeErr> {
        let mut results = Vec::with_capacity(expressions.len());

        for expression in expressions {
            if let ExpressionKind::Spread(value) = &expression.kind {
                if let Object::List(list) = &*self.eval_expression(value)? {
                    for element in list {
                        results.push(Rc::new(element.clone()));
                    }
                    continue;
                }

                return Err(RuntimeErr {
                    message: format!("Expected a List to spread, found: {}", expression),
                    source: expression.source,
                    trace: self.get_trace(),
                });
            }

            results.push(self.eval_expression(expression)?);
        }

        Ok(results)
    }

    fn destructure_let_list_pattern(
        &mut self,
        pattern: &[Expression],
        subject: Rc<Object>,
        is_mutable: bool,
        source: Location,
    ) -> Evaluation {
        let list = match &*subject {
            Object::List(list) => list,
            _ => {
                return Err(RuntimeErr {
                    message: format!("Expected a List to destructure, found: {}", subject.name()),
                    source,
                    trace: self.get_trace(),
                })
            }
        };

        for (position, pattern) in pattern.iter().enumerate() {
            match &pattern.kind {
                ExpressionKind::Identifier(name) => {
                    match self.environment().borrow_mut().declare_variable(
                        name,
                        Rc::new(list.iter().nth(position).unwrap_or(&Object::Nil).clone()),
                        is_mutable,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: pattern.source,
                                trace: self.get_trace(),
                            })
                        }
                    }
                }
                ExpressionKind::RestIdentifier(name) => {
                    match self.environment().borrow_mut().declare_variable(
                        name,
                        Rc::new(Object::List(list.clone().into_iter().skip(position).collect())),
                        is_mutable,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: pattern.source,
                                trace: self.get_trace(),
                            })
                        }
                    }
                    break;
                }
                ExpressionKind::Placeholder => {
                    continue;
                }
                ExpressionKind::IdentifierListPattern(next_pattern) => {
                    let object = if let Some(value) = list.iter().nth(position) {
                        Rc::new(value.clone())
                    } else {
                        Rc::new(Object::List(Vector::new()))
                    };
                    self.destructure_let_list_pattern(next_pattern, object, is_mutable, pattern.source)?;
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unexpected List destructing pattern, found: {}", pattern.kind),
                        source: pattern.source,
                        trace: self.get_trace(),
                    })
                }
            }
        }

        Ok(subject)
    }

    pub fn get_trace(&self) -> Vec<Location> {
        self.frames
            .iter()
            .rev()
            .filter_map(|frame| match frame {
                Frame::ClosureCall { source, .. } | Frame::BuiltinCall { source } | Frame::ExternalCall { source } => {
                    Some(*source)
                }
                _ => None,
            })
            .collect()
    }
}
