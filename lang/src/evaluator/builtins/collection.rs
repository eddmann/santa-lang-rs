use crate::evaluator::lazy_sequence::{LazyFn, LazySequence};
use crate::evaluator::object::{new_integer, new_string, Object};
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::lexer::Location;
use im_rc::{HashMap, HashSet, Vector};
use smallvec::smallvec;
use std::cell::RefCell;
use std::rc::Rc;

builtin! {
    push(value, collection) match {
        (_, Object::List(list)) => {
            let mut next_list = list.clone();
            next_list.push_back((**value).clone());
            Ok(Rc::new(Object::List(next_list)))
        }
        (_, Object::Set(set)) => {
            let mut next_set = set.clone();
            next_set.insert((**value).clone());
            Ok(Rc::new(Object::Set(next_set)))
        }
    }
}

builtin! {
    size(collection) [evaluator, source] match {
        Object::List(list) => {
            Ok(new_integer(list.len() as i64))
        }
        Object::Set(set) => {
            Ok(new_integer(set.len() as i64))
        }
        Object::Dictionary(map) => {
            Ok(new_integer(map.len() as i64))
        }
        Object::String(string) => {
            Ok(new_integer(string.len() as i64))
        }
        Object::LazySequence(sequence) => {
            Ok(new_integer(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).count() as i64))
        }
    }
}

builtin! {
    map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vector::new();
            for element in list {
                elements.push_back((*mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (Object::Function(mapper), Object::Set(set)) => {
            let mut elements = HashSet::default();
            for element in set {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if !mapped.is_hashable() {
                    return Err(RuntimeErr {
                        message: format!("Unable to include a {} within an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
                elements.insert((*mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?).clone());
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        (Object::Function(mapper), Object::Dictionary(map)) => {
            let mut elements = HashMap::default();
            for (key, value) in map {
                elements.insert(key.clone(), (*mapper.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?).clone());
            }
            Ok(Rc::new(Object::Dictionary(elements)))
        }
        (Object::Function(mapper), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Map(mapper.clone())))))
        }
        (Object::Function(mapper), Object::String(string)) => {
            let mut elements = Vector::new();
            for character in string.chars() {
                elements.push_back((*mapper.apply(evaluator, smallvec![new_string(character.to_string())], source)?).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    filter(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            let mut elements = Vector::new();
            for element in list {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    elements.push_back(element.clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (Object::Function(predicate), Object::Set(list)) => {
            let mut elements = HashSet::default();
            for element in list {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    elements.insert(element.clone());
                }
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        (Object::Function(predicate), Object::Dictionary(map)) => {
            let mut elements = HashMap::default();
            for (key, value) in map {
                if predicate.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?.is_truthy() {
                    elements.insert(key.clone(), value.clone());
                }
            }
            Ok(Rc::new(Object::Dictionary(elements)))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Filter(predicate.clone())))))
        }
        (Object::Function(predicate), Object::String(string)) => {
            let mut elements = Vector::new();
            for character in string.chars() {
                let object = new_string(character.to_string());
                if predicate.apply(evaluator, smallvec![Rc::clone(&object)], source)?.is_truthy() {
                    elements.push_back((*object).clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    fold(initial, folder, collection) [evaluator, source] match {
        (_, Object::Function(folder), Object::List(list)) => {
            let mut accumulator = Rc::clone(initial);
            for element in list {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::Set(set)) => {
            let mut accumulator = Rc::clone(initial);
            for element in set {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::Dictionary(map)) => {
            let mut accumulator = Rc::clone(initial);
            for (key, value) in map {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            let mut accumulator = Rc::clone(initial);
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                accumulator = folder.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&accumulator), Rc::clone(&element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::String(string)) => {
            let mut accumulator = Rc::clone(initial);
            for character in string.chars() {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), new_string(character.to_string())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
    }
}

builtin! {
    each(side_effect, collection) [evaluator, source] match {
        (Object::Function(side_effect), Object::List(list)) => {
            for element in list {
                let result = side_effect.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if let Object::Break(_) = &*result {
                    break;
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::Set(set)) => {
            for element in set {
                let result = side_effect.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if let Object::Break(_) = &*result {
                    break;
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::Dictionary(map)) => {
            for (key, value) in map {
                let result = side_effect.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if let Object::Break(_) = &*result {
                    break;
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                let result = side_effect.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?;
                if let Object::Break(_) = &*result {
                    break;
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::String(string)) => {
            for character in string.chars() {
                let result = side_effect.apply(evaluator, smallvec![new_string(character.to_string())], source)?;
                if let Object::Break(_) = &*result {
                    break;
                }
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    reduce(reducer, collection) [evaluator, source] match {
        (Object::Function(reducer), Object::List(list)) => {
            let mut elements = list.iter();
            let mut accumulator = match elements.next() {
                Some(element) => Rc::new(element.clone()),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty List".to_owned(),
                    source,
                    trace: evaluator.get_trace()
                })
            };
            for element in elements {
                accumulator = reducer.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (Object::Function(reducer), Object::Set(set)) => {
            let mut elements = set.iter();
            let mut accumulator = match elements.next() {
                Some(element) => Rc::new(element.clone()),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty Set".to_owned(),
                    source,
                    trace: evaluator.get_trace()
                })
            };
            for element in elements {
                accumulator = reducer.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (Object::Function(reducer), Object::Dictionary(map)) => {
            let mut elements = map.iter();
            let mut accumulator = match elements.next() {
                Some((key, value)) => Rc::new(value.clone()),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty Dictionary".to_owned(),
                    source,
                    trace: evaluator.get_trace()
                })
            };
            for (key, value) in elements {
                accumulator = reducer.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (Object::Function(reducer), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            let mut elements = sequence.resolve_iter(Rc::clone(&shared_evaluator), source);
            let mut accumulator = match elements.next() {
                Some(element) => Rc::clone(&element),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty LazySequence".to_owned(),
                    source,
                    trace: shared_evaluator.borrow().get_trace()
                })
            };
            for element in elements {
                accumulator = reducer.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&accumulator), Rc::clone(&element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (Object::Function(reducer), Object::String(string)) => {
            let mut characters = string.chars();
            let mut accumulator = match characters.next() {
                Some(character) => new_string(character.to_string()),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty String".to_owned(),
                    source,
                    trace: evaluator.get_trace()
                })
            };
            for character in characters {
                accumulator = reducer.apply(evaluator, smallvec![Rc::clone(&accumulator), new_string(character.to_string())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
    }
}

builtin! {
    flat_map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vector::new();
            for element in list {
                if let Object::List(other_elements) = &*mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)? {
                    elements.append(other_elements.clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    find(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            for element in list {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(element.clone()))
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(predicate), Object::Set(set)) => {
            for element in set {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(element.clone()))
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(predicate), Object::Dictionary(map)) => {
            for (key, value) in map {
                if predicate.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(value.clone()));
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                if predicate.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?.is_truthy() {
                    return Ok(Rc::clone(&element))
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(predicate), Object::String(string)) => {
            for character in string.chars() {
                let object = new_string(character.to_string());
                if predicate.apply(evaluator, smallvec![Rc::clone(&object)], source)?.is_truthy() {
                    return Ok(Rc::clone(&object))
                }
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    count(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            let mut count = 0;
            for element in list {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    count += 1;
                }
            }
            Ok(new_integer(count))
        }
        (Object::Function(predicate), Object::Set(set)) => {
            let mut count = 0;
            for element in set {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    count += 1;
                }
            }
            Ok(new_integer(count))
        }
        (Object::Function(predicate), Object::Dictionary(map)) => {
            let mut count = 0;
            for (key, value) in map {
                if predicate.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?.is_truthy() {
                    count += 1;
                }
            }
            Ok(new_integer(count))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            let mut count = 0;
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                if predicate.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?.is_truthy() {
                    count += 1;
                }
            }
            Ok(new_integer(count))
        }
        (Object::Function(predicate), Object::String(string)) => {
            let mut count = 0;
            for character in string.chars() {
                let object = new_string(character.to_string());
                if predicate.apply(evaluator, smallvec![Rc::clone(&object)], source)?.is_truthy() {
                    count += 1;
                }
            }
            Ok(new_integer(count))
        }
    }
}

builtin! {
    sum(collection) [evaluator, source] match {
        Object::List(list) => {
            let mut sum = 0;
            for element in list {
                if let Object::Integer(value) = element {
                    sum += value;
                }
            }
            Ok(new_integer(sum))
        }
        Object::Set(set) => {
            let mut sum = 0;
            for element in set {
                if let Object::Integer(value) = element {
                    sum += value;
                }
            }
            Ok(new_integer(sum))
        }
        Object::Dictionary(map) => {
            let mut sum = 0;
            for (key, value) in map {
                if let Object::Integer(value) = value {
                    sum += value;
                }
            }
            Ok(new_integer(sum))
        }
        Object::LazySequence(sequence) => {
            let mut sum = 0;
            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if let Object::Integer(value) = &*element {
                    sum += value;
                }
            }
            Ok(new_integer(sum))
        }
    }
}

builtin! {
    max(..values) [evaluator, source] {
        let list = if let Object::List(list) = &**values {
            if list.len() == 1 {
                Rc::new(list[0].clone())
            } else {
                Rc::clone(values)
            }
        } else {
            return Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };

        match &*list {
            Object::List(list) => {
                if let Some(max) = list.iter().max() {
                    return Ok(Rc::new(max.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::Set(set) => {
                if let Some(max) = set.iter().max() {
                    return Ok(Rc::new(max.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::Dictionary(map) => {
                if let Some(max) = map.values().max() {
                    return Ok(Rc::new(max.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::LazySequence(sequence) => {
                if let Some(max) = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).max() {
                    return Ok(Rc::clone(&max));
                }

                Ok(Rc::new(Object::Nil))
            }
            _ => Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
    }
}

builtin! {
    min(..values) [evaluator, source] {
        let list = if let Object::List(list) = &**values {
            if list.len() == 1 {
                Rc::new(list[0].clone())
            } else {
                Rc::clone(values)
            }
        } else {
            return Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };

        match &*list {
            Object::List(list) => {
                if let Some(min) = list.iter().min() {
                    return Ok(Rc::new(min.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::Set(set) => {
                if let Some(min) = set.iter().min() {
                    return Ok(Rc::new(min.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::Dictionary(map) => {
                if let Some(min) = map.values().min() {
                    return Ok(Rc::new(min.clone()));
                }

                Ok(Rc::new(Object::Nil))
            }
            Object::LazySequence(sequence) => {
                if let Some(min) = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).min() {
                    return Ok(Rc::clone(&min));
                }

                Ok(Rc::new(Object::Nil))
            }
            _ => Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
    }
}

builtin! {
    skip(total, collection) [evaluator, source] match {
        (Object::Integer(total), Object::List(list)) => {
            Ok(Rc::new(Object::List(list.iter().skip(*total as usize).cloned().collect())))
        }
        (Object::Integer(total), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Skip(*total as usize)))))
        }
    }
}

builtin! {
    take(total, collection) [evaluator, source] match {
        (Object::Integer(total), Object::List(list)) => {
            Ok(Rc::new(Object::List(list.iter().take(*total as usize).cloned().collect())))
        }
        (Object::Integer(total), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::List(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).take(*total as usize).map(|obj| (*obj).clone()).collect::<Vector<Object>>())))
        }
    }
}

builtin! {
    list(value) [evaluator, source] match {
        Object::List(list) => {
            Ok(Rc::new(Object::List(list.clone())))
        }
        Object::Set(set) => {
            Ok(Rc::new(Object::List(set.iter().cloned().collect::<Vector<Object>>())))
        }
        Object::Dictionary(map) => {
            let to_pairs = |(key, value)| Object::List(vec![key, value].into());
            Ok(Rc::new(Object::List(map.iter().map(|(k, v)| to_pairs((k.clone(), v.clone()))).collect::<Vector<Object>>())))
        }
        Object::LazySequence(sequence) => {
            Ok(Rc::new(Object::List(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).map(|obj| (*obj).clone()).collect::<Vector<Object>>())))
        }
        Object::String(string) => {
            Ok(Rc::new(Object::List(string.chars().map(|character| Object::String(character.to_string())).collect::<Vector<Object>>())))
        }
    }
}

builtin! {
    set(value) [evaluator, source] match {
        Object::List(list) => {
            let mut elements = HashSet::default();
            for element in list {
                if !element.is_hashable() {
                    return Err(RuntimeErr {
                        message: format!("Unable to include a {} within an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
                elements.insert(element.clone());
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        Object::Set(set) => {
            Ok(Rc::new(Object::Set(set.clone())))
        }
        Object::LazySequence(sequence) => {
            let mut elements = HashSet::default();
            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if !element.is_hashable() {
                    return Err(RuntimeErr {
                        message: format!("Unable to include a {} within an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
                elements.insert((*element).clone());
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        Object::String(string) => {
            Ok(Rc::new(Object::Set(string.chars().map(|character| Object::String(character.to_string())).collect::<HashSet<_, _>>())))
        }
    }
}

builtin! {
    dict(value) [evaluator, source] match {
        Object::List(list) => {
            let mut elements = HashMap::default();

            for element in list.iter() {
                if let Object::List(pair) = element {
                    if pair.len() == 2 {
                        if !pair[0].is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to use a {} as a Dictionary key", pair[0].name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert(pair[0].clone(), pair[1].clone());
                        continue;
                    }
                }

                return Err(RuntimeErr {
                    message: format!(
                        "Expected a [key, value] List pair, found: {}",
                        element.name()
                    ),
                    source,
                    trace: evaluator.get_trace()
                })
            }

            Ok(Rc::new(Object::Dictionary(elements)))
        }
        Object::Dictionary(map) => {
            Ok(Rc::new(Object::Dictionary(map.clone())))
        }
        Object::LazySequence(sequence) => {
            let mut elements = HashMap::default();

            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if let Object::List(pair) = &*element {
                    if pair.len() == 2 {
                        if !pair[0].is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to use a {} as a Dictionary key", pair[0].name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert(pair[0].clone(), pair[1].clone());
                        continue;
                    }
                }

                return Err(RuntimeErr {
                    message: format!(
                        "Expected a [key, value] List pair, found: {}",
                        element.name()
                    ),
                    source,
                    trace: evaluator.get_trace()
                })
            }

            Ok(Rc::new(Object::Dictionary(elements)))
        }
    }
}

builtin! {
    repeat(value) {
        Ok(Rc::new(Object::LazySequence(LazySequence::repeat(Rc::clone(value)))))
    }
}

builtin! {
    cycle(list) match {
        Object::List(list) => {
            Ok(Rc::new(Object::LazySequence(LazySequence::cycle(list.clone()))))
        }
        Object::String(string) => {
            let characters = string.chars().map(|character| Object::String(character.to_string())).collect::<Vector<Object>>();
            Ok(Rc::new(Object::LazySequence(LazySequence::cycle(characters))))
        }
    }
}

builtin! {
    iterate(generator, initial) match {
        (Object::Function(generator), _) => {
            Ok(Rc::new(Object::LazySequence(LazySequence::iterate(generator.clone(), Rc::clone(initial)))))
        }
    }
}

#[inline]
fn lazy_zipper(sequences: &Vector<Object>) -> Option<Rc<Object>> {
    let mut zipped = Vec::with_capacity(sequences.len());
    for sequence in sequences {
        match sequence {
            Object::LazySequence(sequence) => zipped.push(sequence.clone()),
            _ => return None,
        }
    }

    let sequence = zipped[0]
        .clone()
        .with_fn(LazyFn::Zip(zipped.into_iter().skip(1).collect::<Vec<_>>()));

    Some(Rc::new(Object::LazySequence(sequence)))
}

#[inline]
fn eager_zipper(sequences: Vector<Object>, evaluator: &mut Evaluator, source: Location) -> Evaluation {
    let shared_evaluator = Rc::new(RefCell::new(evaluator));

    let mut iterators: Vec<Box<dyn Iterator<Item = Rc<Object>>>> = Vec::with_capacity(sequences.len());
    for sequence in &sequences {
        match sequence {
            Object::List(list) => iterators.push(Box::new(list.iter().map(|obj| Rc::new(obj.clone())))),
            Object::String(string) => iterators.push(Box::new(
                string
                    .chars()
                    .map(|character| new_string(character.to_string())),
            )),
            Object::LazySequence(sequence) => {
                iterators.push(Box::new(sequence.resolve_iter(Rc::clone(&shared_evaluator), source)));
            }
            _ => {
                return Err(RuntimeErr {
                    message: format!(
                        "Expected a List, String or LazySequence to zip, found: {}",
                        sequence.name()
                    ),
                    source,
                    trace: shared_evaluator.borrow().get_trace(),
                })
            }
        }
    }

    let mut zipped = Vector::new();
    'zipper: loop {
        let mut entry = Vector::new();
        for iterator in iterators.iter_mut() {
            match iterator.next() {
                Some(element) => entry.push_back((*element).clone()),
                None => break 'zipper,
            }
        }
        zipped.push_back(Object::List(entry));
    }

    Ok(Rc::new(Object::List(zipped)))
}

builtin! {
    zip(collection, ..collections) [evaluator, source] match {
        (_, Object::List(collections)) => {
            let mut collections = collections.clone();
            collections.push_front((**collection).clone());

            if let Some(zipped) = lazy_zipper(&collections) {
                return Ok(zipped);
            }

            eager_zipper(collections, evaluator, source)
        }
    }
}

builtin! {
    keys(dictionary) [evaluator, source] match {
        Object::Dictionary(map) => {
            Ok(Rc::new(Object::List(map.iter().map(|(key, _)| key.clone()).collect::<Vector<_>>())))
        }
    }
}

builtin! {
    values(dictionary) [evaluator, source] match {
        Object::Dictionary(map) => {
            Ok(Rc::new(Object::List(map.iter().map(|(_, value)| value.clone()).collect::<Vector<_>>())))
        }
    }
}

builtin! {
    first(collection) [evaluator, source] match {
        Object::List(list) => {
            if let Some(first) = list.front() {
                return Ok(Rc::new(first.clone()));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::Set(set) => {
            if let Some(first) = set.iter().next() {
                return Ok(Rc::new(first.clone()));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::LazySequence(sequence) => {
            let mut iterator = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source);
            if let Some(first) = iterator.next() {
                return Ok(Rc::clone(&first));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::String(string) => {
            if let Some(first) = string.chars().next() {
                return Ok(new_string(first.to_string()));
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    second(collection) [evaluator, source] match {
        Object::List(list) => {
            if let Some(second) = list.get(1) {
                return Ok(Rc::new(second.clone()));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::Set(set) => {
            let mut iterator = set.iter();
            iterator.next();
            if let Some(second) = iterator.next() {
                return Ok(Rc::new(second.clone()));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::LazySequence(sequence) => {
            let mut iterator = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source);
            iterator.next();
            if let Some(second) = iterator.next() {
                return Ok(Rc::clone(&second));
            }
            Ok(Rc::new(Object::Nil))
        }
        Object::String(string) => {
            let mut iterator = string.chars();
            iterator.next();
            if let Some(second) = iterator.next() {
                return Ok(new_string(second.to_string()));
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    rest(collection) [evaluator, source] match {
        Object::List(list) => {
            let mut rest = list.clone();
            rest.pop_front();
            Ok(Rc::new(Object::List(rest)))
        }
        Object::Set(set) => {
            Ok(Rc::new(Object::Set(set.iter().skip(1).cloned().collect())))
        }
        Object::LazySequence(sequence) => {
            let mut iterator = sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source);
            iterator.next();
            Ok(Rc::new(Object::LazySequence(iterator.to_sequence())))
        }
        Object::String(string) => {
            Ok(new_string(string.chars().skip(1).collect()))
        }
    }
}

builtin! {
    get(index, collection) [evaluator, source] {
        crate::evaluator::index::lookup(evaluator, Rc::clone(collection), Rc::clone(index), source)
    }
}

builtin! {
    includes(collection, value) [evaluator, source] match {
        (Object::List(list), _) => {
            Ok(Rc::new(Object::Boolean(list.contains(value))))
        }
        (Object::Set(set), _) => {
            Ok(Rc::new(Object::Boolean(set.contains(value))))
        }
        (Object::Dictionary(map), _) => {
            Ok(Rc::new(Object::Boolean(map.contains_key(value))))
        }
        (Object::LazySequence(sequence), _) => {
            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if element == *value {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
        (Object::String(string), _) => {
            if let Object::String(subject) = &**value {
                return Ok(Rc::new(Object::Boolean(string.contains(subject))));
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
    }
}

builtin! {
    excludes(collection, value) [evaluator, source] match {
        (Object::List(list), _) => {
            Ok(Rc::new(Object::Boolean(!list.contains(value))))
        }
        (Object::Set(set), _) => {
            Ok(Rc::new(Object::Boolean(!set.contains(value))))
        }
        (Object::Dictionary(map), _) => {
            Ok(Rc::new(Object::Boolean(!map.contains_key(value))))
        }
        (Object::LazySequence(sequence), _) => {
            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if element == *value {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
        (Object::String(string), _) => {
            if let Object::String(subject) = &**value {
                return Ok(Rc::new(Object::Boolean(!string.contains(subject))));
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
    }
}

builtin! {
    any(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            for element in list.iter() {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
        (Object::Function(predicate), Object::Set(set)) => {
            for element in set.iter() {
                if predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
        (Object::Function(predicate), Object::Dictionary(map)) => {
            for (key, value) in map.iter() {
                if predicate.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                if predicate.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
        (Object::Function(predicate), Object::String(string)) => {
            for character in string.chars() {
                if predicate.apply(evaluator, smallvec![new_string(character.to_string())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(true)))
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        }
    }
}

builtin! {
    all(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            for element in list.iter() {
                if !predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
        (Object::Function(predicate), Object::Set(set)) => {
            for element in set.iter() {
                if !predicate.apply(evaluator, smallvec![Rc::new(element.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
        (Object::Function(predicate), Object::Dictionary(map)) => {
            for (key, value) in map.iter() {
                if !predicate.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                if !predicate.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
        (Object::Function(predicate), Object::String(string)) => {
            for character in string.chars() {
                if !predicate.apply(evaluator, smallvec![new_string(character.to_string())], source)?.is_truthy() {
                    return Ok(Rc::new(Object::Boolean(false)))
                }
            }
            Ok(Rc::new(Object::Boolean(true)))
        }
    }
}

builtin! {
    sort(comparator, collection) [evaluator, source] match {
        (Object::Function(comparator), Object::List(list)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));

            let mut sorted_list = list.clone();
            sorted_list.sort_by(|a, b| {
                // Note: Comparator errors during sort will panic. This is a limitation of sort_by.
                // TODO: Consider implementing a fallible sorting mechanism.
                match &*comparator.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::new(a.clone()), Rc::new(b.clone())], source)
                    .expect("Comparator function should not error during sort") {
                    Object::Integer(comparison) => comparison.cmp(&0),
                    comparison => if comparison.is_truthy() {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }
            });

            Ok(Rc::new(Object::List(sorted_list)))
        }
    }
}

builtin! {
    union(..values) [evaluator, source] {
        let list = if let Object::List(list) = &**values {
            if list.len() == 1 {
                if let Object::List(list) = &list[0] {
                    list.clone()
                } else {
                    return Err(RuntimeErr {
                        message: "".to_owned(),
                        source,
                        trace: evaluator.get_trace()
                    })
                }
            } else {
                list.clone()
            }
        } else {
            return Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };

        let mut elements = list.iter();
        let mut accumulator = match elements.next() {
            Some(element) => {
                match element {
                    Object::List(list) => {
                        let mut elements = HashSet::default();
                        for element in list {
                            if !element.is_hashable() {
                                return Err(RuntimeErr {
                                    message: format!("Unable to include a {} within an Set", element.name()),
                                    source,
                                    trace: evaluator.get_trace()
                                });
                            }
                            elements.insert(element.clone());
                        }
                        elements
                    }
                    Object::Set(set) => {
                        set.clone()
                    }
                    Object::LazySequence(sequence) => {
                        let mut elements = HashSet::default();
                        for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                            if !element.is_hashable() {
                                return Err(RuntimeErr {
                                    message: format!("Unable to include a {} within an Set", element.name()),
                                    source,
                                    trace: evaluator.get_trace()
                                });
                            }
                            elements.insert((*element).clone());
                        }
                        elements
                    }
                    Object::String(string) => {
                        string.chars().map(|character| Object::String(character.to_string())).collect()
                    }
                    _ => {
                        return Err(RuntimeErr {
                            message: format!("Unable to convert a {} into an Set", element.name()),
                            source,
                            trace: evaluator.get_trace()
                        });
                    }
                }
            }
            None => return Err(RuntimeErr {
                message: "Unable to reduce an empty List".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };
        for element in elements {
            let element = match element {
                Object::List(list) => {
                    let mut elements = HashSet::default();
                    for element in list {
                        if !element.is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to include a {} within an Set", element.name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert(element.clone());
                    }
                    elements
                }
                Object::Set(set) => {
                    set.clone()
                }
                Object::LazySequence(sequence) => {
                    let mut elements = HashSet::default();
                    for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                        if !element.is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to include a {} within an Set", element.name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert((*element).clone());
                    }
                    elements
                }
                Object::String(string) => {
                    string.chars().map(|character| Object::String(character.to_string())).collect()
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unable to convert a {} into an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
            };
            accumulator = accumulator.union(element);
        }
        Ok(Rc::new(Object::Set(accumulator)))
    }
}

builtin! {
    intersection(..values) [evaluator, source] {
        let list = if let Object::List(list) = &**values {
            if list.len() == 1 {
                if let Object::List(list) = &list[0] {
                    list.clone()
                } else {
                    return Err(RuntimeErr {
                        message: "".to_owned(),
                        source,
                        trace: evaluator.get_trace()
                    })
                }
            } else {
                list.clone()
            }
        } else {
            return Err(RuntimeErr {
                message: "".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };

        let mut elements = list.iter();
        let mut accumulator = match elements.next() {
            Some(element) => {
                match element {
                    Object::List(list) => {
                        let mut elements = HashSet::default();
                        for element in list {
                            if !element.is_hashable() {
                                return Err(RuntimeErr {
                                    message: format!("Unable to include a {} within an Set", element.name()),
                                    source,
                                    trace: evaluator.get_trace()
                                });
                            }
                            elements.insert(element.clone());
                        }
                        elements
                    }
                    Object::Set(set) => {
                        set.clone()
                    }
                    Object::LazySequence(sequence) => {
                        let mut elements = HashSet::default();
                        for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                            if !element.is_hashable() {
                                return Err(RuntimeErr {
                                    message: format!("Unable to include a {} within an Set", element.name()),
                                    source,
                                    trace: evaluator.get_trace()
                                });
                            }
                            elements.insert((*element).clone());
                        }
                        elements
                    }
                    Object::String(string) => {
                        string.chars().map(|character| Object::String(character.to_string())).collect()
                    }
                    _ => {
                        return Err(RuntimeErr {
                            message: format!("Unable to convert a {} into an Set", element.name()),
                            source,
                            trace: evaluator.get_trace()
                        });
                    }
                }
            }
            None => return Err(RuntimeErr {
                message: "Unable to reduce an empty List".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        };
        for element in elements {
            let element = match element {
                Object::List(list) => {
                    let mut elements = HashSet::default();
                    for element in list {
                        if !element.is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to include a {} within an Set", element.name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert(element.clone());
                    }
                    elements
                }
                Object::Set(set) => {
                    set.clone()
                }
                Object::LazySequence(sequence) => {
                    let mut elements = HashSet::default();
                    for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                        if !element.is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to include a {} within an Set", element.name()),
                                source,
                                trace: evaluator.get_trace()
                            });
                        }
                        elements.insert((*element).clone());
                    }
                    elements
                }
                Object::String(string) => {
                    string.chars().map(|character| Object::String(character.to_string())).collect()
                }
                _ => {
                    return Err(RuntimeErr {
                        message: format!("Unable to convert a {} into an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
            };
            accumulator = accumulator.intersection(element);
        }
        Ok(Rc::new(Object::Set(accumulator)))
    }
}

builtin! {
    scan(initial, folder, collection) [evaluator, source] match {
        (_, Object::Function(folder), Object::List(list)) => {
            let mut elements = Vector::new();
            elements.push_back((**initial).clone());
            let mut previous = Rc::clone(initial);
            for element in list {
                previous = folder.apply(evaluator, smallvec![Rc::clone(&previous), Rc::new(element.clone())], source)?;
                elements.push_back((*previous).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (_, Object::Function(folder), Object::Set(set)) => {
            let mut elements = Vector::new();
            elements.push_back((**initial).clone());
            let mut previous = Rc::clone(initial);
            for element in set {
                previous = folder.apply(evaluator, smallvec![Rc::clone(&previous), Rc::new(element.clone())], source)?;
                elements.push_back((*previous).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (_, Object::Function(folder), Object::Dictionary(map)) => {
            let mut elements = Vector::new();
            elements.push_back((**initial).clone());
            let mut previous = Rc::clone(initial);
            for (key, value) in map {
                previous = folder.apply(evaluator, smallvec![Rc::clone(&previous), Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                elements.push_back((*previous).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (_, Object::Function(folder), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            let mut elements = Vector::new();
            elements.push_back((**initial).clone());
            let mut previous = Rc::clone(initial);
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                previous = folder.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&previous), element], source)?;
                elements.push_back((*previous).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (_, Object::Function(folder), Object::String(string)) => {
            let mut elements = Vector::new();
            elements.push_back((**initial).clone());
            let mut previous = Rc::clone(initial);
            for character in string.chars() {
                previous = folder.apply(evaluator, smallvec![Rc::clone(&previous), new_string(character.to_string())], source)?;
                elements.push_back((*previous).clone());
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    reverse(collection) [evaluator, source] match {
        Object::List(list) => {
            Ok(Rc::new(Object::List(list.iter().rev().cloned().collect())))
        }
        Object::LazySequence(sequence) => {
            Ok(Rc::new(Object::List(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).map(|obj| (*obj).clone()).collect::<Vector<Object>>().into_iter().rev().collect())))
        }
        Object::String(string) => {
            Ok(new_string(string.chars().rev().collect()))
        }
    }
}

builtin! {
    filter_map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vector::new();
            for element in list {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if mapped.is_truthy() {
                    elements.push_back((*mapped).clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (Object::Function(mapper), Object::Set(set)) => {
            let mut elements = HashSet::default();
            for element in set {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if !mapped.is_hashable() {
                    return Err(RuntimeErr {
                        message: format!("Unable to include a {} within an Set", element.name()),
                        source,
                        trace: evaluator.get_trace()
                    });
                }
                if mapped.is_truthy() {
                    elements.insert((*mapped).clone());
                }
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        (Object::Function(mapper), Object::Dictionary(map)) => {
            let mut elements = HashMap::default();
            for (key, value) in map {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if mapped.is_truthy() {
                    elements.insert(key.clone(), (*mapped).clone());
                }
            }
            Ok(Rc::new(Object::Dictionary(elements)))
        }
        (Object::Function(mapper), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::FilterMap(mapper.clone())))))
        }
        (Object::Function(mapper), Object::String(string)) => {
            let mut elements = Vector::new();
            for character in string.chars() {
                let mapped = mapper.apply(evaluator, smallvec![new_string(character.to_string())], source)?;
                if mapped.is_truthy() {
                    elements.push_back((*mapped).clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    find_map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            for element in list {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if mapped.is_truthy() {
                    return Ok(mapped);
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(mapper), Object::Set(set)) => {
            for element in set {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(element.clone())], source)?;
                if mapped.is_truthy() {
                    return Ok(mapped);
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(mapper), Object::Dictionary(map)) => {
            for (key, value) in map {
                let mapped = mapper.apply(evaluator, smallvec![Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if mapped.is_truthy() {
                    return Ok(mapped);
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(mapper), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                let mapped = mapper.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&element)], source)?;
                if mapped.is_truthy() {
                    return Ok(mapped);
                }
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(mapper), Object::String(string)) => {
            for character in string.chars() {
                let mapped = mapper.apply(evaluator, smallvec![new_string(character.to_string())], source)?;
                if mapped.is_truthy() {
                    return Ok(mapped);
                }
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    assoc(key, value, collection) [evaluator, source] match {
        (Object::Integer(index), _, Object::List(list)) => {
            let mut associated = list.clone();
            if *index as usize >= list.len()  {
                for _ in 0..=*index as usize-list.len() {
                    associated.push_back(Object::Nil);
                }
            }
            Ok(Rc::new(Object::List(associated.update(*index as usize, (**value).clone()))))
        }
        (_, _, Object::Dictionary(map)) => {
            Ok(Rc::new(Object::Dictionary(map.update((**key).clone(), (**value).clone()))))
        }
    }
}

builtin! {
    update(key, updater, collection) [evaluator, source] match {
        (Object::Integer(index), Object::Function(updater), Object::List(list)) => {
            let mut updated = list.clone();
            let index = *index as usize;
            if index >= list.len()  {
                for _ in 0..=index-list.len() {
                    updated.push_back(Object::Nil);
                }
            }
            let previous = match updated.get(index) {
                Some(value) => Rc::new(value.clone()),
                None => Rc::new(Object::Nil),
            };
            Ok(Rc::new(Object::List(updated.update(index, (*updater.apply(evaluator, smallvec![Rc::clone(&previous)], source)?).clone()))))
        }
        (_, Object::Function(updater), Object::Dictionary(map)) => {
            let previous = match map.get(key) {
                Some(value) => Rc::new(value.clone()),
                None => Rc::new(Object::Nil),
            };
            Ok(Rc::new(Object::Dictionary(map.update((**key).clone(), (*updater.apply(evaluator, smallvec![Rc::clone(&previous), Rc::new((**key).clone())], source)?).clone()))))
        }
    }
}

builtin! {
    update_d(key, default, updater, collection) [evaluator, source] match {
        (Object::Integer(index), _, Object::Function(updater), Object::List(list)) => {
            let mut updated = list.clone();
            let index = *index as usize;
            let previous = match updated.get(index) {
                Some(value) => Rc::new(value.clone()),
                None => Rc::clone(default),
            };
            if index >= list.len()  {
                for _ in 0..=index-list.len() {
                    updated.push_back(Object::Nil);
                }
            }
            Ok(Rc::new(Object::List(updated.update(index, (*updater.apply(evaluator, smallvec![Rc::clone(&previous)], source)?).clone()))))
        }
        (_, _,Object::Function(updater), Object::Dictionary(map)) => {
            let previous = match map.get(key) {
                Some(value) => Rc::new(value.clone()),
                None => Rc::clone(default),
            };
            Ok(Rc::new(Object::Dictionary(map.update((**key).clone(), (*updater.apply(evaluator, smallvec![Rc::clone(&previous), Rc::new((**key).clone())], source)?).clone()))))
        }
    }
}

builtin! {
    fold_s(initial, folder, collection) [evaluator, source] match {
        (_, Object::Function(folder), Object::List(list)) => {
            let mut accumulator = Rc::clone(initial);
            for element in list {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::new((**value).clone()));
                }
            }
            if let Object::List(accumulated) = &*accumulator {
                if let Some(value) = accumulated.get(0) {
                    return Ok(Rc::new(value.clone()));
                }
            }
            Err(RuntimeErr {
                message: "Expected a List with an accumulated value at 0 index".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
        (_, Object::Function(folder), Object::Set(set)) => {
            let mut accumulator = Rc::clone(initial);
            for element in set {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(element.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::new((**value).clone()));
                }
            }
            if let Object::List(accumulated) = &*accumulator {
                if let Some(value) = accumulated.get(0) {
                    return Ok(Rc::new(value.clone()));
                }
            }
            Err(RuntimeErr {
                message: "Expected a List with an accumulated value at 0 index".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
        (_, Object::Function(folder), Object::Dictionary(map)) => {
            let mut accumulator = Rc::clone(initial);
            for (key, value) in map {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), Rc::new(value.clone()), Rc::new(key.clone())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::new((**value).clone()));
                }
            }
            if let Object::List(accumulated) = &*accumulator {
                if let Some(value) = accumulated.get(0) {
                    return Ok(Rc::new(value.clone()));
                }
            }
            Err(RuntimeErr {
                message: "Expected a List with an accumulated value at 0 index".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
        (_, Object::Function(folder), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            let mut accumulator = Rc::clone(initial);
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                accumulator = folder.apply(&mut shared_evaluator.borrow_mut(), smallvec![Rc::clone(&accumulator), element], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::new((**value).clone()));
                }
            }
            if let Object::List(accumulated) = &*accumulator {
                if let Some(value) = accumulated.get(0) {
                    return Ok(Rc::new(value.clone()));
                }
            }
            let trace = shared_evaluator.borrow().get_trace();
            Err(RuntimeErr {
                message: "Expected a List with an accumulated value at 0 index".to_owned(),
                source,
                trace
            })
        }
        (_, Object::Function(folder), Object::String(string)) => {
            let mut accumulator = Rc::clone(initial);
            for character in string.chars() {
                accumulator = folder.apply(evaluator, smallvec![Rc::clone(&accumulator), new_string(character.to_string())], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::new((**value).clone()));
                }
            }
            if let Object::List(accumulated) = &*accumulator {
                if let Some(value) = accumulated.get(0) {
                    return Ok(Rc::new(value.clone()));
                }
            }
            Err(RuntimeErr {
                message: "Expected a List with an accumulated value at 0 index".to_owned(),
                source,
                trace: evaluator.get_trace()
            })
        }
    }
}

builtin! {
    rotate(steps, collection) [evaluator, source] match {
        (Object::Integer(steps), Object::List(list)) => {
            if list.len() < 2 {
                return Ok(Rc::clone(collection));
            }
            let mut rotated = list.clone();
            let backwards = *steps < 0;
            for _ in 0..steps.abs() {
                if backwards {
                    let front = rotated.pop_front()
                        .expect("List should not be empty during rotation");
                    rotated.push_back(front);
                } else {
                    let back = rotated.pop_back()
                        .expect("List should not be empty during rotation");
                    rotated.push_front(back);
                }
            }
            Ok(Rc::new(Object::List(rotated)))
        }
    }
}

builtin! {
    chunk(size, collection) [evaluator, source] match {
        (Object::Integer(size), Object::List(list)) => {
            let mut chunked: Vector<Object> = Vector::new();
            let mut remaining_elements = list.iter().cloned().peekable();
            while remaining_elements.peek().is_some() {
                chunked.push_back(Object::List(remaining_elements.by_ref().take(*size as usize).collect()));
            }
            Ok(Rc::new(Object::List(chunked)))
        }
        (Object::Integer(size), Object::String(string)) => {
            let mut chunked: Vector<Object> = Vector::new();
            let mut remaining_elements = string.chars().map(|character| Object::String(character.to_string())).peekable();
            while remaining_elements.peek().is_some() {
                chunked.push_back(Object::List(remaining_elements.by_ref().take(*size as usize).collect()));
            }
            Ok(Rc::new(Object::List(chunked)))
        }
    }
}

builtin! {
    combinations(size, collection) [evaluator, source] match {
        (Object::Integer(size), Object::List(list)) => {
            Ok(Rc::new(Object::LazySequence(LazySequence::combinations(*size as u32, list.clone()))))
        }
    }
}
