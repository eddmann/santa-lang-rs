use crate::evaluator::{Evaluator, Function, Object};
use crate::lexer::Location;
use im_rc::Vector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum LazyValue {
    InclusiveRange {
        current: i64,
        to: i64,
        step: i64,
    },
    ExclusiveRange {
        current: i64,
        until: i64,
        step: i64,
    },
    UnboundedRange {
        current: i64,
        step: i64,
    },
    Repeat {
        value: Rc<Object>,
    },
    Cycle {
        index: usize,
        list: Vector<Rc<Object>>,
    },
    Iterate {
        current: Rc<Object>,
        generator: Function,
    },
    Combinations {
        size: u32,
        min: usize,
        mask: usize,
        collection: Vector<Rc<Object>>,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LazyFn {
    Map(Function),
    Filter(Function),
    FilterMap(Function),
    Skip(usize),
    Zip(Vec<LazySequence>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LazySequence {
    value: LazyValue,
    functions: Vec<LazyFn>,
}

impl fmt::Display for LazySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self.value {
            LazyValue::InclusiveRange { current, to, .. } => format!("{}..={}", current, to),
            LazyValue::ExclusiveRange { current, until, .. } => format!("{}..{}", current, until),
            LazyValue::UnboundedRange { current, .. } => format!("{}..", current),
            LazyValue::Repeat { value } => format!("[{}, ∞]", value),
            LazyValue::Cycle { .. } => "[cycle ∞]".to_owned(),
            LazyValue::Iterate { .. } => "[iterate ∞]".to_owned(),
            LazyValue::Combinations { .. } => "[combinations]".to_owned(),
        };
        write!(f, "{}", s)
    }
}

impl LazySequence {
    pub fn inclusive_range(from: i64, to: i64) -> Self {
        Self {
            value: LazyValue::InclusiveRange {
                current: from,
                to,
                step: if from < to { 1 } else { -1 },
            },
            functions: vec![],
        }
    }

    pub fn inclusive_range_with_step(from: i64, to: i64, step: i64) -> Self {
        Self {
            value: LazyValue::InclusiveRange {
                current: from,
                to,
                step,
            },
            functions: vec![],
        }
    }

    pub fn exclusive_range(from: i64, until: i64) -> Self {
        Self {
            value: LazyValue::ExclusiveRange {
                current: from,
                until,
                step: if from < until { 1 } else { -1 },
            },
            functions: vec![],
        }
    }

    pub fn unbounded_range(from: i64) -> Self {
        Self {
            value: LazyValue::UnboundedRange { current: from, step: 1 },
            functions: vec![],
        }
    }

    pub fn repeat(value: Rc<Object>) -> Self {
        Self {
            value: LazyValue::Repeat { value },
            functions: vec![],
        }
    }

    pub fn cycle(list: Vector<Rc<Object>>) -> Self {
        Self {
            value: LazyValue::Cycle { index: 0, list },
            functions: vec![],
        }
    }

    pub fn iterate(generator: Function, initial: Rc<Object>) -> Self {
        Self {
            value: LazyValue::Iterate {
                current: initial,
                generator,
            },
            functions: vec![],
        }
    }

    pub fn combinations(size: u32, collection: Vector<Rc<Object>>) -> Self {
        let collection_len = collection.len();
        let min = 2_usize.pow(size) - 1;
        let max = if collection_len >= size as usize {
            2_usize.pow(collection_len as u32) - 2_usize.pow((collection_len - size as usize) as u32)
        } else {
            0
        };

        Self {
            value: LazyValue::Combinations {
                size,
                min,
                mask: max,
                collection,
            },
            functions: vec![],
        }
    }

    pub fn with_fn(&self, function: LazyFn) -> Self {
        let mut functions = self.functions.clone();
        functions.push(function);

        Self {
            value: self.value.clone(),
            functions,
        }
    }

    pub fn resolve_iter<'a>(&'a self, evaluator: Rc<RefCell<&'a mut Evaluator>>, source: Location) -> LazySequenceIter {
        LazySequenceIter {
            value: self.value.clone(),
            functions: self.functions.clone(),
            zip_iterators: HashMap::new(),
            evaluator,
            source,
        }
    }

    pub fn is_unbounded_negative_range(&self) -> bool {
        match self.value {
            LazyValue::UnboundedRange { current, .. } => current < 0,
            _ => false,
        }
    }
}

pub struct LazySequenceIter<'a> {
    value: LazyValue,
    functions: Vec<LazyFn>,
    evaluator: Rc<RefCell<&'a mut Evaluator>>,
    zip_iterators: HashMap<usize, Vec<LazySequenceIter<'a>>>,
    source: Location,
}

impl LazySequenceIter<'_> {
    fn next_value(&mut self) -> Option<Rc<Object>> {
        match self.value {
            LazyValue::InclusiveRange {
                ref mut current,
                to,
                step,
            } => {
                if (step > 0 && *current > to) || (step < 0 && *current < to) {
                    return None;
                }
                let next = Rc::new(Object::Integer(*current));
                *current += step;
                Some(next)
            }
            LazyValue::ExclusiveRange {
                ref mut current,
                until,
                step,
            } => {
                if (step > 0 && *current >= until) || (step < 0 && *current <= until) {
                    return None;
                }
                let next = Rc::new(Object::Integer(*current));
                *current += step;
                Some(next)
            }
            LazyValue::UnboundedRange { ref mut current, step } => {
                let next = Rc::new(Object::Integer(*current));
                *current += step;
                Some(next)
            }
            LazyValue::Repeat { ref value } => Some(Rc::clone(value)),
            LazyValue::Cycle {
                ref mut index,
                ref list,
            } => {
                let next = Rc::clone(list.get(*index)?);
                *index = (*index + 1) % list.len();
                Some(next)
            }
            LazyValue::Iterate {
                ref mut current,
                ref generator,
            } => {
                let next = Rc::clone(current);

                *current = generator
                    .apply(&mut self.evaluator.borrow_mut(), vec![Rc::clone(current)], self.source)
                    .ok()?;

                Some(next)
            }
            LazyValue::Combinations {
                size,
                min,
                ref mut mask,
                ref collection,
            } => {
                while *mask >= min {
                    if mask.count_ones() == size {
                        let b = format!("{:01$b}", mask, collection.len());
                        let res = b
                            .chars()
                            .enumerate()
                            .filter(|&(_, e)| e == '1')
                            .map(|(i, _)| (*collection[i]).clone())
                            .collect::<Vector<Object>>();
                        *mask -= 1;
                        return Some(Rc::new(Object::List(res)));
                    }
                    *mask -= 1;
                }
                None
            }
        }
    }

    pub fn to_sequence(&self) -> LazySequence {
        LazySequence {
            value: self.value.clone(),
            functions: self.functions.clone(),
        }
    }
}

impl Iterator for LazySequenceIter<'_> {
    type Item = Rc<Object>;

    fn next(&mut self) -> Option<Rc<Object>> {
        'next: loop {
            let mut next = self.next_value()?;

            for function in self.functions.iter_mut() {
                match function {
                    LazyFn::Map(mapper) => {
                        next = mapper
                            .apply(&mut self.evaluator.borrow_mut(), vec![Rc::clone(&next)], self.source)
                            .ok()?;
                    }
                    LazyFn::Filter(predicate) => {
                        if !predicate
                            .apply(&mut self.evaluator.borrow_mut(), vec![Rc::clone(&next)], self.source)
                            .ok()?
                            .is_truthy()
                        {
                            continue 'next;
                        }
                    }
                    LazyFn::FilterMap(mapper) => {
                        next = mapper
                            .apply(&mut self.evaluator.borrow_mut(), vec![Rc::clone(&next)], self.source)
                            .ok()?;
                        if !next.is_truthy() {
                            continue 'next;
                        }
                    }
                    LazyFn::Skip(total) => {
                        if *total > 0 {
                            *function = LazyFn::Skip(*total - 1);
                            continue 'next;
                        }
                    }
                    LazyFn::Zip(sequences) => {
                        let mut entry = Vector::new();
                        entry.push_back((*next).clone());

                        let iterators = self
                            .zip_iterators
                            .entry(sequences.as_ptr() as usize)
                            .or_insert_with(|| {
                                sequences
                                    .iter()
                                    .map(|sequence| LazySequenceIter {
                                        value: sequence.value.clone(),
                                        functions: sequence.functions.clone(),
                                        zip_iterators: HashMap::new(),
                                        evaluator: Rc::clone(&self.evaluator),
                                        source: self.source,
                                    })
                                    .collect()
                            });

                        for iterator in iterators.iter_mut() {
                            match iterator.next() {
                                Some(element) => entry.push_back((*element).clone()),
                                None => return None,
                            }
                        }

                        next = Rc::new(Object::List(entry));
                    }
                }
            }

            return Some(next);
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LazySequence {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom("Unable to serialize LazySequence"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LazySequence {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom("Unable to deserialize LazySequence"))
    }
}
