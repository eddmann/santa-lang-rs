mod builtins;
mod environment;
mod function;
mod index;
mod infix;
mod lazy_sequence;
mod object;

#[cfg(test)]
mod tests;

use crate::evaluator::environment::{Environment, EnvironmentErr, EnvironmentRef};
use crate::evaluator::function::Function;
use crate::evaluator::lazy_sequence::LazySequence;
use crate::evaluator::object::Object;
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Prefix, Program, Statement, StatementKind};
use im_rc::{HashMap, HashSet, Vector};
use ordered_float::OrderedFloat;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeErr {
    pub message: String,
    pub source: Location,
}

pub type Evaluation = Result<Rc<Object>, RuntimeErr>;

#[derive(Debug)]
pub struct Evaluator {
    frames: Vec<Frame>,
    system_functions: Option<std::collections::HashMap<String, Function>>,
    nil: Rc<Object>,
    placeholder: Rc<Object>,
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
    pub fn new() -> Self {
        Evaluator {
            frames: vec![],
            system_functions: None,
            nil: Rc::new(Object::Nil),
            placeholder: Rc::new(Object::Placeholder),
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

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) {
        self.frames.pop();
    }

    fn enviornment(&self) -> EnvironmentRef {
        match &self.frames.last().unwrap() {
            Frame::Program { environment } => Rc::clone(environment),
            Frame::Block { environment, .. } => Rc::clone(environment),
            Frame::ClosureCall { environment, .. } => Rc::clone(environment),
            _ => panic!(),
        }
    }

    fn eval_statement_block(&mut self, block: &[Statement]) -> Evaluation {
        let mut result = Rc::clone(&self.nil);

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
            StatementKind::Comment(_) => Ok(Rc::clone(&self.nil)),
            StatementKind::Section { name, body } => {
                self.enviornment()
                    .borrow_mut()
                    .add_section(name, Rc::new(*body.clone()));
                Ok(Rc::clone(&self.nil))
            }
            StatementKind::Expression(expression) => self.eval_expression(expression),
            StatementKind::Block(statements) => {
                self.push_frame(Frame::Block {
                    source: statement.source,
                    environment: Environment::from(self.enviornment()),
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
                if let Some(value) = self.enviornment().borrow().get_variable(name) {
                    return Ok(value);
                }

                if let Some(builtin) = crate::evaluator::builtins::builtins(name) {
                    return Ok(builtin);
                }

                if let Some(builtin) = crate::evaluator::builtins::builtin_aliases(name) {
                    return Ok(builtin);
                }

                if let Some(_system) = &self.system_functions {}

                Err(RuntimeErr {
                    message: format!("Identifier can not be found: {}", name),
                    source: expression.source,
                })
            }
            ExpressionKind::Integer(value) => {
                Ok(Rc::new(Object::Integer(value.replace('_', "").parse::<i64>().unwrap())))
            }
            ExpressionKind::Decimal(value) => Ok(Rc::new(Object::Decimal(
                value.replace('_', "").parse::<OrderedFloat<f64>>().unwrap(),
            ))),
            ExpressionKind::String(value) => Ok(Rc::new(Object::String(value.to_owned()))),
            ExpressionKind::Boolean(value) => Ok(Rc::new(Object::Boolean(*value))),
            ExpressionKind::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(condition, consequence, alternative),
            ExpressionKind::Function { parameters, body } => Ok(Rc::new(Object::Function(Function::Closure {
                parameters: parameters.clone(),
                body: *body.clone(),
                environment: Rc::clone(&self.enviornment()),
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
                })
            }
            ExpressionKind::List(list) => Ok(Rc::new(Object::List(Vector::from(self.eval_expressions(list)?)))),
            ExpressionKind::Set(set) => {
                let mut elements = HashSet::default();
                for element in self.eval_expressions(set)? {
                    if !element.is_hashable() {
                        return Err(RuntimeErr {
                            message: format!("Unable to include a {} within an Set", element.name()),
                            source: expression.source,
                        });
                    }
                    elements.insert(element);
                }
                Ok(Rc::new(Object::Set(elements)))
            }
            ExpressionKind::Hash(map) => {
                let mut elements = HashMap::default();
                for (key, value) in map {
                    let evaluated_key = self.eval_expression(key)?;
                    if !evaluated_key.is_hashable() {
                        return Err(RuntimeErr {
                            message: format!("Unable to use a {} as a Hash key", evaluated_key.name()),
                            source: key.source,
                        });
                    }
                    elements.insert(evaluated_key, self.eval_expression(value)?);
                }
                Ok(Rc::new(Object::Hash(elements)))
            }
            ExpressionKind::Index { left, index } => crate::evaluator::index::lookup(self, left, index),
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
                    }),
                }
            }
            ExpressionKind::UnboundedRange { from } => match &*self.eval_expression(from)? {
                Object::Integer(from) => Ok(Rc::new(Object::LazySequence(LazySequence::unbounded_range(*from)))),
                from => Err(RuntimeErr {
                    message: format!("Expected Integer unbounded range, found: {}..", from.name()),
                    source: expression.source,
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
                }),
            },
            ExpressionKind::Nil => Ok(Rc::clone(&self.nil)),
            ExpressionKind::Placeholder => Ok(Rc::clone(&self.placeholder)),
            ExpressionKind::Spread(_) => Err(RuntimeErr {
                message: "Unable to spread within this context".to_owned(),
                source: expression.source,
            }),
            _ => Err(RuntimeErr {
                message: format!("Unimplemented expression: {}", expression),
                source: expression.source,
            }),
        }
    }

    fn eval_let_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .enviornment()
                    .borrow_mut()
                    .declare_variable(id, Rc::clone(&evaluated_value), false)
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                    }),
                }
            }
            ExpressionKind::IdentifierListPattern(pattern) => {
                self.destructure_list_pattern(pattern, evaluated_value, false, name.source)
            }
            _ => Err(RuntimeErr {
                message: format!("Unexpected Let identifier, found: {}", name.kind),
                source: name.source,
            }),
        }
    }

    fn eval_mutable_let_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .enviornment()
                    .borrow_mut()
                    .declare_variable(id, Rc::clone(&evaluated_value), true)
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                    }),
                }
            }
            ExpressionKind::IdentifierListPattern(pattern) => {
                self.destructure_list_pattern(pattern, evaluated_value, true, name.source)
            }
            _ => Err(RuntimeErr {
                message: format!("Unexpected Let identifier, found: {}", name.kind),
                source: name.source,
            }),
        }
    }

    fn eval_assign_expression(&mut self, name: &Expression, value: &Expression) -> Evaluation {
        let evaluated_value = self.eval_expression(value)?;

        match &name.kind {
            ExpressionKind::Identifier(id) => {
                match self
                    .enviornment()
                    .borrow_mut()
                    .assign_variable(id, Rc::clone(&evaluated_value))
                {
                    Ok(_) => Ok(Rc::clone(&evaluated_value)),
                    Err(EnvironmentErr { message }) => Err(RuntimeErr {
                        message,
                        source: name.source,
                    }),
                }
            }
            _ => Ok(Rc::clone(&self.nil)),
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
            Ok(Rc::clone(&self.nil))
        }
    }

    fn eval_expressions(&mut self, expressions: &[Expression]) -> Result<Vec<Rc<Object>>, RuntimeErr> {
        let mut results = Vec::with_capacity(expressions.len());

        for expression in expressions {
            if let ExpressionKind::Spread(value) = &expression.kind {
                if let Object::List(list) = &*self.eval_expression(value)? {
                    for element in list {
                        results.push(Rc::clone(element));
                    }
                    continue;
                }

                return Err(RuntimeErr {
                    message: format!("Expected a List to spread, found: {}", expression),
                    source: expression.source,
                });
            }

            results.push(self.eval_expression(expression)?);
        }

        Ok(results)
    }

    fn destructure_list_pattern(
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
                })
            }
        };

        for (position, pattern) in pattern.iter().enumerate() {
            match &pattern.kind {
                ExpressionKind::Identifier(name) => {
                    match self.enviornment().borrow_mut().declare_variable(
                        name,
                        Rc::clone(list.iter().nth(position).unwrap_or(&Rc::new(Object::Nil))),
                        is_mutable,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: pattern.source,
                            })
                        }
                    }
                }
                ExpressionKind::RestIdentifier(name) => {
                    match self.enviornment().borrow_mut().declare_variable(
                        name,
                        Rc::new(Object::List(list.clone().into_iter().skip(position).collect())),
                        is_mutable,
                    ) {
                        Ok(_) => {}
                        Err(EnvironmentErr { message }) => {
                            return Err(RuntimeErr {
                                message,
                                source: pattern.source,
                            })
                        }
                    }
                    break;
                }
                ExpressionKind::Placeholder => {
                    continue;
                }
                ExpressionKind::IdentifierListPattern(next_pattern) => {
                    self.destructure_list_pattern(
                        next_pattern,
                        Rc::clone(
                            list.iter()
                                .nth(position)
                                .unwrap_or(&Rc::new(Object::List(Vector::new()))),
                        ),
                        is_mutable,
                        pattern.source,
                    )?;
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unexpected List destructing pattern, found: {}", pattern.kind),
                        source: pattern.source,
                    })
                }
            }
        }

        Ok(subject)
    }
}
