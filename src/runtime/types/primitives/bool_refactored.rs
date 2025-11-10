/// BoolType - REFACTORED VERSION
/// Uses new helper functions and metadata system

use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::*;
use crate::runtime::types::TypeHandler;
use crate::error::RaccoonError;
use crate::runtime::{RuntimeValue, StrValue, BoolValue, NullValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct BoolType;

impl BoolType {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("bool", "Boolean type representing true/false values")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
            ])
            .with_static_methods(vec![
                MethodMetadata::new("parse", "bool", "Parse boolean from string")
                    .with_params(vec![ParamMetadata::new("value", "str")]),
                MethodMetadata::new("tryParse", "bool?", "Try parse boolean, returns null on failure")
                    .with_params(vec![ParamMetadata::new("value", "str")]),
            ])
    }
}

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
            _ => Err(static_method_not_found_error("bool", method, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse" | "tryParse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_to_str() {
        let handler = BoolType;
        let mut val = RuntimeValue::Bool(BoolValue::new(true));
        let result = handler.call_instance_method(&mut val, "toStr", vec![], Position::default(), None).unwrap();

        match result {
            RuntimeValue::Str(s) => assert_eq!(s.value, "true"),
            _ => panic!("Expected Str"),
        }
    }

    #[test]
    fn test_bool_parse() {
        let handler = BoolType;
        let args = vec![RuntimeValue::Str(StrValue::new("true".to_string()))];
        let result = handler.call_static_method("parse", args, Position::default(), None).unwrap();

        match result {
            RuntimeValue::Bool(b) => assert!(b.value),
            _ => panic!("Expected Bool"),
        }
    }
}
