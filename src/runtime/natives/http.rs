use crate::ast::types::{FunctionType, MapType, ObjectType, PrimitiveType, Type};
use crate::runtime::values::{
    IntValue, MapValue, NativeAsyncFunctionValue, ObjectValue, RuntimeValue, StrValue,
};
use std::collections::HashMap;
use std::sync::Arc;

pub fn register_async(async_functions: &mut HashMap<String, NativeAsyncFunctionValue>) {
    async_functions.insert(
        "native_http_fetch".to_string(),
        NativeAsyncFunctionValue::new(
            Arc::new(|args: Vec<RuntimeValue>| {
                Box::pin(async move {
                    if let Some(RuntimeValue::Str(url)) = args.first() {
                        http_fetch_impl(&url.value, None).await
                    } else {
                        create_error_response("Invalid URL argument")
                    }
                })
            }),
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: create_http_response_type(),
                is_variadic: false,
            })),
        ),
    );

    async_functions.insert(
        "native_http_fetch_with_options".to_string(),
        NativeAsyncFunctionValue::new(
            Arc::new(|args: Vec<RuntimeValue>| {
                Box::pin(async move {
                    if args.len() < 2 {
                        return create_error_response("Missing URL or options");
                    }

                    let url = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return create_error_response("Invalid URL"),
                    };

                    let options = match &args[1] {
                        RuntimeValue::Object(obj) => Some(obj.clone()),
                        _ => None,
                    };

                    http_fetch_impl(&url, options).await
                })
            }),
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), create_request_options_type()],
                return_type: create_http_response_type(),
                is_variadic: false,
            })),
        ),
    );
}

fn create_http_response_type() -> Type {
    let mut properties = HashMap::new();

    properties.insert(
        "status".to_string(),
        crate::ast::types::ObjectProperty::new(PrimitiveType::int()),
    );
    properties.insert(
        "statusText".to_string(),
        crate::ast::types::ObjectProperty::new(PrimitiveType::str()),
    );
    properties.insert(
        "headers".to_string(),
        crate::ast::types::ObjectProperty::new(Type::Map(Box::new(MapType {
            key_type: PrimitiveType::str(),
            value_type: PrimitiveType::str(),
        }))),
    );
    properties.insert(
        "body".to_string(),
        crate::ast::types::ObjectProperty::new(PrimitiveType::str()),
    );

    let mut json_prop = crate::ast::types::ObjectProperty::new(PrimitiveType::any());
    json_prop.optional = true;
    properties.insert("json".to_string(), json_prop);

    Type::Object(Box::new(ObjectType { properties }))
}

fn create_request_options_type() -> Type {
    let mut properties = HashMap::new();

    let mut method_prop = crate::ast::types::ObjectProperty::new(PrimitiveType::str());
    method_prop.optional = true;
    properties.insert("method".to_string(), method_prop);

    let mut headers_prop = crate::ast::types::ObjectProperty::new(Type::Map(Box::new(MapType {
        key_type: PrimitiveType::str(),
        value_type: PrimitiveType::str(),
    })));
    headers_prop.optional = true;
    properties.insert("headers".to_string(), headers_prop);

    let mut body_prop = crate::ast::types::ObjectProperty::new(PrimitiveType::str());
    body_prop.optional = true;
    properties.insert("body".to_string(), body_prop);

    let mut timeout_prop = crate::ast::types::ObjectProperty::new(PrimitiveType::int());
    timeout_prop.optional = true;
    properties.insert("timeout".to_string(), timeout_prop);

    Type::Object(Box::new(ObjectType { properties }))
}

struct HttpOptions {
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout: u64,
}

impl HttpOptions {
    fn from_object(obj: &ObjectValue) -> Self {
        let method = match obj.properties.get("method") {
            Some(RuntimeValue::Str(s)) => s.value.clone(),
            _ => "GET".to_string(),
        };

        let mut headers = HashMap::new();
        if let Some(RuntimeValue::Map(m)) = obj.properties.get("headers") {
            for (k, v) in &m.entries {
                if let RuntimeValue::Str(s) = v {
                    headers.insert(k.clone(), s.value.clone());
                }
            }
        }

        let body = match obj.properties.get("body") {
            Some(RuntimeValue::Str(s)) => Some(s.value.clone()),
            _ => None,
        };

        let timeout = match obj.properties.get("timeout") {
            Some(RuntimeValue::Int(i)) => i.value as u64,
            _ => 30000,
        };

        HttpOptions {
            method: method.to_uppercase(),
            headers,
            body,
            timeout,
        }
    }
}

async fn http_fetch_impl(url: &str, options: Option<ObjectValue>) -> RuntimeValue {
    let opts = match options {
        Some(obj) => HttpOptions::from_object(&obj),
        None => HttpOptions {
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            timeout: 30000,
        },
    };

    match perform_http_request(url, &opts).await {
        Ok(response_obj) => response_obj,
        Err(e) => create_error_response(&e),
    }
}

async fn perform_http_request(url: &str, opts: &HttpOptions) -> Result<RuntimeValue, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(opts.timeout))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    let request = match opts.method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, url),
        _ => return Err(format!("Unsupported HTTP method: {}", opts.method)),
    };

    let mut request = request;
    for (key, value) in &opts.headers {
        request = request.header(key.as_str(), value.as_str());
    }

    if let Some(body) = &opts.body {
        request = request.body(body.clone());
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status().as_u16() as i64;
    let status_text = response.status().canonical_reason().unwrap_or("Unknown");

    let mut headers_map = HashMap::new();
    for (name, value) in response.headers().iter() {
        let header_name = name.to_string();
        let header_value = value.to_str().unwrap_or("").to_string();
        headers_map.insert(header_name, RuntimeValue::Str(StrValue::new(header_value)));
    }

    let body_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let json_value = try_parse_json(&body_text);

    let mut response_props = HashMap::new();
    response_props.insert(
        "status".to_string(),
        RuntimeValue::Int(IntValue::new(status)),
    );
    response_props.insert(
        "statusText".to_string(),
        RuntimeValue::Str(StrValue::new(status_text.to_string())),
    );
    response_props.insert(
        "headers".to_string(),
        RuntimeValue::Map(MapValue::new(
            headers_map,
            PrimitiveType::str(),
            PrimitiveType::str(),
        )),
    );
    response_props.insert(
        "body".to_string(),
        RuntimeValue::Str(StrValue::new(body_text)),
    );

    if let Some(json_val) = json_value {
        response_props.insert("json".to_string(), json_val);
    }

    Ok(RuntimeValue::Object(ObjectValue::new(
        response_props,
        create_http_response_type(),
    )))
}

fn try_parse_json(body: &str) -> Option<RuntimeValue> {
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json_val) => Some(json_to_runtime_value(&json_val)),
        Err(_) => None,
    }
}

fn json_to_runtime_value(json: &serde_json::Value) -> RuntimeValue {
    match json {
        serde_json::Value::Null => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
        serde_json::Value::Bool(b) => {
            RuntimeValue::Bool(crate::runtime::values::BoolValue::new(*b))
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                RuntimeValue::Int(IntValue::new(i))
            } else if let Some(f) = n.as_f64() {
                RuntimeValue::Float(crate::runtime::values::FloatValue::new(f))
            } else {
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            }
        }
        serde_json::Value::String(s) => RuntimeValue::Str(StrValue::new(s.clone())),
        serde_json::Value::Array(arr) => {
            let elements: Vec<RuntimeValue> = arr.iter().map(json_to_runtime_value).collect();

            if elements.is_empty() {
                RuntimeValue::List(crate::runtime::values::ListValue::new(
                    elements,
                    PrimitiveType::any(),
                ))
            } else {
                let element_type = elements[0].get_type();
                RuntimeValue::List(crate::runtime::values::ListValue::new(
                    elements,
                    element_type,
                ))
            }
        }
        serde_json::Value::Object(obj) => {
            let mut props = HashMap::new();
            for (k, v) in obj {
                props.insert(k.clone(), json_to_runtime_value(v));
            }
            RuntimeValue::Object(ObjectValue::new(props, create_generic_object_type()))
        }
    }
}

fn create_generic_object_type() -> Type {
    Type::Object(Box::new(ObjectType {
        properties: HashMap::new(),
    }))
}

fn create_error_response(message: &str) -> RuntimeValue {
    let mut props = HashMap::new();
    props.insert("status".to_string(), RuntimeValue::Int(IntValue::new(0)));
    props.insert(
        "statusText".to_string(),
        RuntimeValue::Str(StrValue::new("Error".to_string())),
    );
    props.insert(
        "headers".to_string(),
        RuntimeValue::Map(MapValue::new(
            HashMap::new(),
            PrimitiveType::str(),
            PrimitiveType::str(),
        )),
    );
    props.insert(
        "body".to_string(),
        RuntimeValue::Str(StrValue::new(message.to_string())),
    );

    RuntimeValue::Object(ObjectValue::new(props, create_http_response_type()))
}
