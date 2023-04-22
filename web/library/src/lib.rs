mod external_functions;

use js_sys::{Array, Function, Object, Reflect};
use santa_lang::{RunErr, RunEvaluation, Runner, Time};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self, js_name = performance)]
    pub static JS_PERFORMANCE: web_sys::Performance;
}

struct WebTime {}
impl Time for WebTime {
    fn now(&self) -> u128 {
        JS_PERFORMANCE.now() as u128
    }
}

#[wasm_bindgen]
pub fn run(source: &str, puts: Function, read: Function) -> Result<JsValue, JsValue> {
    let mut runner =
        Runner::new_with_external_functions(WebTime {}, &crate::external_functions::definitions(puts, read));

    match runner.run(source) {
        Ok(RunEvaluation::Script(result)) => {
            let object = Object::new();

            Reflect::set(&object, &"value".into(), &result.value.into()).unwrap();
            Reflect::set(&object, &"duration".into(), &result.duration.into()).unwrap();

            Ok(object.into())
        }
        Ok(RunEvaluation::Solution { part_one, part_two }) => {
            let object = Object::new();

            if let Some(part_one) = part_one {
                let part = Object::new();
                Reflect::set(&part, &"value".into(), &part_one.value.into()).unwrap();
                Reflect::set(&part, &"duration".into(), &part_one.duration.into()).unwrap();
                Reflect::set(&object, &"part_one".into(), &part.into()).unwrap();
            }

            if let Some(part_two) = part_two {
                let part = Object::new();
                Reflect::set(&part, &"value".into(), &part_two.value.into()).unwrap();
                Reflect::set(&part, &"duration".into(), &part_two.duration.into()).unwrap();
                Reflect::set(&object, &"part_two".into(), &part.into()).unwrap();
            }

            Ok(object.into())
        }
        Err(error) => Err(to_error_object(error)),
    }
}

#[wasm_bindgen]
pub fn test(source: &str, puts: Function, read: Function) -> Result<JsValue, JsValue> {
    let mut runner =
        Runner::new_with_external_functions(WebTime {}, &crate::external_functions::definitions(puts, read));

    match runner.test(source) {
        Ok(test_cases) => Ok(JsValue::from(
            test_cases
                .iter()
                .map(|test_case| {
                    let object = Object::new();

                    if let Some(part_one) = &test_case.part_one {
                        let part = Object::new();
                        Reflect::set(&part, &"expected".into(), &part_one.expected.to_owned().into()).unwrap();
                        Reflect::set(&part, &"actual".into(), &part_one.actual.to_owned().into()).unwrap();
                        Reflect::set(&part, &"passed".into(), &part_one.passed.into()).unwrap();
                        Reflect::set(&object, &"part_one".into(), &part.into()).unwrap();
                    }

                    if let Some(part_two) = &test_case.part_two {
                        let part = Object::new();
                        Reflect::set(&part, &"expected".into(), &part_two.expected.to_owned().into()).unwrap();
                        Reflect::set(&part, &"actual".into(), &part_two.actual.to_owned().into()).unwrap();
                        Reflect::set(&part, &"passed".into(), &part_two.passed.into()).unwrap();
                        Reflect::set(&object, &"part_two".into(), &part.into()).unwrap();
                    }

                    JsValue::from(object)
                })
                .collect::<Array>(),
        )),
        Err(error) => Err(to_error_object(error)),
    }
}

fn to_error_object(error: RunErr) -> JsValue {
    let object = Object::new();

    Reflect::set(&object, &"message".into(), &error.message.into()).unwrap();

    let source = Object::new();
    Reflect::set(&source, &"start".into(), &error.source.start.into()).unwrap();
    Reflect::set(&source, &"end".into(), &error.source.end.into()).unwrap();
    Reflect::set(&object, &"source".into(), &source.into()).unwrap();

    JsValue::from(object)
}
