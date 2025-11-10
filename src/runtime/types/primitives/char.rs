/// Refactored CharType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// CharType - Single character
// ============================================================================

pub struct CharType;

#[async_trait]
impl TypeHandler for CharType {
    fn type_name(&self) -> &str {
        "char"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let ch = match value {
            RuntimeValue::Str(s) if s.value.len() == 1 => s.value.chars().next().unwrap(),
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected char, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            // Conversion methods
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(ch.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(ch as i64)))
            }

            // Predicate methods
            "isDigit" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(ch.is_numeric())))
            }
            "isAlpha" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(ch.is_alphabetic())))
            }
            "isWhitespace" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(ch.is_whitespace())))
            }

            // Case conversion methods
            "toUpperCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(
                    ch.to_uppercase().to_string(),
                )))
            }
            "toLowerCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(
                    ch.to_lowercase().to_string(),
                )))
            }

            _ => Err(method_not_found_error("char", method, position, file)),
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
            "fromCode" => {
                require_args(&args, 1, method, position, file.clone())?;
                let code = extract_int(&args[0], "code", position, file.clone())?;
                if let Some(ch) = char::from_u32(code as u32) {
                    Ok(RuntimeValue::Str(StrValue::new(ch.to_string())))
                } else {
                    Err(RaccoonError::new(
                        format!("Invalid character code: {}", code),
                        position,
                        file,
                    ))
                }
            }
            _ => Err(static_method_not_found_error(
                "char", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toString"
                | "toStr"
                | "toInt"
                | "isDigit"
                | "isAlpha"
                | "isWhitespace"
                | "toUpperCase"
                | "toLowerCase"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "fromCode")
    }
}
