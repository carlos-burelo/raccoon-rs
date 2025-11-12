use crate::error::RaccoonError;

use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct BoolType;

#[async_trait]
impl TypeHandler for BoolType {
    fn type_name(&self) -> &str {
        "bool"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let b = match value {
            RuntimeValue::Bool(b) => b.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected bool, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(b.to_string()))),
            _ => Err(method_not_found_error("bool", method, position, file)),
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
            "parse" => {
                require_args(&args, 1, "parse", position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file.clone())?;
                let trimmed = s.trim().to_lowercase();

                match trimmed.as_str() {
                    "true" | "1" | "yes" | "y" => Ok(RuntimeValue::Bool(BoolValue::new(true))),
                    "false" | "0" | "no" | "n" => Ok(RuntimeValue::Bool(BoolValue::new(false))),
                    _ => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as bool", s),
                        position,
                        file,
                    )),
                }
            }
            "tryParse" => {
                require_args(&args, 1, "tryParse", position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file)?;
                let trimmed = s.trim().to_lowercase();

                match trimmed.as_str() {
                    "true" | "1" | "yes" | "y" => Ok(RuntimeValue::Bool(BoolValue::new(true))),
                    "false" | "0" | "no" | "n" => Ok(RuntimeValue::Bool(BoolValue::new(false))),
                    _ => Ok(RuntimeValue::Null(NullValue::new())),
                }
            }
            _ => Err(static_method_not_found_error(
                "bool", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse" | "tryParse")
    }
}
