use crate::evaluator::lazy_sequence::LazySequence;
use crate::evaluator::Function;
use im_rc::{HashMap, HashSet, Vector};
use ordered_float::OrderedFloat;
use std::cell::{OnceCell, RefCell};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap as StdHashMap;
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

    List(Vector<Object>),
    Set(HashSet<Object, BuildHasherDefault<DefaultHasher>>),
    Dictionary(HashMap<Object, Object, BuildHasherDefault<DefaultHasher>>),
    LazySequence(LazySequence),

    Function(Function),

    Placeholder,
    Return(Rc<Object>),
    Break(Rc<Object>),
}

impl Object {
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            Self::Nil => "Nil",
            Self::Integer(_) => "Integer",
            Self::Decimal(_) => "Decimal",
            Self::Boolean(_) => "Boolean",
            Self::String(_) => "String",

            Self::List(_) => "List",
            Self::Set(_) => "Set",
            Self::Dictionary(_) => "Dictionary",
            Self::LazySequence(_) => "LazySequence",

            Self::Function(_) => "Function",

            Self::Placeholder => "Placeholder",
            Self::Return(v) => v.name(),
            Self::Break(v) => v.name(),
        }
    }

    #[inline]
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

    #[inline]
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

// Small integer cache for commonly used integers (-128 to 127)
// Avoids repeated Rc allocations for small integers used in loops
const SMALL_INT_MIN: i64 = -128;
const SMALL_INT_MAX: i64 = 127;
const SMALL_INT_CACHE_SIZE: usize = (SMALL_INT_MAX - SMALL_INT_MIN + 1) as usize;

thread_local! {
    static SMALL_INT_CACHE: OnceCell<[Rc<Object>; SMALL_INT_CACHE_SIZE]> = OnceCell::new();
}

fn init_small_int_cache() -> [Rc<Object>; SMALL_INT_CACHE_SIZE] {
    std::array::from_fn(|i| {
        let value = SMALL_INT_MIN + i as i64;
        Rc::new(Object::Integer(value))
    })
}

/// Create an Rc<Object::Integer>, using the cache for small integers
#[inline]
pub fn new_integer(value: i64) -> Rc<Object> {
    if value >= SMALL_INT_MIN && value <= SMALL_INT_MAX {
        SMALL_INT_CACHE.with(|cache| {
            let cache = cache.get_or_init(init_small_int_cache);
            let index = (value - SMALL_INT_MIN) as usize;
            Rc::clone(&cache[index])
        })
    } else {
        Rc::new(Object::Integer(value))
    }
}

// String interning cache for commonly used strings (≤ 64 chars)
// Reduces allocations for repeated strings like single characters, keywords, etc.
thread_local! {
    static STRING_CACHE: RefCell<StdHashMap<String, Rc<Object>>> = RefCell::new(StdHashMap::new());
}

/// Create an Rc<Object::String>, using interning for small strings
/// Interns strings ≤ 64 characters for better memory efficiency
#[inline]
pub fn new_string(value: String) -> Rc<Object> {
    // Only intern small strings to avoid caching large data
    if value.len() <= 64 {
        STRING_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if let Some(cached) = cache.get(&value) {
                Rc::clone(cached)
            } else {
                let obj = Rc::new(Object::String(value.clone()));
                cache.insert(value, Rc::clone(&obj));
                obj
            }
        })
    } else {
        Rc::new(Object::String(value))
    }
}
