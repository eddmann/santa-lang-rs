use js_sys::{Array as JsArray, Function, Object as JsObject};
use santa_lang::{Arguments, Evaluation, ExpressionKind, ExternalFnDef, Location, RuntimeErr};
use std::rc::Rc;
use wasm_bindgen::prelude::JsValue;

pub fn definitions(js_functions: &JsObject) -> Vec<ExternalFnDef> {
    JsObject::entries(js_functions)
        .iter()
        .map(|value: JsValue| {
            let definition = JsArray::from(&value);
            (
                definition.get(0).as_string().expect("External function name should be a string"),
                vec![ExpressionKind::RestIdentifier("arguments".to_owned())],
                Rc::new(move |arguments: Arguments, source: Location| -> Evaluation {
                    let args = arguments.get("arguments").expect("Arguments parameter guaranteed by function signature");
                    let argument = serde_wasm_bindgen::to_value(args)
                        .expect("Failed to serialize arguments to JsValue");
                    if let Ok(result) =
                        Function::from(definition.get(1)).apply(&JsValue::null(), &JsArray::from(&argument))
                    {
                        Ok(Rc::new(serde_wasm_bindgen::from_value(result)
                            .expect("Failed to deserialize result from JsValue")))
                    } else {
                        Err(RuntimeErr {
                            message: "Failed to execute external JavaScript function".to_owned(),
                            source,
                            trace: vec![],
                        })
                    }
                }) as Rc<dyn Fn(Arguments, Location) -> Evaluation>,
            )
        })
        .collect()
}
