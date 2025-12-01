use crate::evaluator::object::Object;
use crate::evaluator::{Environment, EnvironmentErr, Evaluation, Evaluator, Frame, RuntimeErr};
use crate::parser::ast::MatchCase;
use crate::parser::ast::{Expression, ExpressionKind};
use im_rc::Vector;
use std::rc::Rc;

type PatternMatch = bool;

#[inline]
pub fn matcher(evaluator: &mut Evaluator, subject: &Expression, cases: &[MatchCase]) -> Evaluation {
    let evaluated_subject = evaluator.eval_expression(subject)?;

    for case in cases {
        match &case.pattern.kind {
            ExpressionKind::Identifier(name) => {
                evaluator.push_frame(Frame::Block {
                    _source: case.pattern.source,
                    environment: Environment::from(evaluator.environment()),
                });
                match evaluator
                    .environment()
                    .borrow_mut()
                    .declare_variable(name, Rc::clone(&evaluated_subject), false)
                {
                    Ok(_) => {}
                    Err(EnvironmentErr { message }) => {
                        return Err(RuntimeErr {
                            message,
                            source: case.pattern.source,
                            trace: evaluator.get_trace(),
                        });
                    }
                };
                if let Some(guard) = &case.guard {
                    if !evaluator.eval_expression(guard)?.is_truthy() {
                        evaluator.pop_frame();
                        continue;
                    }
                }
                let result = evaluator.eval_statement(&case.consequence)?;
                evaluator.pop_frame();
                return Ok(result);
            }
            ExpressionKind::Placeholder => {
                if let Some(guard) = &case.guard {
                    if !evaluator.eval_expression(guard)?.is_truthy() {
                        continue;
                    }
                }
                return evaluator.eval_statement(&case.consequence);
            }
            ExpressionKind::ListMatchPattern(pattern) => {
                evaluator.push_frame(Frame::Block {
                    _source: case.pattern.source,
                    environment: Environment::from(evaluator.environment()),
                });
                if !destructure_match_list_pattern(evaluator, pattern, Rc::clone(&evaluated_subject))? {
                    evaluator.pop_frame();
                    continue;
                }
                if let Some(guard) = &case.guard {
                    if !evaluator.eval_expression(guard)?.is_truthy() {
                        evaluator.pop_frame();
                        continue;
                    }
                }
                let result = evaluator.eval_statement(&case.consequence)?;
                evaluator.pop_frame();
                return Ok(result);
            }
            ExpressionKind::InclusiveRange { from, to } => {
                if let (ExpressionKind::Integer(from), ExpressionKind::Integer(to), Object::Integer(index)) =
                    (&from.kind, &to.kind, &*evaluated_subject)
                {
                    if (from.replace('_', "").parse::<i64>().unwrap()..=to.replace('_', "").parse::<i64>().unwrap())
                        .contains(index)
                    {
                        if let Some(guard) = &case.guard {
                            if !evaluator.eval_expression(guard)?.is_truthy() {
                                continue;
                            }
                        }
                        return evaluator.eval_statement(&case.consequence);
                    }
                }
            }
            ExpressionKind::ExclusiveRange { from, until } => {
                if let (ExpressionKind::Integer(from), ExpressionKind::Integer(until), Object::Integer(index)) =
                    (&from.kind, &until.kind, &*evaluated_subject)
                {
                    if (from.replace('_', "").parse::<i64>().unwrap()..until.replace('_', "").parse::<i64>().unwrap())
                        .contains(index)
                    {
                        if let Some(guard) = &case.guard {
                            if !evaluator.eval_expression(guard)?.is_truthy() {
                                continue;
                            }
                        }
                        return evaluator.eval_statement(&case.consequence);
                    }
                }
            }
            ExpressionKind::UnboundedRange { from } => {
                if let (ExpressionKind::Integer(from), Object::Integer(index)) = (&from.kind, &*evaluated_subject) {
                    if (from.replace('_', "").parse::<i64>().unwrap()..).contains(index) {
                        if let Some(guard) = &case.guard {
                            if !evaluator.eval_expression(guard)?.is_truthy() {
                                continue;
                            }
                        }
                        return evaluator.eval_statement(&case.consequence);
                    }
                }
            }
            _ => {
                if evaluator.eval_expression(&case.pattern)? != evaluated_subject {
                    continue;
                }
                if let Some(guard) = &case.guard {
                    if !evaluator.eval_expression(guard)?.is_truthy() {
                        continue;
                    }
                }
                return evaluator.eval_statement(&case.consequence);
            }
        }
    }

    Ok(Rc::new(Object::Nil))
}

fn destructure_match_list_pattern(
    evaluator: &mut Evaluator,
    pattern: &[Expression],
    subject: Rc<Object>,
) -> Result<PatternMatch, RuntimeErr> {
    let list = match &*subject {
        Object::List(list) => list,
        _ => return Ok(false),
    };

    if pattern.is_empty() != list.is_empty() {
        return Ok(false);
    }

    let mut position = 0;
    for sub_pattern in pattern.iter() {
        if position >= list.len() {
            return Ok(false);
        }

        match &sub_pattern.kind {
            ExpressionKind::Placeholder => {}
            ExpressionKind::Identifier(name) => {
                match evaluator
                    .environment()
                    .borrow_mut()
                    .declare_variable(name, Rc::clone(&list[position]), false)
                {
                    Ok(_) => {}
                    Err(EnvironmentErr { message }) => {
                        return Err(RuntimeErr {
                            message,
                            source: sub_pattern.source,
                            trace: evaluator.get_trace(),
                        });
                    }
                }
            }
            ExpressionKind::ListMatchPattern(pattern) => {
                if !destructure_match_list_pattern(evaluator, pattern, Rc::clone(&list[position]))? {
                    return Ok(false);
                }
            }
            ExpressionKind::RestIdentifier(name) => {
                let rest = list.clone().into_iter().skip(position).collect::<Vector<Rc<Object>>>();

                match evaluator
                    .environment()
                    .borrow_mut()
                    .declare_variable(name, Rc::new(Object::List(rest)), false)
                {
                    Ok(_) => {}
                    Err(EnvironmentErr { message }) => {
                        return Err(RuntimeErr {
                            message,
                            source: sub_pattern.source,
                            trace: evaluator.get_trace(),
                        });
                    }
                }

                return Ok(true);
            }
            ExpressionKind::InclusiveRange { from, to } => {
                if let (ExpressionKind::Integer(from), ExpressionKind::Integer(to), Object::Integer(index)) =
                    (&from.kind, &to.kind, &*list[position])
                {
                    if !(from.replace('_', "").parse::<i64>().unwrap()..=to.replace('_', "").parse::<i64>().unwrap())
                        .contains(index)
                    {
                        return Ok(false);
                    }
                }
            }
            ExpressionKind::ExclusiveRange { from, until } => {
                if let (ExpressionKind::Integer(from), ExpressionKind::Integer(until), Object::Integer(index)) =
                    (&from.kind, &until.kind, &*list[position])
                {
                    if !(from.replace('_', "").parse::<i64>().unwrap()..until.replace('_', "").parse::<i64>().unwrap())
                        .contains(index)
                    {
                        return Ok(false);
                    }
                }
            }
            ExpressionKind::UnboundedRange { from } => {
                if let (ExpressionKind::Integer(from), Object::Integer(index)) = (&from.kind, &*list[position]) {
                    if !(from.replace('_', "").parse::<i64>().unwrap()..).contains(index) {
                        return Ok(false);
                    }
                }
            }
            _ => {
                if list[position] != evaluator.eval_expression(sub_pattern)? {
                    return Ok(false);
                }
            }
        }

        position += 1;
    }

    if position < list.len() {
        return Ok(false);
    }

    Ok(true)
}
