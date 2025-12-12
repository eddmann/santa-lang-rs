use crate::evaluator::function::Function;
use crate::evaluator::object::Object;
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::lexer::Location;
use crate::parser::ast::{Expression, ExpressionKind, Infix, Statement, StatementKind};
use std::rc::Rc;

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
                    preceded_by_blank_line: false,
                    trailing_comment: None,
                },
                environment: evaluator.environment(),
            })));
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
                    preceded_by_blank_line: false,
                    trailing_comment: None,
                },
                environment: evaluator.environment(),
            })));
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
                    preceded_by_blank_line: false,
                    trailing_comment: None,
                },
                environment: evaluator.environment(),
            })));
        }
        _ => {}
    }

    match operator {
        Infix::Or => {
            return Ok(Rc::new(Object::Boolean(
                evaluator.eval_expression(left)?.is_truthy() || evaluator.eval_expression(right)?.is_truthy(),
            )));
        }
        Infix::And => {
            return Ok(Rc::new(Object::Boolean(
                evaluator.eval_expression(left)?.is_truthy() && evaluator.eval_expression(right)?.is_truthy(),
            )));
        }
        Infix::Call(function) => {
            let evaluated_function = evaluator.eval_expression(function)?;

            if let Object::Function(func) = &*evaluated_function {
                let evaluated_left = evaluator.eval_expression(left)?;
                let evaluated_right = evaluator.eval_expression(right)?;
                return func.apply(evaluator, vec![evaluated_left, evaluated_right], function.source);
            }

            return Err(RuntimeErr {
                message: format!("Expected a Function, found: {}", evaluated_function.name()),
                source: function.source,
                trace: evaluator.get_trace(),
            });
        }
        _ => {}
    }

    let evaluated_left = evaluator.eval_expression(left)?;
    let evaluated_right = evaluator.eval_expression(right)?;

    match operator {
        Infix::Plus => {
            crate::evaluator::builtins::operators::plus(evaluator, &evaluated_left, &evaluated_right, source)
        }
        Infix::Minus => {
            crate::evaluator::builtins::operators::minus(evaluator, &evaluated_left, &evaluated_right, source)
        }
        Infix::Asterisk => crate::evaluator::builtins::operators::asterisk(&evaluated_left, &evaluated_right, source),
        Infix::Slash => crate::evaluator::builtins::operators::slash(&evaluated_left, &evaluated_right, source),
        Infix::Modulo => crate::evaluator::builtins::operators::modulo(&evaluated_left, &evaluated_right, source),
        Infix::Equal => crate::evaluator::builtins::operators::equal(&evaluated_left, &evaluated_right),
        Infix::NotEqual => crate::evaluator::builtins::operators::not_equal(&evaluated_left, &evaluated_right),
        Infix::LessThan => crate::evaluator::builtins::operators::less_than(&evaluated_left, &evaluated_right),
        Infix::LessThanEqual => {
            crate::evaluator::builtins::operators::less_than_equal(&evaluated_left, &evaluated_right)
        }
        Infix::GreaterThan => crate::evaluator::builtins::operators::greater_than(&evaluated_left, &evaluated_right),
        Infix::GreaterThanEqual => {
            crate::evaluator::builtins::operators::greater_than_equal(&evaluated_left, &evaluated_right)
        }
        _ => unreachable!(),
    }
}
