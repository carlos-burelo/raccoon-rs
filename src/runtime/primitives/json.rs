use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::Registrar;

primitive! {
    json::core_json_parse(json: String) -> String {
        match serde_json::from_str::<serde_json::Value>(&json) {
            Ok(value) => serde_json::to_string(&value)
                .unwrap_or_else(|_| "null".to_string()),
            Err(_) => "null".to_string(),
        }
    }
}

primitive! {
    json::core_json_stringify(value: String) -> String {
        value
    }
}

pub fn register_json_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, json, {
        core_json_parse: 1..=1,
        core_json_stringify: 1..=1,
    });
}
