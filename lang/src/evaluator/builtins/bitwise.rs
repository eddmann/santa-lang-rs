use crate::evaluator::object::Object;
use std::rc::Rc;

builtin! {
    bit_and(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(Rc::new(Object::Integer(a & b)))
        }
    }
}

builtin! {
    bit_or(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(Rc::new(Object::Integer(a | b)))
        }
    }
}

builtin! {
    bit_xor(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(Rc::new(Object::Integer(a ^ b)))
        }
    }
}

builtin! {
    bit_shift_left(value, shift) match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(Rc::new(Object::Integer(value << shift)))
        }
    }
}

builtin! {
    bit_shift_right(value, shift) match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(Rc::new(Object::Integer(value >> shift)))
        }
    }
}

builtin! {
    bit_not(value) match {
        Object::Integer(value) => {
            Ok(Rc::new(Object::Integer(!value)))
        }
    }
}
