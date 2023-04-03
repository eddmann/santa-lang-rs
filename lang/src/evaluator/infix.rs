use crate::evaluator::function::Function;
use crate::evaluator::object::Object;
use crate::evaluator::Evaluation;
use crate::evaluator::Evaluator;
use crate::evaluator::RuntimeErr;
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Infix, Statement, StatementKind};
use std::cell::RefCell;
use std::rc::Rc;

#[inline]
pub fn apply_infix_plus(
    evaluator: &mut Evaluator,
    left: &Rc<Object>,
    right: &Rc<Object>,
    source: Location,
) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Rc::new(Object::Integer(a + b))),
        (Object::Integer(a), Object::Decimal(b)) => Ok(Rc::new(Object::Integer(a + (*b as i64)))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(a + b))),
        (Object::Decimal(a), Object::Integer(b)) => Ok(Rc::new(Object::Decimal(a + (*b as f64)))),
        (Object::String(a), Object::String(b)) => Ok(Rc::new(Object::String(format!("{}{}", a, b)))),
        (Object::List(a), Object::List(b)) => {
            let mut list = a.clone();
            list.append(b.clone());
            Ok(Rc::new(Object::List(list)))
        }
        (Object::List(a), Object::Set(b)) => {
            let mut list = a.clone();
            for element in b {
                list.push_back(Rc::clone(element));
            }
            Ok(Rc::new(Object::List(list)))
        }
        (Object::List(a), Object::LazySequence(b)) => {
            let mut list = a.clone();
            for element in b.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                list.push_back(Rc::clone(&element));
            }
            Ok(Rc::new(Object::List(list)))
        }
        (Object::Set(a), Object::Set(b)) => Ok(Rc::new(Object::Set(a.clone().union(b.clone())))),
        (Object::Set(a), Object::List(b)) => {
            let mut set = a.clone();
            for element in b {
                set.insert(Rc::clone(element));
            }
            Ok(Rc::new(Object::Set(set)))
        }
        (Object::Set(a), Object::LazySequence(b)) => {
            let mut set = a.clone();
            for element in b.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                set.insert(Rc::clone(&element));
            }
            Ok(Rc::new(Object::Set(set)))
        }
        (Object::Hash(a), Object::Hash(b)) => {
            let mut map = a.clone();
            for (k, v) in b.clone() {
                map.insert(k, v);
            }
            Ok(Rc::new(Object::Hash(map)))
        }
        _ => Err(RuntimeErr {
            message: format!("Unsupported operator type: {} + {}", left.name(), right.name()),
            source,
        }),
    }
}

#[inline]
pub fn apply_infix_minus(
    evaluator: &mut Evaluator,
    left: &Rc<Object>,
    right: &Rc<Object>,
    source: Location,
) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(Rc::new(Object::Integer(a - b))),
        (Object::Integer(a), Object::Decimal(b)) => Ok(Rc::new(Object::Integer(a - (*b as i64)))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(a - b))),
        (Object::Decimal(a), Object::Integer(b)) => Ok(Rc::new(Object::Decimal(a - (*b as f64)))),
        (Object::List(a), Object::List(b)) => {
            let mut list = a.clone();
            list.retain(|element| !b.contains(element));
            Ok(Rc::new(Object::List(list)))
        }
        (Object::List(a), Object::Set(b)) => {
            let mut list = a.clone();
            list.retain(|element| !b.contains(element));
            Ok(Rc::new(Object::List(list)))
        }
        (Object::List(a), Object::LazySequence(b)) => {
            let mut list = a.clone();
            let resolved_b = b
                .resolve_iter(Rc::new(RefCell::new(evaluator)), source)
                .collect::<Vec<_>>();
            list.retain(|element| !resolved_b.contains(element));
            Ok(Rc::new(Object::List(list)))
        }
        (Object::Set(a), Object::Set(b)) => {
            let mut set = a.clone();
            set.retain(|element| !b.contains(element));
            Ok(Rc::new(Object::Set(set)))
        }
        (Object::Set(a), Object::List(b)) => {
            let mut set = a.clone();
            set.retain(|element| !b.contains(element));
            Ok(Rc::new(Object::Set(set)))
        }
        (Object::Set(a), Object::LazySequence(b)) => {
            let mut set = a.clone();
            let resolved_b = b
                .resolve_iter(Rc::new(RefCell::new(evaluator)), source)
                .collect::<Vec<_>>();
            set.retain(|element| !resolved_b.contains(element));
            Ok(Rc::new(Object::Set(set)))
        }
        _ => Err(RuntimeErr {
            message: format!("Unsupported operator type: {} - {}", left.name(), right.name()),
            source,
        }),
    }
}

#[inline]
pub fn apply(
    evaluator: &mut Evaluator,
    left: &Expression,
    operator: &Infix,
    right: &Expression,
    source: Location,
) -> Evaluation {
    match (&left.kind, &right.kind) {
        (ExpressionKind::Placeholder, ExpressionKind::Placeholder) => {
            return Ok(Rc::new(Object::Function(Function::Closure {
                parameters: vec![
                    Expression {
                        kind: ExpressionKind::Identifier("a".to_owned()),
                        source: left.source,
                    },
                    Expression {
                        kind: ExpressionKind::Identifier("b".to_owned()),
                        source: right.source,
                    },
                ],
                body: Statement {
                    kind: StatementKind::Expression(Box::new(Expression {
                        kind: ExpressionKind::Infix {
                            left: Box::new(Expression {
                                kind: ExpressionKind::Identifier("a".to_owned()),
                                source: left.source,
                            }),
                            operator: operator.clone(),
                            right: Box::new(Expression {
                                kind: ExpressionKind::Identifier("b".to_owned()),
                                source: right.source,
                            }),
                        },
                        source,
                    })),
                    source,
                },
                environment: evaluator.enviornment(),
            })))
        }
        (ExpressionKind::Placeholder, _) => {
            return Ok(Rc::new(Object::Function(Function::Closure {
                parameters: vec![Expression {
                    kind: ExpressionKind::Identifier("a".to_owned()),
                    source: left.source,
                }],
                body: Statement {
                    kind: StatementKind::Expression(Box::new(Expression {
                        kind: ExpressionKind::Infix {
                            left: Box::new(Expression {
                                kind: ExpressionKind::Identifier("a".to_owned()),
                                source: left.source,
                            }),
                            operator: operator.clone(),
                            right: Box::new(right.clone()),
                        },
                        source,
                    })),
                    source,
                },
                environment: evaluator.enviornment(),
            })))
        }
        (_, ExpressionKind::Placeholder) => {
            return Ok(Rc::new(Object::Function(Function::Closure {
                parameters: vec![Expression {
                    kind: ExpressionKind::Identifier("b".to_owned()),
                    source: left.source,
                }],
                body: Statement {
                    kind: StatementKind::Expression(Box::new(Expression {
                        kind: ExpressionKind::Infix {
                            left: Box::new(left.clone()),
                            operator: operator.clone(),
                            right: Box::new(Expression {
                                kind: ExpressionKind::Identifier("b".to_owned()),
                                source: left.source,
                            }),
                        },
                        source,
                    })),
                    source,
                },
                environment: evaluator.enviornment(),
            })))
        }
        _ => {}
    }

    let evaluated_left = evaluator.eval_expression(left)?;
    let evaluated_right = evaluator.eval_expression(right)?;

    match operator {
        Infix::Call(identifier) => {
            if let Object::Function(function) = &*evaluator.eval_expression(identifier)? {
                return function.apply(evaluator, vec![evaluated_left, evaluated_right], source);
            }

            Err(RuntimeErr {
                message: format!("Not a function: {}", identifier),
                source,
            })
        }
        Infix::Plus => apply_infix_plus(evaluator, &evaluated_left, &evaluated_right, source),
        Infix::Minus => apply_infix_minus(evaluator, &evaluated_left, &evaluated_right, source),
        _ => todo!(),
    }
}
