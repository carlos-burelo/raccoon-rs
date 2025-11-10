/// FunctionType - Function type handler with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// FunctionType - Function type handler
// ============================================================================

pub struct FunctionType;

impl FunctionType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("function", "Function type representing callable code")
            .with_instance_methods(vec![
                MethodMetadata::new("length", "int", "Get number of function parameters"),
                MethodMetadata::new("name", "str", "Get function name"),
                MethodMetadata::new("isAsync", "bool", "Check if function is async"),
                MethodMetadata::new("toString", "str", "Convert to string representation")
                    .with_alias("toStr"),
            ])
    }
}

#[async_trait]
impl TypeHandler for FunctionType {
    fn type_name(&self) -> &str {
        "function"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        require_args(&args, 0, method, position, file.clone())?;

        let func = match value {
            RuntimeValue::Function(f) => f,
            RuntimeValue::NativeFunction(_f) => match method {
                "name" => return Ok(RuntimeValue::Str(StrValue::new("[native]".to_string()))),
                "length" => return Ok(RuntimeValue::Int(IntValue::new(0))),
                "toString" | "toStr" => {
                    return Ok(RuntimeValue::Str(StrValue::new(
                        "[Native Function]".to_string(),
                    )))
                }
                _ => return Err(method_not_found_error("function", method, position, file)),
            },
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected function, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
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
            _ => Err(method_not_found_error("function", method, position, file)),
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
            "function", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
