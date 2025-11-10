/// EitherType - Either type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// EitherType - Either type handler
// ============================================================================

pub struct EitherType;

impl EitherType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("either", "Either type for sum types (Left or Right)")
            .with_instance_methods(vec![
                MethodMetadata::new("isLeft", "bool", "Check if value is Left"),
                MethodMetadata::new("isRight", "bool", "Check if value is Right"),
                MethodMetadata::new("unwrapLeft", "any", "Unwrap Left value or panic"),
                MethodMetadata::new("unwrapRight", "any", "Unwrap Right value or panic"),
            ])
    }
}

#[async_trait]
impl TypeHandler for EitherType {
    fn type_name(&self) -> &str {
        "either"
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
            "isLeft" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            "isRight" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }
            "unwrapLeft" | "unwrapRight" => {
                // Placeholder - needs proper implementation
                Err(RaccoonError::new(
                    format!("Method '{}' not yet implemented on either", method),
                    position,
                    file,
                ))
            }
            _ => Err(method_not_found_error("either", method, position, file)),
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
            "either", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
