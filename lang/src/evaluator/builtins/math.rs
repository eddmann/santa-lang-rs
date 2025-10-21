use crate::evaluator::object::{new_integer, Object};
use im_rc::Vector;
use ordered_float::OrderedFloat;
use std::rc::Rc;

builtin! {
    abs(value) match {
        Object::Integer(value) => {
            Ok(new_integer(value.abs()))
        }
        Object::Decimal(OrderedFloat(value)) => {
            Ok(Rc::new(Object::Decimal(OrderedFloat(value.abs()))))
        }
    }
}

builtin! {
    vec_add(a, b) [evaluator, source] match {
        (Object::List(a), Object::List(b)) => {
            let mut added = Vector::new();
            for (v1, v2) in a.iter().zip(b.iter()) {
                let result = crate::evaluator::builtins::operators::plus(evaluator, &Rc::new(v1.clone()), &Rc::new(v2.clone()), source)?;
                added.push_back((*result).clone());
            }
            Ok(Rc::new(Object::List(added)))
        }
    }
}

builtin! {
    signum(value) match {
        Object::Integer(value) => {
            Ok(new_integer(value.signum()))
        }
        Object::Decimal(OrderedFloat(value)) => {
            Ok(Rc::new(Object::Decimal(OrderedFloat(value.signum()))))
        }
    }
}
