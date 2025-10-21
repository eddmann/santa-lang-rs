use crate::evaluator::object::{new_integer, Object};
use std::rc::Rc;

builtin! {
    bit_and(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(new_integer(a & b))
        }
    }
}

builtin! {
    bit_or(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(new_integer(a | b))
        }
    }
}

builtin! {
    bit_xor(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(new_integer(a ^ b))
        }
    }
}

builtin! {
    bit_shift_left(value, shift) match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(new_integer(value << shift))
        }
    }
}

builtin! {
    bit_shift_right(value, shift) match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(new_integer(value >> shift))
        }
    }
}
