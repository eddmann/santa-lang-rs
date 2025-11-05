use crate::evaluator::object::Object;
use im_rc::Vector;
use ordered_float::OrderedFloat;
use std::rc::Rc;

builtin! {
    abs(value) [evaluator, _source] match {
        Object::Integer(value) => {
            Ok(evaluator.pool().integer(value.abs()))
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
                added.push_back(crate::evaluator::builtins::operators::plus(evaluator, v1, v2, source)?);
            }
            Ok(Rc::new(Object::List(added)))
        }
    }
}

builtin! {
    signum(value) [evaluator, _source] match {
        Object::Integer(value) => {
            Ok(evaluator.pool().integer(value.signum()))
        }
        Object::Decimal(OrderedFloat(value)) => {
            Ok(Rc::new(Object::Decimal(OrderedFloat(value.signum()))))
        }
    }
}
