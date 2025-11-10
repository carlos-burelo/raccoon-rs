use crate::ast::types::PrimitiveType;
use crate::runtime::{
    BoolValue, FromRaccoon, FutureValue, IntValue, MapValue, NullValue, ObjectValue, Registrar,
    RuntimeValue, StrValue, ToRaccoon,
};
use std::collections::HashMap;

pub fn register_http_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "native_http_fetch_with_options",
        None::<&str>,
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(NullValue::new());
            }

            let url = String::from_raccoon(&args[0]).unwrap_or_default();

            let method = if args.len() > 1 {
                if let RuntimeValue::Object(obj) = &args[1] {
                    if let Some(method_val) = obj.properties.get("method") {
                        String::from_raccoon(method_val).unwrap_or_else(|_| "GET".to_string())
                    } else {
                        "GET".to_string()
                    }
                } else {
                    "GET".to_string()
                }
            } else {
                "GET".to_string()
            };

            let mut response_properties = HashMap::new();
            response_properties.insert("status".to_string(), RuntimeValue::Int(IntValue::new(200)));
            response_properties.insert(
                "statusText".to_string(),
                RuntimeValue::Str(StrValue::new("OK".to_string())),
            );

            let headers_map =
                MapValue::new(HashMap::new(), PrimitiveType::str(), PrimitiveType::str());
            response_properties.insert("headers".to_string(), RuntimeValue::Map(headers_map));

            let body = format!(
                "{{\"method\":\"{}\",\"url\":\"{}\",\"simulated\":true}}",
                method, url
            );
            response_properties.insert(
                "body".to_string(),
                RuntimeValue::Str(StrValue::new(body.clone())),
            );

            let mut json_properties = HashMap::new();
            json_properties.insert(
                "method".to_string(),
                RuntimeValue::Str(StrValue::new(method)),
            );
            json_properties.insert("url".to_string(), RuntimeValue::Str(StrValue::new(url)));
            json_properties.insert(
                "simulated".to_string(),
                RuntimeValue::Bool(BoolValue::new(true)),
            );
            let json_obj = ObjectValue::new(json_properties, PrimitiveType::any());
            response_properties.insert("json".to_string(), RuntimeValue::Object(json_obj));

            let response_obj = ObjectValue::new(response_properties, PrimitiveType::any());

            RuntimeValue::Future(FutureValue::new_resolved(
                RuntimeValue::Object(response_obj),
                PrimitiveType::any(),
            ))
        },
        2,
        Some(2),
    );

    registrar.register_fn(
        "fetch",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();

            format!("Fetched from: {}", url).to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "get",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            format!("GET {}", url).to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "post",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            let body = String::from_raccoon(&args[1]).unwrap_or_default();
            format!("POST {} with body: {}", url, body).to_raccoon()
        },
        2,
        Some(2),
    );
}
