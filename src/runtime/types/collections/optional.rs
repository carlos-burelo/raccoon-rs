use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, NullValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// OptionalType - Optional value (Option<T> / Maybe<T>)
// ============================================================================

pub struct OptionalType;

#[async_trait]
impl TypeHandler for OptionalType {
    fn type_name(&self) -> &str {
        "optional"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "isSome" => {
                let is_some = !matches!(value, RuntimeValue::Null(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_some)))
            }
            "isNone" => {
                let is_none = matches!(value, RuntimeValue::Null(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_none)))
            }
            "unwrap" => {
                if matches!(value, RuntimeValue::Null(_)) {
                    Err(RaccoonError::new(
                        "Cannot unwrap None value".to_string(),
                        position,
                        file,
                    ))
                } else {
                    Ok(value.clone())
                }
            }
            "unwrapOr" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "unwrapOr requires 1 argument (default value)".to_string(),
                        position,
                        file,
                    ));
                }
                if matches!(value, RuntimeValue::Null(_)) {
                    Ok(args[0].clone())
                } else {
                    Ok(value.clone())
                }
            }
            "map" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "map requires 1 argument (function)".to_string(),
                        position,
                        file,
                    ));
                }
                if matches!(value, RuntimeValue::Null(_)) {
                    Ok(RuntimeValue::Null(NullValue::new()))
                } else {
                    // Would need to call the function with the value
                    // This requires callback execution which is more complex
                    Err(RaccoonError::new(
                        "Optional.map not yet fully implemented".to_string(),
                        position,
                        file,
                    ))
                }
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on optional", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "some" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "some requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(args[0].clone())
            }
            "none" => Ok(RuntimeValue::Null(NullValue::new())),
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on optional type", method),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isSome" | "isNone" | "unwrap" | "unwrapOr" | "map"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "some" | "none")
    }
}
