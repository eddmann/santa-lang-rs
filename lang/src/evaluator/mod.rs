mod builtins;
mod environment;
mod function;
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
use crate::parser::ast::{Expression, ExpressionKind, Program, Statement, StatementKind};
use im_rc::{HashMap, HashSet, Vector};
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

        for (_index, statement) in block.iter().enumerate() {
            if let StatementKind::Comment(_) = statement.kind {
                continue;
            }

            result = self.eval_statement(statement)?;

            if let Object::Return(_) = *result {
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

                if let Some(system) = &self.system_functions {}

                Err(RuntimeErr {
                    message: format!("Identifier '{}' can not be found", name),
                    source: expression.source,
                })
            }
            ExpressionKind::Integer(value) => {
                Ok(Rc::new(Object::Integer(value.replace('_', "").parse::<i64>().unwrap())))
            }
            ExpressionKind::Decimal(value) => {
                Ok(Rc::new(Object::Decimal(value.replace('_', "").parse::<f64>().unwrap())))
            }
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
                    message: format!("Unable to call: {}", function),
                    source: function.source,
                })
            }
            ExpressionKind::List(list) => Ok(Rc::new(Object::List(Vector::from(self.eval_expressions(list)?)))),
            ExpressionKind::Set(set) => Ok(Rc::new(Object::Set(HashSet::from(self.eval_expressions(set)?)))),
            ExpressionKind::Hash(map) => {
                let mut elements = HashMap::new();
                for (key, value) in map {
                    elements.insert(self.eval_expression(key)?, self.eval_expression(value)?);
                }
                Ok(Rc::new(Object::Hash(elements)))
            }
            ExpressionKind::FunctionThread { initial, functions } => {
                let mut result = self.eval_expression(initial)?;

                for function in functions {
                    if let Object::Function(f) = &*self.eval_expression(function)? {
                        result = f.apply(self, vec![result], function.source)?;
                        continue;
                    }

                    return Err(RuntimeErr {
                        message: format!("Not a function: {}", function),
                        source: function.source,
                    });
                }

                Ok(result)
            }
            ExpressionKind::FunctionComposition(functions) => {
                let mut evaluated_functions = Vec::with_capacity(functions.len());

                for function in functions {
                    if let Object::Function(f) = &*self.eval_expression(function)? {
                        evaluated_functions.push(f.clone());
                        continue;
                    }

                    return Err(RuntimeErr {
                        message: format!("Not a function: {}", function),
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
                    _ => Err(RuntimeErr {
                        message: "Inclusive range requires Integer values".to_owned(),
                        source: expression.source,
                    }),
                }
            }
            ExpressionKind::ExclusiveRange { from, until } => {
                match (&*self.eval_expression(from)?, &*self.eval_expression(until)?) {
                    (Object::Integer(from), Object::Integer(until)) => Ok(Rc::new(Object::LazySequence(
                        LazySequence::exclusive_range(*from, *until),
                    ))),
                    _ => Err(RuntimeErr {
                        message: "Exclusive range requires Integer values".to_owned(),
                        source: expression.source,
                    }),
                }
            }
            ExpressionKind::UnboundedRange { from } => {
                if let Object::Integer(from) = &*self.eval_expression(from)? {
                    return Ok(Rc::new(Object::LazySequence(LazySequence::unbounded_range(*from))));
                }
                Err(RuntimeErr {
                    message: "Exclusive range requires Integer values".to_owned(),
                    source: expression.source,
                })
            }
            ExpressionKind::Infix { left, operator, right } => {
                crate::evaluator::infix::apply(self, left, operator, right, expression.source)
            }
            ExpressionKind::Nil => Ok(Rc::clone(&self.nil)),
            ExpressionKind::Placeholder => Ok(Rc::clone(&self.placeholder)),
            ExpressionKind::Spread(_) => Err(RuntimeErr {
                message: "Unable to spread in this context".to_owned(),
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
            _ => Ok(Rc::clone(&self.nil)),
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
            _ => Ok(Rc::clone(&self.nil)),
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
                    message: format!("Expected a list to spread: {}", expression),
                    source: expression.source,
                });
            }

            results.push(self.eval_expression(expression)?);
        }

        Ok(results)
    }
}
