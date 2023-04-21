use crate::evaluator::function::Function;
use crate::evaluator::object::Object;
use crate::evaluator::LazySequence;
use crate::evaluator::RuntimeErr;
use std::cell::RefCell;
use std::rc::Rc;

builtin! {
    range(from, to, step) match {
        (Object::Integer(from), Object::Integer(to), Object::Integer(step)) => {
            Ok(Rc::new(Object::LazySequence(LazySequence::inclusive_range_with_step(*from, *to, *step))))
        }
    }
}

builtin! {
    type_name(value) {
        Ok(Rc::new(Object::String(value.name().to_owned())))
    }
}

builtin! {
    id(value) {
        Ok(Rc::clone(value))
    }
}

builtin! {
    memoize(function) [evaluator, source] match {
        Object::Function(Function::Closure { parameters, body, environment, }) => {
            let function = Function::MemoizedClosure {
                parameters: parameters.clone(),
                body: body.clone(),
                environment: Rc::clone(environment),
                cache: Rc::new(RefCell::new(std::collections::HashMap::default()))
            };
            Ok(Rc::new(Object::Function(function)))
        }
    }
}

builtin! {
    evaluate(source) [evaluator, source_location] match {
        Object::String(source) => {
            let environment = crate::Environment::new();
            let lexer = crate::lexer::Lexer::new(source);
            let mut parser = crate::parser::Parser::new(lexer);
            let program = match parser.parse() {
                Ok(program) => program,
                Err(error) => return Err(RuntimeErr {
                    message: format!("{:?}", error),
                    source: source_location,
                    trace: vec![],
                })
            };
            evaluator.evaluate_with_environment(&program, Rc::clone(&environment))
        }
    }
}
