use js_sys::{Array as JsArray, BigInt, Map as JsMap, Object as JsObject, Set as JsSet};
use santa_lang::{Location, Object, RuntimeErr};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use wasm_bindgen::prelude::JsValue;

pub fn to_js_value(value: &Object, source: Location) -> Result<JsValue, RuntimeErr> {
    match value {
        Object::Nil => Ok(JsValue::NULL),
        Object::Integer(v) => Ok(JsValue::from(*v)),
        Object::Decimal(v) => Ok(JsValue::from(**v)),
        Object::Boolean(v) => Ok(JsValue::from(*v)),
        Object::String(v) => Ok(JsValue::from(v)),
        Object::List(v) => {
            let array = JsArray::new();
            for element in v.iter() {
                array.push(&to_js_value(element, source)?);
            }
            Ok(JsValue::from(array))
        }
        Object::Set(v) => {
            let set = JsSet::new(&JsValue::UNDEFINED);
            for element in v.iter() {
                set.add(&to_js_value(element, source)?);
            }
            Ok(JsValue::from(set))
        }
        Object::Hash(v) => {
            let map = JsMap::new();
            for (key, value) in v.iter() {
                map.set(&to_js_value(key, source)?, &to_js_value(value, source)?);
            }
            Ok(JsValue::from(map))
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

pub fn to_santa_object(value: &JsValue, source: Location) -> Result<Object, RuntimeErr> {
    if value.is_null() || value.is_undefined() {
        return Ok(Object::Nil);
    }

    if value.is_bigint() {
        return Ok(Object::Integer(i64::try_from(BigInt::from(value.clone())).unwrap()));
    }

    if let Some(v) = value.as_f64() {
        return Ok(Object::Decimal(v.into()));
    }

    if let Some(v) = value.as_bool() {
        return Ok(Object::Boolean(v));
    }

    if let Some(v) = value.as_string() {
        return Ok(Object::String(v));
    }

    if value.is_array() {
        let mut list = Vec::new();
        for element in JsArray::from(value).iter() {
            list.push(Rc::new(to_santa_object(&element, source)?));
        }
        return Ok(Object::List(list.into()));
    }

    if value.is_object() && JsObject::from(value.clone()).constructor().name() == "Set" {
        let mut set = HashSet::new();
        for element in JsSet::from(value.clone()).values() {
            if let Ok(element) = element {
                set.insert(Rc::new(to_santa_object(&element, source)?));
            } else {
                return Err(RuntimeErr {
                    message: "Failed to translate JavaScript Set element into santa-lang equivalent".to_owned(),
                    source,
                    trace: vec![],
                });
            }
        }
        return Ok(Object::Set(set.into()));
    }

    if value.is_object() && JsObject::from(value.clone()).constructor().name() == "Map" {
        let mut map = HashMap::new();
        for element in JsMap::from(value.clone()).entries() {
            if let Ok(element) = element {
                let entry = JsArray::from(&element);
                map.insert(
                    Rc::new(to_santa_object(&entry.get(0), source)?),
                    Rc::new(to_santa_object(&entry.get(1), source)?),
                );
            } else {
                return Err(RuntimeErr {
                    message: "Failed to translate JavaScript Map entry into santa-lang equivalent".to_owned(),
                    source,
                    trace: vec![],
                });
            }
        }
        return Ok(Object::Hash(map.into()));
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
