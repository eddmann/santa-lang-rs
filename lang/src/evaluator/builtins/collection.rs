use crate::evaluator::lazy_sequence::{LazyFn, LazySequence};
use crate::evaluator::object::Object;
use crate::evaluator::{Evaluation, Evaluator, RuntimeErr};
use crate::lexer::Location;
use im_rc::{HashMap, HashSet, Vector};
use std::cell::RefCell;
use std::rc::Rc;

builtin! {
    push(value, collection) match {
        (_, Object::List(list)) => {
            let mut next_list = list.clone();
            next_list.push_back(Rc::clone(value));
            Ok(Rc::new(Object::List(next_list)))
        }
        (_, Object::Set(set)) => {
            let mut next_set = set.clone();
            next_set.insert(Rc::clone(value));
            Ok(Rc::new(Object::Set(next_set)))
        }
    }
}

builtin! {
    map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vector::new();
            for element in list {
                elements.push_back(mapper.apply(evaluator, vec![Rc::clone(element)], source)?);
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (Object::Function(mapper), Object::Set(set)) => {
            let mut elements = HashSet::default();
            for element in set {
                elements.insert(mapper.apply(evaluator, vec![Rc::clone(element)], source)?);
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        (Object::Function(mapper), Object::Hash(map)) => {
            let mut elements = HashMap::default();
            for (key, value) in map {
                elements.insert(Rc::clone(key), mapper.apply(evaluator, vec![Rc::clone(value), Rc::clone(key)], source)?);
            }
            Ok(Rc::new(Object::Hash(elements)))
        }
        (Object::Function(mapper), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Map(mapper.clone())))))
        }
        (Object::Function(mapper), Object::String(string)) => {
            let mut elements = Vector::new();
            for character in string.chars() {
                elements.push_back(mapper.apply(evaluator, vec![Rc::new(Object::String(character.to_string()))], source)?);
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
                if predicate.apply(evaluator, vec![Rc::clone(element)], source)?.is_truthy() {
                    elements.push_back(Rc::clone(element));
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
        (Object::Function(predicate), Object::Set(list)) => {
            let mut elements = HashSet::default();
            for element in list {
                if predicate.apply(evaluator, vec![Rc::clone(element)], source)?.is_truthy() {
                    elements.insert(Rc::clone(element));
                }
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        (Object::Function(predicate), Object::Hash(map)) => {
            let mut elements = HashMap::default();
            for (key, value) in map {
                if predicate.apply(evaluator, vec![Rc::clone(value), Rc::clone(key)], source)?.is_truthy() {
                    elements.insert(Rc::clone(key), Rc::clone(value));
                }
            }
            Ok(Rc::new(Object::Hash(elements)))
        }
        (Object::Function(predicate), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Filter(predicate.clone())))))
        }
        (Object::Function(predicate), Object::String(string)) => {
            let mut elements = Vector::new();
            for character in string.chars() {
                let object = Rc::new(Object::String(character.to_string()));
                if predicate.apply(evaluator, vec![Rc::clone(&object)], source)?.is_truthy() {
                    elements.push_back(Rc::clone(&object));
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
                accumulator = folder.apply(evaluator, vec![Rc::clone(&accumulator), Rc::clone(element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::Set(set)) => {
            let mut accumulator = Rc::clone(initial);
            for element in set {
                accumulator = folder.apply(evaluator, vec![Rc::clone(&accumulator), Rc::clone(element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::Hash(map)) => {
            let mut accumulator = Rc::clone(initial);
            for (key, value) in map {
                accumulator = folder.apply(evaluator, vec![Rc::clone(&accumulator), Rc::clone(value), Rc::clone(key)], source)?;
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
                accumulator = folder.apply(&mut shared_evaluator.borrow_mut(), vec![Rc::clone(&accumulator), Rc::clone(&element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (_, Object::Function(folder), Object::String(string)) => {
            let mut accumulator = Rc::clone(initial);
            for character in string.chars() {
                accumulator = folder.apply(evaluator, vec![Rc::clone(&accumulator), Rc::new(Object::String(character.to_string()))], source)?;
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
                side_effect.apply(evaluator, vec![Rc::clone(element)], source)?;
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::Set(set)) => {
            for element in set {
                side_effect.apply(evaluator, vec![Rc::clone(element)], source)?;
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::Hash(map)) => {
            for (key, value) in map {
                side_effect.apply(evaluator, vec![Rc::clone(value), Rc::clone(key)], source)?;
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::LazySequence(sequence)) => {
            let shared_evaluator = Rc::new(RefCell::new(evaluator));
            for element in sequence.resolve_iter(Rc::clone(&shared_evaluator), source) {
                side_effect.apply(&mut shared_evaluator.borrow_mut(), vec![Rc::clone(&element)], source)?;
            }
            Ok(Rc::new(Object::Nil))
        }
        (Object::Function(side_effect), Object::String(string)) => {
            for character in string.chars() {
                side_effect.apply(evaluator, vec![Rc::new(Object::String(character.to_string()))], source)?;
            }
            Ok(Rc::new(Object::Nil))
        }
    }
}

builtin! {
    reduce(reducer, collection) [evaluator, source] match {
        (Object::Function(reducer), Object::List(list)) => {
            let mut accumulator = match list.get(0) {
                Some(element) => Rc::clone(element),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty List".to_owned(),
                    source
                })
            };
            for element in list.iter().skip(1) {
                accumulator = reducer.apply(evaluator, vec![Rc::clone(&accumulator), Rc::clone(element)], source)?;
                if let Object::Break(value) = &*accumulator {
                    return Ok(Rc::clone(value));
                }
            }
            Ok(Rc::clone(&accumulator))
        }
        (Object::Function(reducer), Object::String(string)) => {
            let mut characters = string.chars();
            let mut accumulator = match characters.next() {
                Some(character) => Rc::new(Object::String(character.to_string())),
                None => return Err(RuntimeErr {
                    message: "Unable to reduce an empty String".to_owned(),
                    source
                })
            };
            for character in characters {
                accumulator = reducer.apply(evaluator, vec![Rc::clone(&accumulator), Rc::new(Object::String(character.to_string()))], source)?;
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
                if let Object::List(other_elements) = &*mapper.apply(evaluator, vec![Rc::clone(element)], source)? {
                    elements.append(other_elements.clone());
                }
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}

builtin! {
    skip(total, collection) [evaluator, source] match {
        (Object::Integer(total), Object::List(list)) => {
            Ok(Rc::new(Object::List(list.clone().into_iter().skip(*total as usize).collect())))
        }
        (Object::Integer(total), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::LazySequence(sequence.with_fn(LazyFn::Skip(*total as usize)))))
        }
    }
}

builtin! {
    take(total, collection) [evaluator, source] match {
        (Object::Integer(total), Object::List(list)) => {
            Ok(Rc::new(Object::List(list.clone().into_iter().take(*total as usize).collect())))
        }
        (Object::Integer(total), Object::LazySequence(sequence)) => {
            Ok(Rc::new(Object::List(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).take(*total as usize).collect::<Vector<Rc<Object>>>())))
        }
    }
}

builtin! {
    list(value) [evaluator, source] match {
        Object::List(list) => {
            Ok(Rc::new(Object::List(list.clone())))
        }
        Object::Set(set) => {
            Ok(Rc::new(Object::List(set.clone().into_iter().collect::<Vector<Rc<Object>>>())))
        }
        Object::Hash(map) => {
            let to_pairs = |(key, value)| Rc::new(Object::List(vec![key, value].into()));
            Ok(Rc::new(Object::List(map.clone().into_iter().map(to_pairs).collect::<Vector<Rc<Object>>>())))
        }
        Object::LazySequence(sequence) => {
            Ok(Rc::new(Object::List(sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source).collect::<Vector<Rc<Object>>>())))
        }
        Object::String(string) => {
            Ok(Rc::new(Object::List(string.chars().map(|character| Rc::new(Object::String(character.to_string()))).collect::<Vector<Rc<Object>>>())))
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
                        source
                    });
                }
                elements.insert(Rc::clone(element));
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
                        source
                    });
                }
                elements.insert(Rc::clone(&element));
            }
            Ok(Rc::new(Object::Set(elements)))
        }
        Object::String(string) => {
            Ok(Rc::new(Object::Set(string.chars().map(|character| Rc::new(Object::String(character.to_string()))).collect::<HashSet<_, _>>())))
        }
    }
}

builtin! {
    hash(value) [evaluator, source] match {
        Object::List(list) => {
            let mut elements = HashMap::default();

            for element in list.clone() {
                if let Object::List(pair) = &*element {
                    if pair.len() == 2 {
                        if !pair[0].is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to use a {} as a Hash key", pair[0].name()),
                                source
                            });
                        }
                        elements.insert(Rc::clone(&pair[0]), Rc::clone(&pair[1]));
                        continue;
                    }
                }

                return Err(RuntimeErr {
                    message: format!(
                        "Expected a [key, value] List pair, found: {}",
                        element.name()
                    ),
                    source,
                })
            }

            Ok(Rc::new(Object::Hash(elements)))
        }
        Object::LazySequence(sequence) => {
            let mut elements = HashMap::default();

            for element in sequence.resolve_iter(Rc::new(RefCell::new(evaluator)), source) {
                if let Object::List(pair) = &*element {
                    if pair.len() == 2 {
                        if !pair[0].is_hashable() {
                            return Err(RuntimeErr {
                                message: format!("Unable to use a {} as a Hash key", pair[0].name()),
                                source
                            });
                        }
                        elements.insert(Rc::clone(&pair[0]), Rc::clone(&pair[1]));
                        continue;
                    }
                }

                return Err(RuntimeErr {
                    message: format!(
                        "Expected a [key, value] List pair, found: {}",
                        element.name()
                    ),
                    source,
                })
            }

            Ok(Rc::new(Object::Hash(elements)))
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
            let characters = string.chars().map(|character| Rc::new(Object::String(character.to_string()))).collect::<Vector<Rc<Object>>>();
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
fn lazy_zipper(sequences: &Vector<Rc<Object>>) -> Option<Rc<Object>> {
    let mut zipped = Vec::with_capacity(sequences.len());
    for sequence in sequences {
        match &**sequence {
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
fn eager_zipper(sequences: Vector<Rc<Object>>, evaluator: &mut Evaluator, source: Location) -> Evaluation {
    let shared_evaluator = Rc::new(RefCell::new(evaluator));

    let mut iterators: Vec<Box<dyn Iterator<Item = Rc<Object>>>> = Vec::with_capacity(sequences.len());
    for sequence in &sequences {
        match &**sequence {
            Object::List(list) => iterators.push(Box::new(list.clone().into_iter())),
            Object::String(string) => iterators.push(Box::new(
                string
                    .chars()
                    .map(|character| Rc::new(Object::String(character.to_string()))),
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
                })
            }
        }
    }

    let mut zipped = Vector::new();
    'zipper: loop {
        let mut entry = Vector::new();
        for iterator in iterators.iter_mut() {
            match iterator.next() {
                Some(element) => entry.push_back(element),
                None => break 'zipper,
            }
        }
        zipped.push_back(Rc::new(Object::List(entry)));
    }

    Ok(Rc::new(Object::List(zipped)))
}

builtin! {
    zip(collection, ..collections) [evaluator, source] match {
        (_, Object::List(collections)) => {
            let mut collections = collections.clone();
            collections.push_front(Rc::clone(collection));

            if let Some(zipped) = lazy_zipper(&collections) {
                return Ok(zipped);
            }

            eager_zipper(collections, evaluator, source)
        }
    }
}
