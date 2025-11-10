/// ClassType - Class type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ClassType - Class type handler
// ============================================================================

pub struct ClassType;

impl ClassType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new(
            "class",
            "Class type representing class definitions and instances",
        )
        .with_instance_methods(vec![
            MethodMetadata::new("name", "str", "Get class name"),
            MethodMetadata::new("constructor", "str", "Get constructor name"),
            MethodMetadata::new("toString", "str", "Convert to string representation")
                .with_alias("toStr"),
        ])
    }
}

#[async_trait]
impl TypeHandler for ClassType {
    fn type_name(&self) -> &str {
        "class"
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

        match value {
            RuntimeValue::Class(c) => match method {
                "name" => Ok(RuntimeValue::Str(StrValue::new(c.class_name.clone()))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[Class: {}]",
                    c.class_name
                )))),
                _ => Err(method_not_found_error("class", method, position, file)),
            },
            RuntimeValue::ClassInstance(inst) => match method {
                "constructor" => Ok(RuntimeValue::Str(StrValue::new(inst.class_name.clone()))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[instance of {}]",
                    inst.class_name
                )))),
                _ => Err(method_not_found_error(
                    "class instance",
                    method,
                    position,
                    file,
                )),
            },
            _ => Err(RaccoonError::new(
                format!("Expected class or class instance, got {}", value.get_name()),
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
        Err(static_method_not_found_error(
            "class", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
