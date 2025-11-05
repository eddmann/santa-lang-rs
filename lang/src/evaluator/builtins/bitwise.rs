use crate::evaluator::object::Object;
use std::rc::Rc;

builtin! {
    bit_and(a, b) [evaluator, _source] match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(evaluator.pool().integer(a & b))
        }
    }
}

builtin! {
    bit_or(a, b) [evaluator, _source] match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(evaluator.pool().integer(a | b))
        }
    }
}

builtin! {
    bit_xor(a, b) [evaluator, _source] match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(evaluator.pool().integer(a ^ b))
        }
    }
}

builtin! {
    bit_shift_left(value, shift) [evaluator, _source] match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(evaluator.pool().integer(value << shift))
        }
    }
}

builtin! {
    bit_shift_right(value, shift) [evaluator, _source] match {
        (Object::Integer(value), Object::Integer(shift)) => {
            Ok(evaluator.pool().integer(value >> shift))
        }
    }
}
