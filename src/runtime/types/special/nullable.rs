use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use async_trait::async_trait;

pub struct NullableType;

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
        Err(RaccoonError::new(
            format!("Method '{}' not found on nullable", method),
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
            format!("Static method '{}' not found on nullable type", method),
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
