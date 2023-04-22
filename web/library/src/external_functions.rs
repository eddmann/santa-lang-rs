use js_sys::{Array, Function};
use santa_lang::{Arguments, Evaluation, ExpressionKind, ExternalFnDef, Location, Object};
use std::rc::Rc;
use wasm_bindgen::prelude::JsValue;

pub fn definitions(puts: Function, read: Function) -> Vec<ExternalFnDef> {
    vec![
        (
            "puts".to_owned(),
            vec![ExpressionKind::RestIdentifier("values".to_owned())],
            Rc::new(move |arguments: Arguments, _source: Location| -> Evaluation {
                if let Object::List(values) = &**arguments.get("values").unwrap() {
                    puts.call1(
                        &JsValue::null(),
                        &values
                            .iter()
                            .map(|value| JsValue::from_str(&value.to_string()))
                            .collect::<Array>(),
                    )
                    .unwrap();
                }
                Ok(Rc::new(Object::Nil))
            }),
        ),
        (
            "read".to_owned(),
            vec![ExpressionKind::Identifier("path".to_owned())],
            Rc::new(move |arguments: Arguments, _source: Location| -> Evaluation {
                if let Object::String(path) = &**arguments.get("path").unwrap() {
                    let result = read.call1(&JsValue::null(), &JsValue::from_str(path)).unwrap();
                    return Ok(Rc::new(Object::String(result.as_string().unwrap())));
                }
                Ok(Rc::new(Object::Nil))
            }),
        ),
    ]
}
