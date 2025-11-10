use crate::ast::types::PrimitiveType;
use crate::runtime::{
    ArrayValue, BoolValue, FloatValue, FromRaccoon, IntValue, NullValue, ObjectValue, Registrar,
    RuntimeValue, StrValue, ToRaccoon,
};
use serde_json::Value as JsonValue;

pub fn register_json_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "parse",
        Some("json"),
        |args| {
            let json_str = String::from_raccoon(&args[0]).unwrap_or_default();
            match serde_json::from_str::<JsonValue>(&json_str) {
                Ok(json) => convert_serde_to_runtime(&json),
                Err(_) => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "stringify",
        Some("json"),
        |args| {
            let json = convert_runtime_to_serde(&args[0]);
            match serde_json::to_string(&json) {
                Ok(s) => s.to_raccoon(),
                Err(_) => "null".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "stringify_pretty",
        Some("json"),
        |args| {
            let json = convert_runtime_to_serde(&args[0]);
            match serde_json::to_string_pretty(&json) {
                Ok(s) => s.to_raccoon(),
                Err(_) => "null".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );
}

fn convert_serde_to_runtime(value: &JsonValue) -> RuntimeValue {
    match value {
        JsonValue::Null => RuntimeValue::Null(NullValue::new()),
        JsonValue::Bool(b) => RuntimeValue::Bool(BoolValue::new(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                RuntimeValue::Int(IntValue::new(i))
            } else if let Some(f) = n.as_f64() {
                RuntimeValue::Float(FloatValue::new(f))
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        }
        JsonValue::String(s) => RuntimeValue::Str(StrValue::new(s.clone())),
        JsonValue::Array(arr) => {
            let elements = arr.iter().map(convert_serde_to_runtime).collect();
            RuntimeValue::Array(ArrayValue::new(elements, PrimitiveType::any()))
        }
        JsonValue::Object(obj) => {
            let properties = obj
                .iter()
                .map(|(k, v)| (k.clone(), convert_serde_to_runtime(v)))
                .collect();
            RuntimeValue::Object(ObjectValue::new(properties, PrimitiveType::any()))
        }
    }
}

fn convert_runtime_to_serde(value: &RuntimeValue) -> JsonValue {
    match value {
        RuntimeValue::Null(_) => JsonValue::Null,
        RuntimeValue::Bool(b) => JsonValue::Bool(b.value),
        RuntimeValue::Int(i) => JsonValue::Number(serde_json::Number::from(i.value)),
        RuntimeValue::Float(f) => serde_json::Number::from_f64(f.value)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        RuntimeValue::Str(s) => JsonValue::String(s.value.clone()),
        RuntimeValue::Array(list) => {
            let arr = list.elements.iter().map(convert_runtime_to_serde).collect();
            JsonValue::Array(arr)
        }
        RuntimeValue::Object(obj) => {
            let map = obj
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), convert_runtime_to_serde(v)))
                .collect();
            JsonValue::Object(map)
        }
        RuntimeValue::Map(map) => {
            let obj = map
                .entries
                .iter()
                .map(|(k, v)| (k.clone(), convert_runtime_to_serde(v)))
                .collect();
            JsonValue::Object(obj)
        }
        _ => JsonValue::Null,
    }
}
