mod external_functions;

use js_sys::{Array, Object};
use santa_lang::{run, AoCRunner, Environment, Time};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[cfg(test)]
mod tests;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = this, js_name = performance)]
    pub static JS_PERFORMANCE: web_sys::Performance;
}

struct WebTime {}
impl Time for WebTime {
    fn now(&self) -> u128 {
        JS_PERFORMANCE.now() as u128
    }
}

#[wasm_bindgen]
pub fn aoc_run(source: &str, js_functions: Object) -> Result<JsValue, JsValue> {
    let mut runner =
        AoCRunner::new_with_external_functions(WebTime {}, &crate::external_functions::definitions(&js_functions));

    match runner.run(source) {
        Ok(result) => Ok(serde_wasm_bindgen::to_value(&result).unwrap()),
        Err(error) => Err(serde_wasm_bindgen::to_value(&error).unwrap()),
    }
}

#[wasm_bindgen]
pub fn aoc_test(source: &str, js_functions: Object) -> Result<JsValue, JsValue> {
    let mut runner =
        AoCRunner::new_with_external_functions(WebTime {}, &crate::external_functions::definitions(&js_functions));

    match runner.test(source) {
        Ok(test_cases) => Ok(JsValue::from(
            test_cases
                .iter()
                .map(|test_case| serde_wasm_bindgen::to_value(test_case).unwrap())
                .collect::<Array>(),
        )),
        Err(error) => Err(serde_wasm_bindgen::to_value(&error).unwrap()),
    }
}

#[wasm_bindgen]
pub fn evaluate(expression: &str, js_functions: Option<Object>) -> Result<JsValue, JsValue> {
    let enviornment = Environment::new();
    let external_functions = if let Some(js_functions) = js_functions {
        crate::external_functions::definitions(&js_functions)
    } else {
        vec![]
    };

    match run(expression, enviornment, &external_functions) {
        Ok(evaluated) => Ok(JsValue::from(evaluated.to_string())),
        Err(error) => Err(serde_wasm_bindgen::to_value(&error).unwrap()),
    }
}
