/// JSON functions: native_json_parse, native_json_stringify, native_json_stringify_pretty
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::*;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    functions.insert(
        "native_json_parse".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }

                let json_str = match &args[0] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Null(NullValue::new()),
                };

                match serde_json::from_str::<JsonValue>(&json_str) {
                    Ok(json) => convert_serde_to_runtime(&json),
                    Err(_) => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ),
    );

    functions.insert(
        "native_json_stringify".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Str(StrValue::new("null".to_string()));
                }

                let json = convert_runtime_to_serde(&args[0]);
                match serde_json::to_string(&json) {
                    Ok(s) => RuntimeValue::Str(StrValue::new(s)),
                    Err(_) => RuntimeValue::Str(StrValue::new("null".to_string())),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::any()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    functions.insert(
        "native_json_stringify_pretty".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Str(StrValue::new("null".to_string()));
                }

                let json = convert_runtime_to_serde(&args[0]);
                match serde_json::to_string_pretty(&json) {
                    Ok(s) => RuntimeValue::Str(StrValue::new(s)),
                    Err(_) => RuntimeValue::Str(StrValue::new("null".to_string())),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::any(), PrimitiveType::int()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
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
            RuntimeValue::List(ListValue::new(elements, PrimitiveType::any()))
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
        RuntimeValue::List(list) => {
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
