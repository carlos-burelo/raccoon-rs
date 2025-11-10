/// Refactored CharType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct CharTypeRefactored;

impl CharTypeRefactored {
    /// Returns complete type metadata
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("char", "Single character type")
            .with_instance_methods(vec![
                MethodMetadata::new("toString", "str", "Convert to string")
                    .with_alias("toStr"),
                MethodMetadata::new("toInt", "int", "Get character code"),
                MethodMetadata::new("isDigit", "bool", "Check if character is digit"),
                MethodMetadata::new("isAlpha", "bool", "Check if character is alphabetic"),
                MethodMetadata::new("isWhitespace", "bool", "Check if character is whitespace"),
                MethodMetadata::new("toUpperCase", "str", "Convert to uppercase"),
                MethodMetadata::new("toLowerCase", "str", "Convert to lowercase"),
            ])
            .with_static_methods(vec![
                MethodMetadata::new("fromCode", "char", "Create char from character code")
                    .with_params(vec![ParamMetadata::new("code", "int")]),
            ])
    }

    /// Extract char from RuntimeValue
    fn extract_char(
        value: &RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<char, RaccoonError> {
        match value {
            RuntimeValue::Str(s) if s.value.len() == 1 => {
                Ok(s.value.chars().next().unwrap())
            }
            _ => Err(RaccoonError::new(
                format!("Expected char, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

#[async_trait]
impl TypeHandler for CharTypeRefactored {
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
        let ch = Self::extract_char(value, position, file.clone())?;

        match method {
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(ch.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(ch as i64)))
            }
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
            "toUpperCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(ch.to_uppercase().to_string())))
            }
            "toLowerCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(ch.to_lowercase().to_string())))
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
                match char::from_u32(code as u32) {
                    Some(ch) => Ok(RuntimeValue::Str(StrValue::new(ch.to_string()))),
                    None => Err(RaccoonError::new(
                        format!("Invalid character code: {}", code),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("char", method, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }

    fn has_async_instance_method(&self, _method: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_int() {
        let handler = CharTypeRefactored;
        let mut value = RuntimeValue::Str(StrValue::new("A".to_string()));
        let result = handler
            .call_instance_method(&mut value, "toInt", vec![], Position::default(), None)
            .unwrap();

        match result {
            RuntimeValue::Int(i) => assert_eq!(i.value, 65),
            _ => panic!("Expected int"),
        }
    }

    #[test]
    fn test_metadata() {
        let metadata = CharTypeRefactored::metadata();
        assert_eq!(metadata.type_name, "char");
        assert!(metadata.has_instance_method("toStr"));
        assert!(metadata.has_static_method("fromCode"));
    }
}
