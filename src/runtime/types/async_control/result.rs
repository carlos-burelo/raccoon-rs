/// ResultType - Result type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ResultType - Result type handler
// ============================================================================

pub struct ResultType;

impl ResultType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("result", "Result type for error handling").with_instance_methods(vec![
            MethodMetadata::new("isOk", "bool", "Check if result is Ok"),
            MethodMetadata::new("isErr", "bool", "Check if result is Err"),
            MethodMetadata::new("unwrap", "any", "Unwrap Ok value or panic"),
            MethodMetadata::new("unwrapErr", "any", "Unwrap Err value or panic"),
        ])
    }
}

#[async_trait]
impl TypeHandler for ResultType {
    fn type_name(&self) -> &str {
        "result"
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
            "isOk" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            "isErr" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }
            "unwrap" | "unwrapErr" => {
                // Placeholder - needs proper implementation
                Err(RaccoonError::new(
                    format!("Method '{}' not yet implemented on result", method),
                    position,
                    file,
                ))
            }
            _ => Err(method_not_found_error("result", method, position, file)),
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
            "result", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
