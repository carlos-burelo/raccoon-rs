use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ResultType - Result type for error handling (Result<T, E>)
// ============================================================================
// Note: This is a placeholder - actual Result type would need proper value representation

pub struct ResultType;

#[async_trait]
impl TypeHandler for ResultType {
    fn type_name(&self) -> &str {
        "result"
    }

    fn call_instance_method(
        &self,
        _value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "isOk" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            "isErr" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on result", method),
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
        Err(RaccoonError::new(
            format!("Static method '{}' not found on result type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "isOk" | "isErr" | "unwrap" | "unwrapErr")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
