use crate::evaluator::object::{new_string, Object};
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::lexer::Location;
use im_rc::Vector;
use std::cell::RefCell;
use std::rc::Rc;

#[inline]
pub fn lookup(evaluator: &mut Evaluator, left: Rc<Object>, index: Rc<Object>, source: Location) -> Evaluation {
    match (&*left, &*index) {
        (Object::List(list), Object::Integer(index)) => {
            Ok(list_lookup(list, *index).unwrap_or_else(|| Rc::new(Object::Nil)))
        }
        (Object::List(list), Object::LazySequence(sequence)) => {
            let is_unbounded_negative_range = sequence.is_unbounded_negative_range();

            let mut result = Vector::new();
            for step in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if let Object::Integer(index) = &*step {
                    if *index == 0 && is_unbounded_negative_range {
                        break;
                    }
                    match list_lookup(list, *index) {
                        Some(object) => result.push_back((*object).clone()),
                        None => break,
                    }
                } else {
                    return Err(RuntimeErr {
                        message: format!("Expected Integer List index, found: {}", step.name()),
                        source,
                        trace: evaluator.get_trace(),
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
        (Object::Dictionary(map), index) => {
            if let Some(value) = map.get(index) {
                Ok(Rc::new(value.clone()))
            } else {
                Ok(Rc::new(Object::Nil))
            }
        }
        (Object::String(string), Object::Integer(index)) => {
            if let Some(character) = string_lookup(string, *index) {
                Ok(new_string(character.to_string()))
            } else {
                Ok(Rc::new(Object::Nil))
            }
        }
        (Object::LazySequence(sequence), Object::Integer(index)) => {
            let mut iterator = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source);
            if let Some(element) = iterator.nth(*index as usize) {
                Ok(Rc::clone(&element))
            } else {
                Ok(Rc::new(Object::Nil))
            }
        }
        (Object::String(string), Object::LazySequence(sequence)) => {
            let is_unbounded_negative_range = sequence.is_unbounded_negative_range();

            let mut result = String::new();
            for step in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
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
                        source,
                        trace: evaluator.get_trace(),
                    });
                }
            }

            Ok(new_string(result))
        }
        (_, _) => Err(RuntimeErr {
            message: format!(
                "Unable to perform index operation, found: {}[{}]",
                left.name(),
                index.name()
            ),
            source,
            trace: evaluator.get_trace(),
        }),
    }
}

fn list_lookup(list: &Vector<Object>, index: i64) -> Option<Rc<Object>> {
    if index > 0 && index as usize >= list.len() {
        return None;
    }

    if index < 0 && list.len() as i64 + index < 0 {
        return None;
    }

    if index < 0 {
        Some(Rc::new(list[(list.len() as i64 + index) as usize].clone()))
    } else {
        Some(Rc::new(list[index as usize].clone()))
    }
}

fn string_lookup(string: &str, index: i64) -> Option<char> {
    if index < 0 {
        string.chars().nth((string.len() as i64 + index) as usize)
    } else {
        string.chars().nth(index as usize)
    }
}
