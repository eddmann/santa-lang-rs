use crate::evaluator::object::Object;
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::parser::ast::Expression;
use im_rc::Vector;
use std::cell::RefCell;
use std::rc::Rc;

#[inline]
pub fn lookup(evaluator: &mut Evaluator, left: &Expression, index: &Expression) -> Evaluation {
    match (&*evaluator.eval_expression(left)?, &*evaluator.eval_expression(index)?) {
        (Object::List(list), Object::Integer(index)) => Ok(list_lookup(list, *index).unwrap_or(Rc::new(Object::Nil))),
        (Object::List(list), Object::LazySequence(sequence)) => {
            let is_unbounded_negative_range = sequence.is_unbounded_negative_range();

            let mut result = Vector::new();
            for step in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), index.source) {
                if let Object::Integer(index) = &*step {
                    if *index == 0 && is_unbounded_negative_range {
                        break;
                    }
                    match list_lookup(list, *index) {
                        Some(object) => result.push_back(object),
                        None => break,
                    }
                } else {
                    return Err(RuntimeErr {
                        message: format!("Expected Integer List index, found: {}", step.name()),
                        source: index.source,
                    });
                }
            }

            Ok(Rc::new(Object::List(result)))
        }
        (Object::Set(set), index) => Ok(Rc::new(if set.contains(index) {
            index.clone()
        } else {
            Object::Nil
        })),
        (Object::Hash(map), index) => {
            if let Some(value) = map.get(index) {
                Ok(Rc::clone(value))
            } else {
                Ok(Rc::new(Object::Nil))
            }
        }
        (Object::String(string), Object::Integer(index)) => {
            if let Some(character) = string_lookup(string, *index) {
                Ok(Rc::new(Object::String(character.to_string())))
            } else {
                Ok(Rc::new(Object::Nil))
            }
        }
        (Object::String(string), Object::LazySequence(sequence)) => {
            let is_unbounded_negative_range = sequence.is_unbounded_negative_range();

            let mut result = String::new();
            for step in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), index.source) {
                if let Object::Integer(index) = &*step {
                    if *index == 0 && is_unbounded_negative_range {
                        break;
                    }
                    match string_lookup(string, *index) {
                        Some(character) => result.push(character),
                        None => break,
                    }
                } else {
                    return Err(RuntimeErr {
                        message: format!("Expected Integer String index, found: {}", step.name()),
                        source: index.source,
                    });
                }
            }

            Ok(Rc::new(Object::String(result)))
        }
        (evaluated_left, evaluated_index) => Err(RuntimeErr {
            message: format!(
                "Unable to perform index operation, found: {}[{}]",
                evaluated_left.name(),
                evaluated_index.name()
            ),
            source: left.source,
        }),
    }
}

fn list_lookup(list: &Vector<Rc<Object>>, index: i64) -> Option<Rc<Object>> {
    if index.unsigned_abs() as usize >= list.len() {
        return None;
    }

    if index < 0 {
        Some(Rc::clone(&list[(list.len() as i64 + index) as usize]))
    } else {
        Some(Rc::clone(&list[index as usize]))
    }
}

fn string_lookup(string: &str, index: i64) -> Option<char> {
    if index < 0 {
        string.chars().nth((string.len() as i64 + index) as usize)
    } else {
        string.chars().nth(index as usize)
    }
}