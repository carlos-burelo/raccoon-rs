use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// NullType - Null/None value
// ============================================================================

pub struct NullType;

#[async_trait]
impl TypeHandler for NullType {
    fn type_name(&self) -> &str {
        "null"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match value {
            RuntimeValue::Null(_) => {}
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected null, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        }

        match method {
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new("null".to_string()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on null", method),
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
            format!("Static method '{}' not found on null type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toString" | "toStr")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
