use crate::evaluator::object::Object;
use std::rc::Rc;

builtin! {
    plus(a, b) [evaulator, source] {
        crate::evaluator::infix::apply_infix_plus(evaulator, a, b, source)
    }
}

builtin! {
    minus(a, b) [evaulator, source] {
        crate::evaluator::infix::apply_infix_minus(evaulator, a, b, source)
    }
}
