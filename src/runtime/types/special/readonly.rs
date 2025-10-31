use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ReadonlyType - Readonly type (readonly T)
// ============================================================================
// Note: Readonly types are typically compile-time only

pub struct ReadonlyType;

#[async_trait]
impl TypeHandler for ReadonlyType {
    fn type_name(&self) -> &str {
        "readonly"
    }

    fn call_instance_method(
        &self,
        _value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!("Method '{}' not found on readonly", method),
            position,
            file,
        ))
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!("Static method '{}' not found on readonly type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, _method: &str) -> bool {
        false
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
