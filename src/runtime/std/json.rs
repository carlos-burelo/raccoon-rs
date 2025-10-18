use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{
    BoolValue, FloatValue, IntValue, ListValue, MapValue, NullValue, RuntimeValue, StrValue,
};
use std::collections::HashMap;

pub struct JSONModule;

impl JSONModule {
    pub fn name() -> &'static str {
        "std:json"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("parse".to_string(), Self::create_parse_fn());
        exports.insert("stringify".to_string(), Self::create_stringify_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "parse" => Some(Self::create_parse_fn()),
            "stringify" => Some(Self::create_stringify_fn()),
            _ => None,
        }
    }

    fn create_parse_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(json_str) => {
                        match Self::parse_json_value(&json_str.value) {
                            Ok(value) => value,
                            Err(_) => RuntimeValue::Null(NullValue::new()),
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ))
    }

    fn create_stringify_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Str(StrValue::new("null".to_string()));
                }
                let value = &args[0];
                let indent = if args.len() > 1 {
                    match &args[1] {
                        RuntimeValue::Int(i) => i.value as usize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let json_str = Self::stringify_value(value, indent, 0);
                RuntimeValue::Str(StrValue::new(json_str))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::any()],
                return_type: PrimitiveType::str(),
                is_variadic: true,
            })),
        ))
    }

    // Simple JSON parser (basic implementation)
    fn parse_json_value(s: &str) -> Result<RuntimeValue, String> {
        let trimmed = s.trim();

        if trimmed == "null" {
            return Ok(RuntimeValue::Null(NullValue::new()));
        }

        if trimmed == "true" {
            return Ok(RuntimeValue::Bool(BoolValue::new(true)));
        }

        if trimmed == "false" {
            return Ok(RuntimeValue::Bool(BoolValue::new(false)));
        }

        // Try to parse as number
        if let Ok(num) = trimmed.parse::<i64>() {
            return Ok(RuntimeValue::Int(IntValue::new(num)));
        }

        if let Ok(num) = trimmed.parse::<f64>() {
            return Ok(RuntimeValue::Float(FloatValue::new(num)));
        }

        // Try to parse as string
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            let content = &trimmed[1..trimmed.len() - 1];
            let unescaped = content
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t");
            return Ok(RuntimeValue::Str(StrValue::new(unescaped)));
        }

        // Try to parse as array
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            return Self::parse_json_array(trimmed);
        }

        // Try to parse as object
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            return Self::parse_json_object(trimmed);
        }

        Err(format!("Invalid JSON: {}", trimmed))
    }

    fn parse_json_array(s: &str) -> Result<RuntimeValue, String> {
        let content = s[1..s.len() - 1].trim();
        if content.is_empty() {
            return Ok(RuntimeValue::List(ListValue::new(
                vec![],
                PrimitiveType::any(),
            )));
        }

        let elements = Self::split_json_elements(content)?;
        let mut values = Vec::new();

        for elem in elements {
            values.push(Self::parse_json_value(elem.trim())?);
        }

        Ok(RuntimeValue::List(ListValue::new(
            values,
            PrimitiveType::any(),
        )))
    }

    fn parse_json_object(s: &str) -> Result<RuntimeValue, String> {
        let content = s[1..s.len() - 1].trim();
        if content.is_empty() {
            return Ok(RuntimeValue::Map(MapValue::new(
                HashMap::new(),
                PrimitiveType::str(),
                PrimitiveType::any(),
            )));
        }

        let pairs = Self::split_json_elements(content)?;
        let mut map = HashMap::new();

        for pair in pairs {
            let parts: Vec<&str> = pair.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid JSON object pair: {}", pair));
            }

            let key_str = parts[0].trim();
            if !key_str.starts_with('"') || !key_str.ends_with('"') {
                return Err(format!("Invalid JSON object key: {}", key_str));
            }

            let key = &key_str[1..key_str.len() - 1];
            let value = Self::parse_json_value(parts[1].trim())?;

            map.insert(key.to_string(), value);
        }

        Ok(RuntimeValue::Map(MapValue::new(
            map,
            PrimitiveType::str(),
            PrimitiveType::any(),
        )))
    }

    fn split_json_elements(s: &str) -> Result<Vec<&str>, String> {
        let mut elements = Vec::new();
        let mut start = 0;
        let mut depth = 0;
        let mut in_string = false;
        let mut escape = false;

        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let ch = chars[i];

            if escape {
                escape = false;
                continue;
            }

            if ch == '\\' {
                escape = true;
                continue;
            }

            if ch == '"' {
                in_string = !in_string;
                continue;
            }

            if in_string {
                continue;
            }

            match ch {
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    elements.push(&s[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }

        if start < s.len() {
            elements.push(&s[start..]);
        }

        Ok(elements)
    }

    fn stringify_value(value: &RuntimeValue, indent: usize, current_depth: usize) -> String {
        match value {
            RuntimeValue::Null(_) => "null".to_string(),
            RuntimeValue::Bool(b) => b.value.to_string(),
            RuntimeValue::Int(i) => i.value.to_string(),
            RuntimeValue::Float(f) => {
                if f.value.is_nan() {
                    "null".to_string()
                } else if f.value.is_infinite() {
                    "null".to_string()
                } else {
                    f.value.to_string()
                }
            }
            RuntimeValue::Str(s) => {
                let escaped = s
                    .value
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t");
                format!("\"{}\"", escaped)
            }
            RuntimeValue::List(list) => {
                if list.elements.is_empty() {
                    return "[]".to_string();
                }

                if indent == 0 {
                    let elements: Vec<String> = list
                        .elements
                        .iter()
                        .map(|v| Self::stringify_value(v, 0, 0))
                        .collect();
                    format!("[{}]", elements.join(","))
                } else {
                    let mut result = "[\n".to_string();
                    let spaces = " ".repeat((current_depth + 1) * indent);
                    let elements: Vec<String> = list
                        .elements
                        .iter()
                        .map(|v| {
                            format!(
                                "{}{}",
                                spaces,
                                Self::stringify_value(v, indent, current_depth + 1)
                            )
                        })
                        .collect();
                    result.push_str(&elements.join(",\n"));
                    result.push('\n');
                    result.push_str(&" ".repeat(current_depth * indent));
                    result.push(']');
                    result
                }
            }
            RuntimeValue::Map(map) => {
                if map.entries.is_empty() {
                    return "{}".to_string();
                }

                if indent == 0 {
                    let mut pairs = Vec::new();
                    for (k, v) in &map.entries {
                        pairs.push(format!(
                            "\"{}\":{}",
                            k.replace('\\', "\\\\").replace('"', "\\\""),
                            Self::stringify_value(v, 0, 0)
                        ));
                    }
                    format!("{{{}}}", pairs.join(","))
                } else {
                    let mut result = "{\n".to_string();
                    let spaces = " ".repeat((current_depth + 1) * indent);
                    let mut pairs = Vec::new();
                    for (k, v) in &map.entries {
                        pairs.push(format!(
                            "{}\"{}\": {}",
                            spaces,
                            k.replace('\\', "\\\\").replace('"', "\\\""),
                            Self::stringify_value(v, indent, current_depth + 1)
                        ));
                    }
                    result.push_str(&pairs.join(",\n"));
                    result.push('\n');
                    result.push_str(&" ".repeat(current_depth * indent));
                    result.push('}');
                    result
                }
            }
            _ => "null".to_string(),
        }
    }
}
