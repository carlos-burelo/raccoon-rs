use super::TypeHandler;
use crate::error::RaccoonError;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;

pub struct BoolType;

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
                ))
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(b.to_string()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on bool", method),
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
            "parse" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "parse requires 1 argument (string)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        let trimmed = s.value.trim().to_lowercase();
                        match trimmed.as_str() {
                            "true" | "1" | "yes" | "y" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(true))),
                            "false" | "0" | "no" | "n" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                            _ => Err(RaccoonError::new(
                                format!("Failed to parse '{}' as bool", s.value),
                                position,
                                file,
                            )),
                        }
                    }
                    _ => Err(RaccoonError::new(
                        "parse requires string argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "tryParse" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "tryParse requires 1 argument (string)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        let trimmed = s.value.trim().to_lowercase();
                        match trimmed.as_str() {
                            "true" | "1" | "yes" | "y" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(true))),
                            "false" | "0" | "no" | "n" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                            _ => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                        }
                    }
                    _ => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on bool type", method),
                position,
                file,
            ))
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse" | "tryParse")
    }
}
