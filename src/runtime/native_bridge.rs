use crate::ast::types::{FunctionType, FutureType, PrimitiveType, Type};
use crate::runtime::values::{
    BoolValue, FloatValue, IntValue, ListValue, MapValue, NativeAsyncFunctionValue,
    NativeFunctionValue, NullValue, ObjectValue, RuntimeValue, StrValue,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NativeBridge {
    functions: HashMap<String, NativeFunctionValue>,
    async_functions: HashMap<String, NativeAsyncFunctionValue>,
}

impl NativeBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            functions: HashMap::new(),
            async_functions: HashMap::new(),
        };
        bridge.register_all_natives();
        bridge
    }

    fn register_all_natives(&mut self) {
        self.register_io_primitives();
        self.register_time_primitives();
        self.register_random_primitives();
        self.register_json_primitives();
        self.register_http_primitives();
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        self.functions
            .get(name)
            .map(|f| RuntimeValue::NativeFunction(f.clone()))
    }

    pub fn get_async(&self, name: &str) -> Option<RuntimeValue> {
        self.async_functions
            .get(name)
            .map(|f| RuntimeValue::NativeAsyncFunction(f.clone()))
    }

    fn register_io_primitives(&mut self) {
        self.functions.insert(
            "native_print".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    let output = args
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", output);
                    RuntimeValue::Null(NullValue::new())
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::void(),
                    is_variadic: true,
                })),
            ),
        );

        self.functions.insert(
            "native_eprint".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    let output = args
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    eprintln!("{}", output);
                    RuntimeValue::Null(NullValue::new())
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::void(),
                    is_variadic: true,
                })),
            ),
        );
    }

    fn register_time_primitives(&mut self) {}

    fn register_random_primitives(&mut self) {
        self.functions.insert(
            "native_random".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();
                    let nanos = now.as_nanos() as u128;
                    let val = (nanos as f64) / 1_000_000_000.0;
                    RuntimeValue::Float(FloatValue::new(val))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_json_primitives(&mut self) {}

    fn register_http_primitives(&mut self) {
        let client = reqwest::Client::new();

        self.async_functions.insert(
            //
            "native_http_request".to_string(),
            NativeAsyncFunctionValue::new(
                Arc::new(move |args: Vec<RuntimeValue>| {
                    let client = client.clone();

                    Box::pin(async move {
                        if args.len() < 2 {
                            return RuntimeValue::Null(NullValue::new());
                        }

                        let method_str = match &args[0] {
                            RuntimeValue::Str(s) => s.value.clone(),
                            _ => return RuntimeValue::Null(NullValue::new()),
                        };

                        let url = match &args[1] {
                            RuntimeValue::Str(s) => s.value.clone(),
                            _ => return RuntimeValue::Null(NullValue::new()),
                        };

                        let method = match method_str.as_str() {
                            "GET" => reqwest::Method::GET,
                            "POST" => reqwest::Method::POST,
                            "PUT" => reqwest::Method::PUT,
                            "DELETE" => reqwest::Method::DELETE,
                            _ => reqwest::Method::GET,
                        };

                        match client.request(method, url.clone()).send().await {
                            Ok(resp) => {
                                let status = resp.status().as_u16() as i64;
                                let status_text =
                                    resp.status().canonical_reason().unwrap_or("").to_string();

                                let mut headers_map_val = HashMap::new();
                                for (key, value) in resp.headers().iter() {
                                    if let Ok(val_str) = value.to_str() {
                                        headers_map_val.insert(
                                            key.to_string(),
                                            RuntimeValue::Str(StrValue::new(val_str.to_string())),
                                        );
                                    }
                                }

                                let body_val = match resp.json::<JsonValue>().await {
                                    Ok(json_body) => convert_serde_to_runtime(&json_body),
                                    Err(_) => RuntimeValue::Null(NullValue::new()),
                                };

                                let mut response_map = HashMap::new();
                                response_map.insert(
                                    "status".to_string(),
                                    RuntimeValue::Int(IntValue::new(status)),
                                );
                                response_map.insert(
                                    "statusText".to_string(),
                                    RuntimeValue::Str(StrValue::new(status_text)),
                                );
                                response_map.insert("body".to_string(), body_val);
                                response_map.insert(
                                    "url".to_string(),
                                    RuntimeValue::Str(StrValue::new(url)),
                                );
                                let headers_obj = RuntimeValue::Map(MapValue::new(
                                    headers_map_val,
                                    PrimitiveType::str(),
                                    PrimitiveType::str(),
                                ));
                                response_map.insert("headers".to_string(), headers_obj);

                                RuntimeValue::Object(ObjectValue::new(
                                    response_map,
                                    PrimitiveType::any(),
                                ))
                            }
                            Err(e) => {
                                let mut response_map = HashMap::new();
                                response_map.insert(
                                    "status".to_string(),
                                    RuntimeValue::Int(IntValue::new(500)),
                                );
                                response_map.insert(
                                    "statusText".to_string(),
                                    RuntimeValue::Str(StrValue::new(e.to_string())),
                                );
                                response_map.insert(
                                    "body".to_string(),
                                    RuntimeValue::Null(NullValue::new()),
                                );
                                response_map.insert(
                                    "url".to_string(),
                                    RuntimeValue::Str(StrValue::new(url)),
                                );
                                let headers_obj = RuntimeValue::Map(MapValue::new(
                                    HashMap::new(),
                                    PrimitiveType::str(),
                                    PrimitiveType::str(),
                                ));
                                response_map.insert("headers".to_string(), headers_obj);

                                RuntimeValue::Object(ObjectValue::new(
                                    response_map,
                                    PrimitiveType::any(),
                                ))
                            }
                        }
                    })
                }),
                Type::Function(Box::new(FunctionType {
                    //
                    params: vec![
                        PrimitiveType::str(),
                        PrimitiveType::str(),
                        PrimitiveType::any(),
                        PrimitiveType::any(),
                        PrimitiveType::int(),
                    ],
                    return_type: Type::Future(Box::new(FutureType {
                        inner_type: PrimitiveType::any(),
                    })),
                    is_variadic: false,
                })),
            ),
        );
    }
}

impl Default for NativeBridge {
    fn default() -> Self {
        Self::new()
    }
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
