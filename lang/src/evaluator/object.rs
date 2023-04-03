use crate::evaluator::lazy_sequence::LazySequence;
use crate::evaluator::Function;
use im_rc::{HashMap, HashSet, Vector};
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
    String(String),

    List(Vector<Rc<Object>>),
    Set(HashSet<Rc<Object>>),
    Hash(HashMap<Rc<Object>, Rc<Object>>),
    LazySequence(LazySequence),

    Function(Function),

    Placeholder,
    Return(Rc<Object>),
    Break(Rc<Object>),
}

impl Object {
    pub fn name(&self) -> &str {
        match self {
            Self::Nil => "Nil",
            Self::Integer(_) => "Integer",
            Self::Decimal(_) => "Decimal",
            Self::Boolean(_) => "Boolean",
            Self::String(_) => "String",

            Self::List(_) => "List",
            Self::Set(_) => "Set",
            Self::Hash(_) => "Hash",
            Self::LazySequence(_) => "LazySequence",

            Self::Function(_) => "Function",

            Self::Placeholder => "Placeholder",
            Self::Return(v) => v.name(),
            Self::Break(v) => v.name(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Integer(v) => *v > 0,
            Self::Decimal(v) => *v > 0.0,
            Self::Boolean(v) => *v,
            Self::String(v) => !v.is_empty(),

            Self::List(v) => !v.is_empty(),
            Self::Set(v) => !v.is_empty(),
            Self::Hash(v) => !v.is_empty(),
            Self::LazySequence(_) => true,

            Self::Function(_) => true,

            Self::Placeholder => false,
            Self::Return(v) => v.is_truthy(),
            Self::Break(v) => v.is_truthy(),
        }
    }

    pub fn is_hashable(&self) -> bool {
        matches!(
            self,
            Self::Nil
                | Self::Integer(_)
                | Self::Decimal(_)
                | Self::Boolean(_)
                | Self::String(_)
                | Self::List(_)
                | Self::Set(_)
                | Self::Hash(_)
        )
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
            Self::Hash(v) => {
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

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Integer(v1), Self::Integer(v2)) => v1 == v2,
            (Self::Decimal(v1), Self::Decimal(v2)) => v1 == v2,
            (Self::Boolean(v1), Self::Boolean(v2)) => v1 == v2,
            (Self::String(v1), Self::String(v2)) => v1 == v2,

            (Self::List(v1), Self::List(v2)) => v1 == v2,
            (Self::Set(v1), Self::Set(v2)) => v1 == v2,
            (Self::Hash(v1), Self::Hash(v2)) => v1 == v2,
            (Self::LazySequence(v1), Self::LazySequence(v2)) => v1 == v2,

            (Self::Function(v1), Self::Function(v2)) => v1 == v2,

            (Self::Placeholder, Self::Placeholder) => false,
            (Self::Return(v1), Self::Return(v2)) => v1 == v2,
            (v1, Self::Return(v2)) => v1.eq(v2),
            (Self::Return(v1), v2) => v2.eq(v1),
            (Self::Break(v1), Self::Break(v2)) => v1 == v2,
            (v1, Self::Break(v2)) => v1.eq(v2),
            (Self::Break(v1), v2) => v2.eq(v1),

            _ => false,
        }
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Nil => {}
            Self::Integer(v) => state.write_i64(*v),
            Self::Decimal(v) => state.write_u64(v.to_bits()),
            Self::Boolean(v) => state.write_u8(*v as u8),
            Self::String(v) => state.write(v.as_bytes()),

            Self::List(v) => v.hash(state),
            Self::Set(v) => v.hash(state),
            Self::Hash(v) => v.hash(state),

            _ => unreachable!(),
        }
    }
}
