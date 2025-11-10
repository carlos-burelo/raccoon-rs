/// NullableType - Nullable type handler with metadata system (compile-time only)
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::TypeMetadata;
use crate::runtime::types::TypeHandler;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// NullableType - Nullable type handler (compile-time only)
// ============================================================================

pub struct NullableType;

impl NullableType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("nullable", "Nullable type (compile-time only)")
    }
}

#[async_trait]
impl TypeHandler for NullableType {
    fn type_name(&self) -> &str {
        "nullable"
    }

    fn call_instance_method(
        &self,
        _value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(method_not_found_error("nullable", method, position, file))
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error(
            "nullable", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
