//! HTTP context primitives
//! Basic HTTP operations

use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

// HTTP GET request
primitive! {
    http::core_http_get(url: String) -> String {
        match ureq::get(&url).call() {
            Ok(response) => match response.into_string() {
                Ok(body) => body,
                Err(_) => "".to_string(),
            },
            Err(_) => "".to_string(),
        }
    }
}

// HTTP POST request
primitive! {
    http::core_http_post(url: String, body: String) -> String {
        match ureq::post(&url).send_string(&body) {
            Ok(response) => match response.into_string() {
                Ok(body) => body,
                Err(_) => "".to_string(),
            },
            Err(_) => "".to_string(),
        }
    }
}

// Generic HTTP request with headers
pub fn core_http_request(args: Vec<RuntimeValue>) -> RuntimeValue {
    let method = String::from_raccoon(&args[0]).unwrap_or_default();
    let url = String::from_raccoon(&args[1]).unwrap_or_default();
    let body = String::from_raccoon(&args[2]).unwrap_or_default();
    let headers_json = String::from_raccoon(&args[3]).unwrap_or_else(|_| "{}".to_string());

    let agent = ureq::AgentBuilder::new().build();
    let mut request = agent.request(&method, &url);

    // Parse and add headers
    if let Ok(headers) =
        serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&headers_json)
    {
        for (key, value) in headers {
            if let Some(val_str) = value.as_str() {
                request = request.set(&key, val_str);
            }
        }
    }

    let result = if body.is_empty() {
        request.call()
    } else {
        request.send_string(&body)
    };

    match result {
        Ok(response) => match response.into_string() {
            Ok(body) => body.to_raccoon(),
            Err(_) => "".to_string().to_raccoon(),
        },
        Err(_) => "".to_string().to_raccoon(),
    }
}

/// Register all HTTP primitives
pub fn register_http_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, http, {
        core_http_get: 1..=1,
        core_http_post: 2..=2,
        core_http_request: 4..=4,
    });
}
