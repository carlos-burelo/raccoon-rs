use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
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
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let ch = match value {
            RuntimeValue::Str(s) if s.value.len() == 1 => {
                s.value.chars().next().unwrap()
            }
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected char, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(ch.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(ch as i64))),
            "isDigit" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                ch.is_numeric(),
            ))),
            "isAlpha" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                ch.is_alphabetic(),
            ))),
            "isWhitespace" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                ch.is_whitespace(),
            ))),
            "toUpperCase" => Ok(RuntimeValue::Str(StrValue::new(
                ch.to_uppercase().to_string(),
            ))),
            "toLowerCase" => Ok(RuntimeValue::Str(StrValue::new(
                ch.to_lowercase().to_string(),
            ))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on char", method),
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
            "fromCode" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "fromCode requires 1 argument (int)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Int(i) => {
                        if let Some(ch) = char::from_u32(i.value as u32) {
                            Ok(RuntimeValue::Str(StrValue::new(ch.to_string())))
                        } else {
                            Err(RaccoonError::new(
                                format!("Invalid character code: {}", i.value),
                                position,
                                file,
                            ))
                        }
                    }
                    _ => Err(RaccoonError::new(
                        "fromCode requires int argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on char type", method),
                position,
                file,
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
