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

    // Find the rest pattern position (if any)
    let rest_position = pattern
        .iter()
        .position(|p| matches!(p.kind, ExpressionKind::RestIdentifier(_)));

    match rest_position {
        None => {
            // No rest pattern - exact length match required
            if pattern.len() != list.len() {
                return Ok(false);
            }
            for (position, sub_pattern) in pattern.iter().enumerate() {
                if !match_single_pattern(evaluator, sub_pattern, Rc::clone(&list[position]))? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Some(rest_idx) => {
            let patterns_before = rest_idx;
            let patterns_after = pattern.len() - rest_idx - 1;
            let min_required = patterns_before + patterns_after;

            // List must have at least enough elements for non-rest patterns
            if list.len() < min_required {
                return Ok(false);
            }

            // Match patterns before the rest
            for i in 0..patterns_before {
                if !match_single_pattern(evaluator, &pattern[i], Rc::clone(&list[i]))? {
                    return Ok(false);
                }
            }

            // Match patterns after the rest (from the end)
            for i in 0..patterns_after {
                let pattern_idx = rest_idx + 1 + i;
                let list_idx = list.len() - patterns_after + i;
                if !match_single_pattern(evaluator, &pattern[pattern_idx], Rc::clone(&list[list_idx]))? {
                    return Ok(false);
                }
            }

            // Bind the rest pattern
            let rest_pattern = &pattern[rest_idx];
            if let ExpressionKind::RestIdentifier(name) = &rest_pattern.kind {
                let rest_start = patterns_before;
                let rest_end = list.len() - patterns_after;
                let rest: Vector<Rc<Object>> = list
                    .iter()
                    .skip(rest_start)
                    .take(rest_end - rest_start)
                    .cloned()
                    .collect();

                match evaluator
                    .environment()
                    .borrow_mut()
                    .declare_variable(name, Rc::new(Object::List(rest)), false)
                {
                    Ok(_) => {}
                    Err(EnvironmentErr { message }) => {
                        return Err(RuntimeErr {
                            message,
                            source: rest_pattern.source,
                            trace: evaluator.get_trace(),
                        });
                    }
                }
            }

            Ok(true)
        }
    }
}

fn match_single_pattern(
    evaluator: &mut Evaluator,
    sub_pattern: &Expression,
    element: Rc<Object>,
) -> Result<PatternMatch, RuntimeErr> {
    match &sub_pattern.kind {
        ExpressionKind::Placeholder => Ok(true),
        ExpressionKind::Identifier(name) => {
            match evaluator
                .environment()
                .borrow_mut()
                .declare_variable(name, element, false)
            {
                Ok(_) => Ok(true),
                Err(EnvironmentErr { message }) => Err(RuntimeErr {
                    message,
                    source: sub_pattern.source,
                    trace: evaluator.get_trace(),
                }),
            }
        }
        ExpressionKind::ListMatchPattern(pattern) => {
            destructure_match_list_pattern(evaluator, pattern, element)
        }
        ExpressionKind::InclusiveRange { from, to } => {
            if let (ExpressionKind::Integer(from), ExpressionKind::Integer(to), Object::Integer(index)) =
                (&from.kind, &to.kind, &*element)
            {
                if !(from.replace('_', "").parse::<i64>().unwrap()..=to.replace('_', "").parse::<i64>().unwrap())
                    .contains(index)
                {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ExpressionKind::ExclusiveRange { from, until } => {
            if let (ExpressionKind::Integer(from), ExpressionKind::Integer(until), Object::Integer(index)) =
                (&from.kind, &until.kind, &*element)
            {
                if !(from.replace('_', "").parse::<i64>().unwrap()..until.replace('_', "").parse::<i64>().unwrap())
                    .contains(index)
                {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ExpressionKind::UnboundedRange { from } => {
            if let (ExpressionKind::Integer(from), Object::Integer(index)) = (&from.kind, &*element) {
                if !(from.replace('_', "").parse::<i64>().unwrap()..).contains(index) {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => {
            if element != evaluator.eval_expression(sub_pattern)? {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }
}
