use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, NullValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

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
                require_args(&args, 0, method, position, file)?;
                let is_some = !matches!(value, RuntimeValue::Null(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_some)))
            }
            "isNone" => {
                require_args(&args, 0, method, position, file)?;
                let is_none = matches!(value, RuntimeValue::Null(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_none)))
            }
            "unwrap" => {
                require_args(&args, 0, method, position, file.clone())?;
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
                require_args(&args, 1, method, position, file)?;
                if matches!(value, RuntimeValue::Null(_)) {
                    Ok(args[0].clone())
                } else {
                    Ok(value.clone())
                }
            }
            "expect" => {
                require_args(&args, 1, method, position, file.clone())?;
                let message = extract_str(&args[0], "message", position, file.clone())?;
                if matches!(value, RuntimeValue::Null(_)) {
                    Err(RaccoonError::new(
                        format!("Expect failed: {}", message),
                        position,
                        file,
                    ))
                } else {
                    Ok(value.clone())
                }
            }
            _ => Err(method_not_found_error("optional", method, position, file)),
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
                require_args(&args, 1, method, position, file)?;
                Ok(args[0].clone())
            }
            "none" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            _ => Err(static_method_not_found_error(
                "optional", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isSome" | "isNone" | "unwrap" | "unwrapOr" | "expect"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "some" | "none")
    }
}
