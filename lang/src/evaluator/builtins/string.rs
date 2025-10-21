use crate::evaluator::object::Object;
use crate::evaluator::RuntimeErr;
use im_rc::Vector;
use ordered_float::OrderedFloat;
use regex::Regex;
use std::rc::Rc;

builtin! {
    int(value) match {
        Object::Boolean(value) => {
            Ok(Rc::new(Object::Integer(if *value { 1 } else { 0 })))
        }
        Object::Integer(value) => {
            Ok(Rc::new(Object::Integer(*value)))
        }
        Object::Decimal(OrderedFloat(value)) => {
            Ok(Rc::new(Object::Integer(value.round() as i64)))
        }
        Object::String(value) => {
            if let Ok(parsed) = value.trim().parse::<i64>() {
                return Ok(Rc::new(Object::Integer(parsed)));
            }

            if let Ok(parsed) = value.trim().parse::<f64>() {
                return Ok(Rc::new(Object::Integer(parsed.round() as i64)))
            }

            Ok(Rc::new(Object::Integer(0)))
        }
    }
}

builtin! {
    ints(value) match {
        Object::String(value) => {
            let pattern = Regex::new(r"(-?[0-9]+)")
                .expect("Hardcoded regex pattern should always compile");

            let mut ints = Vector::new();
            for capture in pattern.captures_iter(value) {
                if let Ok(parsed) = capture[0].parse::<i64>() {
                    ints.push_back(Object::Integer(parsed));
                }
            }

            Ok(Rc::new(Object::List(ints)))
        }
    }
}

builtin! {
    lines(value) match {
        Object::String(value) => {
            Ok(Rc::new(Object::List(value.lines().map(|line| Object::String(line.to_owned())).collect())))
        }
    }
}

builtin! {
    split(seperator, value) match {
        (Object::String(seperator), Object::String(value)) => {
            if seperator.is_empty() {
                return Ok(Rc::new(Object::List(value.chars().map(|seperated| Object::String(seperated.to_string())).collect())))
            }
            Ok(Rc::new(Object::List(value.split(seperator).map(|seperated| Object::String(seperated.to_owned())).collect())))
        }
    }
}

builtin! {
    regex_match(pattern, value) [evaluator, source] match {
        (Object::String(pattern), Object::String(value)) => {
            match Regex::new(pattern) {
                Ok(compiled_pattern) => {
                    if let Some(matched) = compiled_pattern.captures(value) {
                        return Ok(Rc::new(Object::List(
                            matched
                                .iter()
                                .skip(1)
                                .filter_map(|matched| matched.map(|m| Object::String(m.as_str().to_owned())))
                                .collect()
                            )
                        ));
                    }
                    Ok(Rc::new(Object::List(Vector::new())))
                }
                Err(_) => {
                    Err(RuntimeErr {
                        message: format!("Failed to compile regex pattern: {}", pattern),
                        source,
                        trace: evaluator.get_trace()
                    })
                }
            }
        }
    }
}

builtin! {
    regex_match_all(pattern, value) [evaluator, source] match {
        (Object::String(pattern), Object::String(value)) => {
            match Regex::new(pattern) {
                Ok(compiled_pattern) => {
                    Ok(Rc::new(Object::List(
                        compiled_pattern
                            .captures_iter(value)
                            .filter_map(|matched| matched.get(0).map(|m| Object::String(m.as_str().to_owned())))
                            .collect()
                    )))
                }
                Err(_) => {
                    Err(RuntimeErr {
                        message: format!("Failed to compile regex pattern: {}", pattern),
                        source,
                        trace: evaluator.get_trace()
                    })
                }
            }
        }
    }
}
