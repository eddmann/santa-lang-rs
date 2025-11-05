use crate::evaluator::object::Object;
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::lexer::Location;
use std::cell::RefCell;
use std::rc::Rc;

#[inline]
pub fn plus(evaluator: &mut Evaluator, left: &Rc<Object>, right: &Rc<Object>, source: Location) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(evaluator.pool().integer(a + b)),
        (Object::Integer(a), Object::Decimal(b)) => Ok(evaluator.pool().integer(a + (f64::from(*b) as i64))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(*a + *b))),
        (Object::Decimal(a), Object::Integer(b)) => Ok(Rc::new(Object::Decimal(a + (*b as f64)))),
        (Object::String(a), Object::String(b)) => Ok(Rc::new(Object::String(format!("{}{}", a, b)))),
        (Object::String(a), Object::Integer(b)) => Ok(Rc::new(Object::String(format!("{}{}", a, b)))),
        (Object::String(a), Object::Decimal(b)) => Ok(Rc::new(Object::String(format!("{}{}", a, b)))),
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
        (Object::Dictionary(a), Object::Dictionary(b)) => {
            let mut map = a.clone();
            for (k, v) in b.clone() {
                map.insert(k, v);
            }
            Ok(Rc::new(Object::Dictionary(map)))
        }
        _ => Err(RuntimeErr {
            message: format!("Unsupported operation: {} + {}", left.name(), right.name()),
            source,
            trace: vec![],
        }),
    }
}

builtin! {
    plus(a, b) [evaulator, source] {
        plus(evaulator, a, b, source)
    }
}

#[inline]
pub fn minus(evaluator: &mut Evaluator, left: &Rc<Object>, right: &Rc<Object>, source: Location) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(evaluator.pool().integer(a - b)),
        (Object::Integer(a), Object::Decimal(b)) => Ok(evaluator.pool().integer(a - (f64::from(*b) as i64))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(*a - *b))),
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
            message: format!("Unsupported operation: {} - {}", left.name(), right.name()),
            source,
            trace: vec![],
        }),
    }
}

builtin! {
    minus(a, b) [evaulator, source] {
        minus(evaulator, a, b, source)
    }
}

#[inline]
pub fn asterisk(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>, source: Location) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(evaluator.pool().integer(a * b)),
        (Object::Integer(a), Object::Decimal(b)) => Ok(evaluator.pool().integer(a * (f64::from(*b) as i64))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(*a * *b))),
        (Object::Decimal(a), Object::Integer(b)) => Ok(Rc::new(Object::Decimal(a * (*b as f64)))),
        (Object::String(a), Object::Integer(b)) => Ok(Rc::new(Object::String(a.repeat(*b as usize)))),
        (Object::List(a), Object::Integer(b)) => {
            let mut list = a.clone();
            for _ in 1..*b {
                list.append(a.clone());
            }
            Ok(Rc::new(Object::List(list)))
        }
        _ => Err(RuntimeErr {
            message: format!("Unsupported operation: {} * {}", left.name(), right.name()),
            source,
            trace: vec![],
        }),
    }
}

builtin! {
    asterisk(a, b) [evaulator, source] {
        asterisk(evaulator, a, b, source)
    }
}

#[inline]
pub fn slash(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>, source: Location) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => Ok(evaluator.pool().integer(a / b)),
        (Object::Integer(a), Object::Decimal(b)) => Ok(evaluator.pool().integer(a / (f64::from(*b) as i64))),
        (Object::Decimal(a), Object::Decimal(b)) => Ok(Rc::new(Object::Decimal(*a / *b))),
        (Object::Decimal(a), Object::Integer(b)) => Ok(Rc::new(Object::Decimal(a / (*b as f64)))),
        _ => Err(RuntimeErr {
            message: format!("Unsupported operation: {} / {}", left.name(), right.name()),
            source,
            trace: vec![],
        }),
    }
}

builtin! {
    slash(a, b) [evaulator, source] {
        slash(evaulator, a, b, source)
    }
}

#[inline]
pub fn modulo(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>, source: Location) -> Evaluation {
    match (&**left, &**right) {
        (Object::Integer(a), Object::Integer(b)) => {
            // http://python-history.blogspot.com/2010/08/why-pythons-integer-division-floors.html
            let remainder = a % b;
            let result = if remainder == 0 || a.signum() == b.signum() {
                remainder
            } else {
                remainder + b
            };
            Ok(evaluator.pool().integer(result))
        }
        _ => Err(RuntimeErr {
            message: format!("Unsupported operation: {} % {}", left.name(), right.name()),
            source,
            trace: vec![],
        }),
    }
}

builtin! {
    modulo(a, b) [evaulator, source] {
        modulo(evaulator, a, b, source)
    }
}

#[inline]
pub fn equal(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left == right))
}

builtin! {
    equal(a, b) [evaulator, _source] {
        equal(evaulator, a, b)
    }
}

#[inline]
pub fn not_equal(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left != right))
}

builtin! {
    not_equal(a, b) [evaulator, _source] {
        not_equal(evaulator, a, b)
    }
}

#[inline]
pub fn less_than(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left < right))
}

builtin! {
    less_than(a, b) [evaulator, _source] {
        less_than(evaulator, a, b)
    }
}

#[inline]
pub fn less_than_equal(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left <= right))
}

builtin! {
    less_than_equal(a, b) [evaulator, _source] {
        less_than_equal(evaulator, a, b)
    }
}

#[inline]
pub fn greater_than(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left > right))
}

builtin! {
    greater_than(a, b) [evaulator, _source] {
        greater_than(evaulator, a, b)
    }
}

#[inline]
pub fn greater_than_equal(evaluator: &Evaluator, left: &Rc<Object>, right: &Rc<Object>) -> Evaluation {
    Ok(evaluator.pool().boolean(left >= right))
}

builtin! {
    greater_than_equal(a, b) [evaulator, _source] {
        greater_than_equal(evaulator, a, b)
    }
}

builtin! {
    and(a, b) [evaulator, _source] {
        Ok(evaulator.pool().boolean(a.is_truthy() && b.is_truthy()))
    }
}

builtin! {
    or(a, b) [evaulator, _source] {
        Ok(evaulator.pool().boolean(a.is_truthy() || b.is_truthy()))
    }
}
