use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// EitherType - Either left or right value (Either<L, R>)
// ============================================================================
// Note: This is a placeholder - actual Either type would need proper value representation

pub struct EitherType;

#[async_trait]
impl TypeHandler for EitherType {
    fn type_name(&self) -> &str {
        "either"
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
            "isLeft" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            "isRight" => {
                // Placeholder implementation
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on either", method),
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
            format!("Static method '{}' not found on either type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isLeft" | "isRight" | "unwrapLeft" | "unwrapRight"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
