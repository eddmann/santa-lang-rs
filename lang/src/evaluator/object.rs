use crate::evaluator::Function;
use crate::evaluator::lazy_sequence::LazySequence;
use im_rc::{HashMap, HashSet, Vector};
use ordered_float::OrderedFloat;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Object {
    Nil,
    Integer(i64),
    Decimal(OrderedFloat<f64>),
    Boolean(bool),
    String(String),

    List(Vector<Rc<Object>>),
    Set(HashSet<Rc<Object>, BuildHasherDefault<DefaultHasher>>),
    Dictionary(HashMap<Rc<Object>, Rc<Object>, BuildHasherDefault<DefaultHasher>>),
    LazySequence(LazySequence),

    Function(Function),

    Placeholder,
    Return(Rc<Object>),
    Break(Rc<Object>),
}

impl Object {
    pub fn name(&self) -> String {
        match self {
            Self::Nil => "Nil".to_owned(),
            Self::Integer(_) => "Integer".to_owned(),
            Self::Decimal(_) => "Decimal".to_owned(),
            Self::Boolean(_) => "Boolean".to_owned(),
            Self::String(_) => "String".to_owned(),

            Self::List(_) => "List".to_owned(),
            Self::Set(_) => "Set".to_owned(),
            Self::Dictionary(_) => "Dictionary".to_owned(),
            Self::LazySequence(sequence) => {
                if sequence.is_unbounded() {
                    "UnboundedRange".to_owned()
                } else {
                    "BoundedRange".to_owned()
                }
            }

            Self::Function(_) => "Function".to_owned(),

            Self::Placeholder => "Placeholder".to_owned(),
            Self::Return(v) => v.name(),
            Self::Break(v) => v.name(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Integer(v) => *v != 0,
            Self::Decimal(OrderedFloat(v)) => *v != 0.0,
            Self::Boolean(v) => *v,
            Self::String(v) => !v.is_empty(),

            Self::List(v) => !v.is_empty(),
            Self::Set(v) => !v.is_empty(),
            Self::Dictionary(v) => !v.is_empty(),
            Self::LazySequence(_) => true,

            Self::Function(_) => true,

            Self::Placeholder => false,
            Self::Return(v) => v.is_truthy(),
            Self::Break(v) => v.is_truthy(),
        }
    }

    pub fn is_hashable(&self) -> bool {
        match self {
            Self::Nil | Self::Integer(_) | Self::Decimal(_) | Self::Boolean(_) | Self::String(_) | Self::Set(_) => true,
            Self::List(list) => list.iter().all(|element| element.is_hashable()),
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Nil => "nil".to_owned(),
            Self::Integer(v) => format!("{}", v),
            Self::Decimal(v) => format!("{}", v),
            Self::Boolean(v) => format!("{}", v),
            Self::String(v) => format!("\"{}\"", v),

            Self::List(v) => {
                let elements: Vec<String> = v.iter().map(|element| element.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Self::Set(v) => {
                let elements: Vec<String> = v.iter().map(|element| element.to_string()).collect();
                format!("{{{}}}", elements.join(", "))
            }
            Self::Dictionary(v) => {
                let formatted: Vec<String> = v.iter().map(|(key, value)| format!("{}: {}", key, value)).collect();
                format!("#{{{}}}", formatted.join(", "))
            }
            Self::LazySequence(sequence) => sequence.to_string(),

            Self::Function(function) => format!("{}", function),

            Self::Placeholder => "_".to_owned(),
            Self::Return(v) => format!("{}", v),
            Self::Break(v) => format!("{}", v),
        };
        write!(f, "{}", s)
    }
}
