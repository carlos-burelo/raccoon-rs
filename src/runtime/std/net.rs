// use crate::ast::types::{PrimitiveType, Type};
// use crate::runtime::values::{BoolValue, IntValue, MapValue, NullValue, RuntimeValue, StrValue};
// use isahc::{HttpClient, Request, prelude::*};
// use std::collections::HashMap;

// pub struct NetModule;

// impl NetModule {
//     pub fn name() -> &'static str {
//         "std:net"
//     }

//     pub fn get_export() -> HashMap<String, RuntimeValue> {
//         let mut exports = HashMap::new();
//         exports.insert(
//             "get".to_string(),
//             RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
//                 fetch_get,
//                 Type::Function(Box::new(crate::ast::types::FunctionType {
//                     params: vec![PrimitiveType::str()],
//                     return_type: PrimitiveType::any(),
//                     is_variadic: true,
//                 })),
//             )),
//         );
//         exports.insert(
//             "post".to_string(),
//             RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
//                 fetch_post,
//                 Type::Function(Box::new(crate::ast::types::FunctionType {
//                     params: vec![PrimitiveType::str()],
//                     return_type: PrimitiveType::any(),
//                     is_variadic: true,
//                 })),
//             )),
//         );
//         exports
//     }

//     pub fn parse_headers(headers_value: &RuntimeValue) -> HashMap<String, String> {
//         let mut headers = HashMap::new();
//         if let RuntimeValue::Map(map) = headers_value {
//             for (k, v) in &map.entries {
//                 if let RuntimeValue::Str(s) = v {
//                     headers.insert(k.clone(), s.value.clone());
//                 }
//             }
//         }
//         headers
//     }

//     pub fn value_to_string(value: &RuntimeValue) -> String {
//         match value {
//             RuntimeValue::Str(s) => s.value.clone(),
//             RuntimeValue::Int(i) => i.value.to_string(),
//             RuntimeValue::Float(f) => f.value.to_string(),
//             RuntimeValue::Bool(b) => b.value.to_string(),
//             RuntimeValue::Map(_) | RuntimeValue::List(_) => Self::serialize_as_json(value),
//             _ => value.to_string(),
//         }
//     }

//     pub fn serialize_as_json(value: &RuntimeValue) -> String {
//         match value {
//             RuntimeValue::Null(_) => "null".to_string(),
//             RuntimeValue::Bool(b) => b.value.to_string(),
//             RuntimeValue::Int(i) => i.value.to_string(),
//             RuntimeValue::Float(f) => f.value.to_string(),
//             RuntimeValue::Str(s) => {
//                 format!("\"{}\"", s.value.replace('\\', "\\\\").replace('"', "\\\""))
//             }
//             RuntimeValue::List(list) => {
//                 let elems: Vec<String> = list
//                     .elements
//                     .iter()
//                     .map(|v| Self::serialize_as_json(v))
//                     .collect();
//                 format!("[{}]", elems.join(","))
//             }
//             RuntimeValue::Map(map) => {
//                 let mut pairs = Vec::new();
//                 for (k, v) in &map.entries {
//                     pairs.push(format!("\"{}\":{}", k, Self::serialize_as_json(v)));
//                 }
//                 format!("{{{}}}", pairs.join(","))
//             }
//             _ => "null".to_string(),
//         }
//     }

//     pub fn parse_json(s: &str) -> Result<RuntimeValue, String> {
//         let trimmed = s.trim();
//         if trimmed == "null" {
//             return Ok(RuntimeValue::Null(NullValue::new()));
//         }
//         if trimmed == "true" {
//             return Ok(RuntimeValue::Bool(BoolValue::new(true)));
//         }
//         if trimmed == "false" {
//             return Ok(RuntimeValue::Bool(BoolValue::new(false)));
//         }
//         if let Ok(num) = trimmed.parse::<i64>() {
//             return Ok(RuntimeValue::Int(IntValue::new(num)));
//         }
//         if let Ok(num) = trimmed.parse::<f64>() {
//             return Ok(RuntimeValue::Float(crate::runtime::FloatValue::new(num)));
//         }
//         if trimmed.starts_with('"') && trimmed.ends_with('"') {
//             let content = &trimmed[1..trimmed.len() - 1];
//             let unescaped = content
//                 .replace("\\\"", "\"")
//                 .replace("\\\\", "\\")
//                 .replace("\\n", "\n");
//             return Ok(RuntimeValue::Str(StrValue::new(unescaped)));
//         }
//         Ok(RuntimeValue::Str(StrValue::new(trimmed.to_string())))
//     }

//     pub fn http_request(
//         url: &str,
//         method: &str,
//         headers: HashMap<String, String>,
//         body: Option<String>,
//     ) -> RuntimeValue {
//         let client = HttpClient::new().unwrap();
//         let mut req = match method.to_uppercase().as_str() {
//             "GET" => Request::get(url).body(()).unwrap(),
//             "POST" => Request::post(url)
//                 .body(body.clone().unwrap_or_default())
//                 .unwrap(),
//             "PUT" => Request::put(url)
//                 .body(body.clone().unwrap_or_default())
//                 .unwrap(),
//             "DELETE" => Request::delete(url).body(()).unwrap(),
//             "PATCH" => Request::patch(url)
//                 .body(body.clone().unwrap_or_default())
//                 .unwrap(),
//             "HEAD" => Request::head(url).body(()).unwrap(),
//             _ => Request::get(url).body(()).unwrap(),
//         };

//         for (k, v) in headers {
//             req = req.header(&k, &v).unwrap();
//         }

//         let resp_result = client.send(req);

//         match resp_result {
//             Ok(resp) => {
//                 let status = resp.status().as_u16() as i64;
//                 let mut result = HashMap::new();
//                 result.insert(
//                     "status".to_string(),
//                     RuntimeValue::Int(IntValue::new(status)),
//                 );
//                 result.insert(
//                     "ok".to_string(),
//                     RuntimeValue::Bool(BoolValue::new(status >= 200 && status < 300)),
//                 );
//                 result.insert(
//                     "statusText".to_string(),
//                     RuntimeValue::Str(StrValue::new("".to_string())),
//                 );

//                 let mut headers_map = HashMap::new();
//                 for (name, value) in resp.headers() {
//                     headers_map.insert(
//                         name.to_string(),
//                         RuntimeValue::Str(StrValue::new(value.to_str().unwrap_or("").to_string())),
//                     );
//                 }
//                 result.insert(
//                     "headers".to_string(),
//                     RuntimeValue::Map(MapValue::new(
//                         headers_map,
//                         PrimitiveType::str(),
//                         PrimitiveType::str(),
//                     )),
//                 );

//                 let body_text = resp.text().unwrap_or_default();
//                 result.insert(
//                     "text".to_string(),
//                     RuntimeValue::Str(StrValue::new(body_text.clone())),
//                 );
//                 if let Ok(json_val) = Self::parse_json(&body_text) {
//                     result.insert("json".to_string(), json_val);
//                 } else {
//                     result.insert("json".to_string(), RuntimeValue::Null(NullValue::new()));
//                 }

//                 RuntimeValue::Map(MapValue::new(
//                     result,
//                     PrimitiveType::str(),
//                     PrimitiveType::any(),
//                 ))
//             }
//             Err(e) => {
//                 let mut err_map = HashMap::new();
//                 err_map.insert("ok".to_string(), RuntimeValue::Bool(BoolValue::new(false)));
//                 err_map.insert(
//                     "error".to_string(),
//                     RuntimeValue::Str(StrValue::new(e.to_string())),
//                 );
//                 err_map.insert("status".to_string(), RuntimeValue::Int(IntValue::new(0)));
//                 RuntimeValue::Map(MapValue::new(
//                     err_map,
//                     PrimitiveType::str(),
//                     PrimitiveType::any(),
//                 ))
//             }
//         }
//     }
// }

// // Free functions

// fn fetch_get(args: Vec<RuntimeValue>) -> RuntimeValue {
//     if args.is_empty() {
//         return RuntimeValue::Null(NullValue::new());
//     }
//     let url = match &args[0] {
//         RuntimeValue::Str(s) => s.value.clone(),
//         _ => return RuntimeValue::Null(NullValue::new()),
//     };
//     let headers = if args.len() > 1 {
//         NetModule::parse_headers(&args[1])
//     } else {
//         HashMap::new()
//     };
//     NetModule::http_request(&url, "GET", headers, None)
// }

// fn fetch_post(args: Vec<RuntimeValue>) -> RuntimeValue {
//     if args.is_empty() {
//         return RuntimeValue::Null(NullValue::new());
//     }
//     let url = match &args[0] {
//         RuntimeValue::Str(s) => s.value.clone(),
//         _ => return RuntimeValue::Null(NullValue::new()),
//     };
//     let body = if args.len() > 1 {
//         Some(NetModule::value_to_string(&args[1]))
//     } else {
//         None
//     };
//     let headers = if args.len() > 2 {
//         NetModule::parse_headers(&args[2])
//     } else {
//         HashMap::new()
//     };
//     NetModule::http_request(&url, "POST", headers, body)
// }
