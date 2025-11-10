use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct NeverType;

#[async_trait]
impl TypeHandler for NeverType {
    fn type_name(&self) -> &str {
        "never"
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
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new("never".to_string()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on never", method),
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
            format!("Static method '{}' not found on never type", method),
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
