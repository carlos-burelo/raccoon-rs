/// EnumType - Enumeration type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// EnumType - Enumeration type handler
// ============================================================================

pub struct EnumType;

impl EnumType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("enum", "Enumeration type representing named constants")
            .with_instance_methods(vec![
                MethodMetadata::new("name", "str", "Get enum variant name"),
                MethodMetadata::new("toString", "str", "Convert to string representation")
                    .with_alias("toStr"),
            ])
    }
}

#[async_trait]
impl TypeHandler for EnumType {
    fn type_name(&self) -> &str {
        "enum"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        require_args(&args, 0, method, position, file.clone())?;

        let enum_val = match value {
            RuntimeValue::Enum(e) => e,
            RuntimeValue::EnumObject(e) => match method {
                "name" => return Ok(RuntimeValue::Str(StrValue::new(e.enum_name.clone()))),
                "toString" | "toStr" => {
                    return Ok(RuntimeValue::Str(StrValue::new(format!(
                        "[Enum: {}]",
                        e.enum_name
                    ))))
                }
                _ => {
                    return Err(method_not_found_error(
                        "enum object",
                        method,
                        position,
                        file,
                    ))
                }
            },
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected enum, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "name" => Ok(RuntimeValue::Str(StrValue::new(format!(
                "{}.{}",
                enum_val.enum_name, enum_val.member_name
            )))),
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(
                enum_val.member_name.clone(),
            ))),
            _ => Err(method_not_found_error("enum", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error(
            "enum", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
