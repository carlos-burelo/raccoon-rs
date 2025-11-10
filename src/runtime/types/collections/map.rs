/// Refactored MapType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, IntValue, MapValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// MapType - Key-value map/dictionary (Map<K, V>)
// ============================================================================

pub struct MapType;

impl MapType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("map", "Key-value map/dictionary with string keys").with_instance_methods(
            vec![
                MethodMetadata::new("get", "any?", "Get value by key, returns null if not found")
                    .with_params(vec![ParamMetadata::new("key", "any")]),
                MethodMetadata::new("set", "null", "Set value for key").with_params(vec![
                    ParamMetadata::new("key", "any"),
                    ParamMetadata::new("value", "any"),
                ]),
                MethodMetadata::new("has", "bool", "Check if key exists")
                    .with_params(vec![ParamMetadata::new("key", "any")]),
                MethodMetadata::new("delete", "bool", "Remove key, returns true if existed")
                    .with_params(vec![ParamMetadata::new("key", "any")]),
                MethodMetadata::new("clear", "null", "Remove all entries"),
                MethodMetadata::new("size", "int", "Get number of entries").with_alias("length"),
                MethodMetadata::new("isEmpty", "bool", "Check if map is empty"),
                MethodMetadata::new("keys", "list<str>", "Get list of all keys"),
                MethodMetadata::new("values", "list<any>", "Get list of all values"),
                MethodMetadata::new("toStr", "str", "Convert to string"),
            ],
        )
    }

    /// Helper to extract map from RuntimeValue
    fn extract_map_mut<'a>(
        value: &'a mut RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<&'a mut MapValue, RaccoonError> {
        match value {
            RuntimeValue::Map(m) => Ok(m),
            _ => Err(RaccoonError::new(
                format!("Expected map, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

#[async_trait]
impl TypeHandler for MapType {
    fn type_name(&self) -> &str {
        "map"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let map = Self::extract_map_mut(value, position, file.clone())?;

        match method {
            "get" => {
                require_args(&args, 1, method, position, file)?;
                let key = args[0].to_string();
                Ok(map
                    .entries
                    .get(&key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())))
            }
            "set" => {
                require_args(&args, 2, method, position, file)?;
                let key = args[0].to_string();
                let value = args[1].clone();
                map.entries.insert(key, value);
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "has" => {
                require_args(&args, 1, method, position, file)?;
                let key = args[0].to_string();
                Ok(RuntimeValue::Bool(BoolValue::new(
                    map.entries.contains_key(&key),
                )))
            }
            "delete" => {
                require_args(&args, 1, method, position, file)?;
                let key = args[0].to_string();
                let existed = map.entries.remove(&key).is_some();
                Ok(RuntimeValue::Bool(BoolValue::new(existed)))
            }
            "clear" => {
                require_args(&args, 0, method, position, file)?;
                map.entries.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "size" | "length" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(map.entries.len() as i64)))
            }
            "isEmpty" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(map.entries.is_empty())))
            }
            "keys" => {
                require_args(&args, 0, method, position, file)?;
                let keys: Vec<RuntimeValue> = map
                    .entries
                    .keys()
                    .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                    .collect();
                Ok(RuntimeValue::List(crate::runtime::ListValue::new(
                    keys,
                    crate::ast::types::PrimitiveType::str(),
                )))
            }
            "values" => {
                require_args(&args, 0, method, position, file)?;
                let values: Vec<RuntimeValue> = map.entries.values().cloned().collect();
                Ok(RuntimeValue::List(crate::runtime::ListValue::new(
                    values,
                    crate::ast::types::PrimitiveType::any(),
                )))
            }
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(map.to_string())))
            }
            _ => Err(method_not_found_error("map", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error("map", method, position, file))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
