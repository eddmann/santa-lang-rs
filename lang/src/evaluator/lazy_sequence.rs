use crate::evaluator::{Evaluator, Function, Object};
use crate::lexer::Location;
use im_rc::Vector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum LazyValue {
    InclusiveRange { current: i64, to: i64, step: i64 },
    ExclusiveRange { current: i64, until: i64, step: i64 },
    UnboundedRange { current: i64, step: i64 },
    Repeat { value: Rc<Object> },
    Cycle { index: usize, list: Vector<Rc<Object>> },
    Iterate { current: Rc<Object>, generator: Function },
}

#[derive(Debug, Clone)]
pub enum LazyFn {
    Map(Function),
    Filter(Function),
    FilterMap(Function),
    Skip(usize),
    Zip(Vec<LazySequence>),
}

#[derive(Debug, Clone)]
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
        };
        write!(f, "{}", s)
    }
}

impl PartialEq for LazySequence {
    fn eq(&self, other: &Self) -> bool {
        false
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
                        if predicate
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
