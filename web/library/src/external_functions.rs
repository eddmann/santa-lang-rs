use js_sys::{Array, Function, Object as JsObject};
use santa_lang::{Arguments, Evaluation, ExpressionKind, ExternalFnDef, Location, Object, RuntimeErr};
use std::rc::Rc;
use wasm_bindgen::prelude::JsValue;

pub fn definitions(external_function_defs: &JsObject) -> Vec<ExternalFnDef> {
    js_sys::Object::entries(external_function_defs)
        .iter()
        .map(|value: JsValue| {
            let definition = Array::from(&value);
            (
                definition.get(0).as_string().unwrap(),
                vec![ExpressionKind::RestIdentifier("arguments".to_owned())],
                Rc::new(move |arguments: Arguments, source: Location| -> Evaluation {
                    let argument = to_js_value(arguments.get("arguments").unwrap(), source)?;
                    if let Ok(result) =
                        Function::from(definition.get(1)).apply(&JsValue::null(), &Array::from(&argument))
                    {
                        Ok(Rc::new(to_object(&result, source)?))
                    } else {
                        Err(RuntimeErr {
                            message: "Failed to execute external function".to_owned(),
                            source,
                            trace: vec![],
                        })
                    }
                }) as Rc<dyn Fn(Arguments, Location) -> Evaluation>,
            )
        })
        .collect()
}

fn to_js_value(value: &Object, source: Location) -> Result<JsValue, RuntimeErr> {
    match value {
        Object::Nil => Ok(JsValue::NULL),
        Object::Integer(v) => Ok(JsValue::from(*v)),
        Object::Boolean(v) => Ok(JsValue::from(*v)),
        Object::String(v) => Ok(JsValue::from(v)),
        Object::List(v) => {
            let array = Array::new();
            for element in v.iter() {
                array.push(&to_js_value(element, source)?);
            }
            Ok(JsValue::from(array))
        }
        _ => Err(RuntimeErr {
            message: format!(
                "Unable to translate santa-lang {} into JavaScript equivalent",
                value.name()
            ),
            source,
            trace: vec![],
        }),
    }
}

fn to_object(value: &JsValue, source: Location) -> Result<Object, RuntimeErr> {
    if value.is_null() || value.is_undefined() {
        return Ok(Object::Nil);
    }

    if let Some(v) = value.as_f64() {
        return Ok(Object::Integer(v as i64));
    }

    if let Some(v) = value.as_bool() {
        return Ok(Object::Boolean(v));
    }

    if let Some(v) = value.as_string() {
        return Ok(Object::String(v));
    }

    Err(RuntimeErr {
        message: format!(
            "Unable to translate JavaScript {} into santa-lang equivalent",
            value.js_typeof().as_string().unwrap()
        ),
        source,
        trace: vec![],
    })
}
