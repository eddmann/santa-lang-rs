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
        indices: Vec<usize>,
        collection: Vector<Rc<Object>>,
    },
    // Concrete collection sources for transducer-like lazy evaluation
    FromList {
        items: Vector<Rc<Object>>,
        index: usize,
    },
    FromSet {
        items: Vec<Rc<Object>>,
        index: usize,
    },
    FromDict {
        items: Vec<(Rc<Object>, Rc<Object>)>,
        index: usize,
    },
    FromString {
        chars: Vec<char>,
        index: usize,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LazyFn {
    Map(Function),
    Filter(Function),
    FilterMap(Function),
    Skip(usize),
    Take(usize),
    FlatMap(Function),
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
            LazyValue::FromList { items, .. } => format!("[lazy list({})]", items.len()),
            LazyValue::FromSet { items, .. } => format!("[lazy set({})]", items.len()),
            LazyValue::FromDict { items, .. } => format!("[lazy dict({})]", items.len()),
            LazyValue::FromString { chars, .. } => format!("[lazy string({})]", chars.len()),
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
        let size = size as usize;
        let collection_len = collection.len();

        // If size > collection length, return empty sequence (no valid combinations)
        // Otherwise, initialize indices to [0, 1, 2, ..., size-1]
        let indices = if size > collection_len {
            vec![] // Will immediately return None in iterator
        } else {
            (0..size).collect()
        };

        Self {
            value: LazyValue::Combinations { indices, collection },
            functions: vec![],
        }
    }

    /// Create a lazy sequence from a List (for transducer-like lazy evaluation)
    pub fn from_list(items: Vector<Rc<Object>>) -> Self {
        Self {
            value: LazyValue::FromList { items, index: 0 },
            functions: vec![],
        }
    }

    /// Create a lazy sequence from a Set (for transducer-like lazy evaluation)
    pub fn from_set(items: Vec<Rc<Object>>) -> Self {
        Self {
            value: LazyValue::FromSet { items, index: 0 },
            functions: vec![],
        }
    }

    /// Create a lazy sequence from a Dictionary (yields [key, value] pairs)
    pub fn from_dict(items: Vec<(Rc<Object>, Rc<Object>)>) -> Self {
        Self {
            value: LazyValue::FromDict { items, index: 0 },
            functions: vec![],
        }
    }

    /// Create a lazy sequence from a String (yields individual characters)
    pub fn from_string(s: &str) -> Self {
        Self {
            value: LazyValue::FromString {
                chars: s.chars().collect(),
                index: 0,
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

    pub fn resolve_iter<'a>(
        &'a self,
        evaluator: Rc<RefCell<&'a mut Evaluator>>,
        source: Location,
    ) -> LazySequenceIter<'a> {
        LazySequenceIter {
            value: self.value.clone(),
            functions: self.functions.clone(),
            zip_iterators: HashMap::new(),
            flat_map_buffer: Vec::new(),
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

    /// If this is a range with negative indices, returns a new LazySequence
    /// with the indices adjusted relative to the given collection length.
    /// Returns None if no adjustment is needed.
    pub fn with_adjusted_negative_indices(&self, collection_len: usize) -> Option<Self> {
        let len = collection_len as i64;

        match &self.value {
            LazyValue::ExclusiveRange { current, until, .. } if *until < 0 || *current < 0 => {
                let adjusted_current = if *current < 0 { len + current } else { *current };
                let adjusted_until = if *until < 0 { len + until } else { *until };

                if adjusted_until <= adjusted_current || adjusted_current < 0 {
                    // Result would be empty
                    Some(Self {
                        value: LazyValue::ExclusiveRange {
                            current: 0,
                            until: 0, // Empty range
                            step: 1,
                        },
                        functions: self.functions.clone(),
                    })
                } else {
                    Some(Self {
                        value: LazyValue::ExclusiveRange {
                            current: adjusted_current,
                            until: adjusted_until,
                            step: 1,
                        },
                        functions: self.functions.clone(),
                    })
                }
            }
            LazyValue::InclusiveRange { current, to, .. } if *to < 0 || *current < 0 => {
                let adjusted_current = if *current < 0 { len + current } else { *current };
                let adjusted_to = if *to < 0 { len + to } else { *to };

                if adjusted_to < adjusted_current || adjusted_current < 0 {
                    // Result would be empty
                    Some(Self {
                        value: LazyValue::ExclusiveRange {
                            current: 0,
                            until: 0, // Empty range
                            step: 1,
                        },
                        functions: self.functions.clone(),
                    })
                } else {
                    Some(Self {
                        value: LazyValue::InclusiveRange {
                            current: adjusted_current,
                            to: adjusted_to,
                            step: 1,
                        },
                        functions: self.functions.clone(),
                    })
                }
            }
            _ => None,
        }
    }
}

pub struct LazySequenceIter<'a> {
    value: LazyValue,
    functions: Vec<LazyFn>,
    evaluator: Rc<RefCell<&'a mut Evaluator>>,
    zip_iterators: HashMap<usize, Vec<LazySequenceIter<'a>>>,
    flat_map_buffer: Vec<Rc<Object>>,
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
                ref mut indices,
                ref collection,
            } => {
                if indices.is_empty() {
                    return None;
                }

                let size = indices.len();
                let n = collection.len();

                // Build the current combination from indices
                let result: Vector<Rc<Object>> = indices.iter().map(|&i| Rc::clone(&collection[i])).collect();

                // Advance to next combination
                // Find the rightmost index that can be incremented
                let mut i = size;
                while i > 0 {
                    i -= 1;
                    if indices[i] < n - size + i {
                        // Increment this index and reset all following indices
                        indices[i] += 1;
                        for j in (i + 1)..size {
                            indices[j] = indices[j - 1] + 1;
                        }
                        return Some(Rc::new(Object::List(result)));
                    }
                }

                // No more combinations - clear indices to signal exhaustion
                indices.clear();
                Some(Rc::new(Object::List(result)))
            }
            // Concrete collection sources for transducer-like lazy evaluation
            LazyValue::FromList {
                ref items,
                ref mut index,
            } => {
                if *index >= items.len() {
                    return None;
                }
                let next = Rc::clone(&items[*index]);
                *index += 1;
                Some(next)
            }
            LazyValue::FromSet {
                ref items,
                ref mut index,
            } => {
                if *index >= items.len() {
                    return None;
                }
                let next = Rc::clone(&items[*index]);
                *index += 1;
                Some(next)
            }
            LazyValue::FromDict {
                ref items,
                ref mut index,
            } => {
                if *index >= items.len() {
                    return None;
                }
                let (key, value) = &items[*index];
                let pair = Vector::from(vec![Rc::clone(key), Rc::clone(value)]);
                *index += 1;
                Some(Rc::new(Object::List(pair)))
            }
            LazyValue::FromString {
                ref chars,
                ref mut index,
            } => {
                if *index >= chars.len() {
                    return None;
                }
                let next = Rc::new(Object::String(chars[*index].to_string()));
                *index += 1;
                Some(next)
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
        // First check for buffered items from FlatMap
        if let Some(buffered) = self.flat_map_buffer.pop() {
            return Some(buffered);
        }

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
                    LazyFn::Take(total) => {
                        if *total == 0 {
                            return None;
                        }
                        *function = LazyFn::Take(*total - 1);
                    }
                    LazyFn::FlatMap(mapper) => {
                        // Apply the mapper to get a collection/sequence
                        let result = mapper
                            .apply(&mut self.evaluator.borrow_mut(), vec![Rc::clone(&next)], self.source)
                            .ok()?;

                        // Extract items from the result
                        // For LazySequence, we need to collect it within its own scope
                        let mut items: Vec<Rc<Object>> = match &*result {
                            Object::List(list) => list.iter().cloned().collect(),
                            Object::LazySequence(seq) => {
                                // Clone the sequence and collect items
                                // Need to create a new iterator to avoid lifetime issues
                                let seq = seq.clone();
                                let mut inner_iter = LazySequenceIter {
                                    value: seq.value.clone(),
                                    functions: seq.functions.clone(),
                                    zip_iterators: HashMap::new(),
                                    flat_map_buffer: Vec::new(),
                                    evaluator: Rc::clone(&self.evaluator),
                                    source: self.source,
                                };
                                let mut collected = Vec::new();
                                while let Some(item) = inner_iter.next() {
                                    collected.push(item);
                                }
                                collected
                            }
                            _ => {
                                // Non-collection result is treated as single element
                                vec![Rc::clone(&result)]
                            }
                        };

                        if items.is_empty() {
                            continue 'next; // Empty result, get next source item
                        }

                        // Take first item and buffer the rest (reversed so we can pop)
                        let first = items.remove(0);
                        items.reverse();
                        self.flat_map_buffer = items;
                        next = first;
                    }
                    LazyFn::Zip(sequences) => {
                        let mut entry = Vector::new();
                        entry.push_back(next);

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
                                        flat_map_buffer: Vec::new(),
                                        evaluator: Rc::clone(&self.evaluator),
                                        source: self.source,
                                    })
                                    .collect()
                            });

                        for iterator in iterators.iter_mut() {
                            match iterator.next() {
                                Some(element) => entry.push_back(element),
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
