use crate::evaluator::object::Object;
use std::rc::Rc;

const SMALL_INT_MIN: i64 = -128;
const SMALL_INT_MAX: i64 = 127;
const SMALL_INT_COUNT: usize = 256;

#[derive(Debug)]
pub struct ObjectPool {
    nil: Rc<Object>,
    true_val: Rc<Object>,
    false_val: Rc<Object>,
    small_ints: Box<[Rc<Object>; SMALL_INT_COUNT]>,
    empty_list: Rc<Object>,
    empty_set: Rc<Object>,
    empty_dict: Rc<Object>,
}

impl ObjectPool {
    pub fn new() -> Self {
        // Pre-allocate all small integers
        let small_ints: Vec<Rc<Object>> = (SMALL_INT_MIN..=SMALL_INT_MAX)
            .map(|i| Rc::new(Object::Integer(i)))
            .collect();

        ObjectPool {
            nil: Rc::new(Object::Nil),
            true_val: Rc::new(Object::Boolean(true)),
            false_val: Rc::new(Object::Boolean(false)),
            small_ints: small_ints.try_into().unwrap(),
            empty_list: Rc::new(Object::List(im_rc::Vector::new())),
            empty_set: Rc::new(Object::Set(im_rc::HashSet::default())),
            empty_dict: Rc::new(Object::Dictionary(im_rc::HashMap::default())),
        }
    }

    #[inline]
    pub fn nil(&self) -> Rc<Object> {
        Rc::clone(&self.nil)
    }

    #[inline]
    pub fn boolean(&self, value: bool) -> Rc<Object> {
        if value {
            Rc::clone(&self.true_val)
        } else {
            Rc::clone(&self.false_val)
        }
    }

    #[inline]
    pub fn integer(&self, value: i64) -> Rc<Object> {
        if value >= SMALL_INT_MIN && value <= SMALL_INT_MAX {
            let index = (value - SMALL_INT_MIN) as usize;
            Rc::clone(&self.small_ints[index])
        } else {
            Rc::new(Object::Integer(value))  // Fallback for large integers
        }
    }

    #[inline]
    pub fn empty_list(&self) -> Rc<Object> {
        Rc::clone(&self.empty_list)
    }

    #[inline]
    pub fn empty_set(&self) -> Rc<Object> {
        Rc::clone(&self.empty_set)
    }

    #[inline]
    pub fn empty_dict(&self) -> Rc<Object> {
        Rc::clone(&self.empty_dict)
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_reuses_nil() {
        let pool = ObjectPool::new();
        let a = pool.nil();
        let b = pool.nil();
        assert!(Rc::ptr_eq(&a, &b));  // Same allocation
    }

    #[test]
    fn test_pool_reuses_booleans() {
        let pool = ObjectPool::new();
        let t1 = pool.boolean(true);
        let t2 = pool.boolean(true);
        let f1 = pool.boolean(false);
        let f2 = pool.boolean(false);

        assert!(Rc::ptr_eq(&t1, &t2));
        assert!(Rc::ptr_eq(&f1, &f2));
        assert!(!Rc::ptr_eq(&t1, &f1));
    }

    #[test]
    fn test_pool_reuses_small_integers() {
        let pool = ObjectPool::new();
        let a = pool.integer(42);
        let b = pool.integer(42);
        assert!(Rc::ptr_eq(&a, &b));  // Same allocation
    }

    #[test]
    fn test_pool_allocates_large_integers() {
        let pool = ObjectPool::new();
        let a = pool.integer(1000);
        let b = pool.integer(1000);
        assert!(!Rc::ptr_eq(&a, &b));  // Different allocations (expected)
    }

    #[test]
    fn test_pool_boundary_cases() {
        let pool = ObjectPool::new();

        // Test boundaries
        let min = pool.integer(-128);
        let max = pool.integer(127);
        let below = pool.integer(-129);
        let above = pool.integer(128);

        // Pooled values should be reused
        assert!(Rc::ptr_eq(&min, &pool.integer(-128)));
        assert!(Rc::ptr_eq(&max, &pool.integer(127)));

        // Non-pooled values shouldn't
        assert!(!Rc::ptr_eq(&below, &pool.integer(-129)));
        assert!(!Rc::ptr_eq(&above, &pool.integer(128)));
    }

    #[test]
    fn test_pool_empty_collections() {
        let pool = ObjectPool::new();

        let list1 = pool.empty_list();
        let list2 = pool.empty_list();
        assert!(Rc::ptr_eq(&list1, &list2));

        let set1 = pool.empty_set();
        let set2 = pool.empty_set();
        assert!(Rc::ptr_eq(&set1, &set2));

        let dict1 = pool.empty_dict();
        let dict2 = pool.empty_dict();
        assert!(Rc::ptr_eq(&dict1, &dict2));
    }
}
