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
    bit_shift_left(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(Rc::new(Object::Integer(a << b)))
        }
    }
}

builtin! {
    bit_shift_right(a, b) match {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(Rc::new(Object::Integer(a >> b)))
        }
    }
}
