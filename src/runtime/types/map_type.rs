use super::TypeHandler;
use async_trait::async_trait;
use crate::error::RaccoonError;
use crate::runtime::{BoolValue, IntValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;

pub struct MapType;

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
        let map = match value {
            RuntimeValue::Map(m) => m,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected map, got {}", value.get_name()),
                    position,
                    file,
                ))
            }
        };

        match method {
            "get" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "get requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let key = args[0].to_string();
                Ok(map
                    .entries
                    .get(&key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())))
            }
            "set" => {
                if args.len() != 2 {
                    return Err(RaccoonError::new(
                        "set requires 2 arguments (key, value)".to_string(),
                        position,
                        file,
                    ));
                }
                let key = args[0].to_string();
                let value = args[1].clone();
                map.entries.insert(key, value);
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "has" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "has requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let key = args[0].to_string();
                Ok(RuntimeValue::Bool(BoolValue::new(
                    map.entries.contains_key(&key),
                )))
            }
            "delete" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "delete requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let key = args[0].to_string();
                let existed = map.entries.remove(&key).is_some();
                Ok(RuntimeValue::Bool(BoolValue::new(existed)))
            }
            "clear" => {
                if args.len() != 0 {
                    return Err(RaccoonError::new(
                        "clear requires 0 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                map.entries.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "size" => {
                if args.len() != 0 {
                    return Err(RaccoonError::new(
                        "size requires 0 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(RuntimeValue::Int(IntValue::new(map.entries.len() as i64)))
            }
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(map.to_string()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on map", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!("Static method '{}' not found on map type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "get" | "set" | "has" | "delete" | "clear" | "size" | "toStr"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
