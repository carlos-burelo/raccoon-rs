use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct FunctionType;

#[async_trait]
impl TypeHandler for FunctionType {
    fn type_name(&self) -> &str {
        "function"
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
            RuntimeValue::Function(func) => match method {
                "length" => Ok(RuntimeValue::Int(IntValue::new(
                    func.parameters.len() as i64
                ))),
                "isAsync" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                    func.is_async,
                ))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[Function: {} params]",
                    func.parameters.len()
                )))),
                _ => Err(RaccoonError::new(
                    format!("Method '{}' not found on function", method),
                    position,
                    file,
                )),
            },
            RuntimeValue::NativeFunction(_f) => match method {
                "name" => Ok(RuntimeValue::Str(StrValue::new("[native]".to_string()))),
                "length" => Ok(RuntimeValue::Int(IntValue::new(0))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(
                    "[Native Function]".to_string(),
                ))),
                _ => Err(RaccoonError::new(
                    format!("Method '{}' not found on native function", method),
                    position,
                    file,
                )),
            },
            _ => Err(RaccoonError::new(
                format!("Expected function, got {}", value.get_name()),
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
            format!("Static method '{}' not found on function type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "length" | "name" | "isAsync" | "toString" | "toStr")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
