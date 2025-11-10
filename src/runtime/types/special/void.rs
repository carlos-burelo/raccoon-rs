/// VoidType - Void type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// VoidType - Void type handler
// ============================================================================

pub struct VoidType;

impl VoidType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("void", "Void type representing absence of return value")
            .with_instance_methods(vec![MethodMetadata::new(
                "toString",
                "str",
                "Convert to string representation",
            )
            .with_alias("toStr")])
    }
}

#[async_trait]
impl TypeHandler for VoidType {
    fn type_name(&self) -> &str {
        "void"
    }

    fn call_instance_method(
        &self,
        _value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        require_args(&args, 0, method, position, file.clone())?;

        match method {
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new("void".to_string()))),
            _ => Err(method_not_found_error("void", method, position, file)),
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
            "void", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
