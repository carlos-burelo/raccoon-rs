use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct ClassType;

#[async_trait]
impl TypeHandler for ClassType {
    fn type_name(&self) -> &str {
        "class"
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
            RuntimeValue::Class(c) => match method {
                "name" => Ok(RuntimeValue::Str(StrValue::new(c.class_name.clone()))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[Class: {}]",
                    c.class_name
                )))),
                _ => Err(RaccoonError::new(
                    format!("Method '{}' not found on class", method),
                    position,
                    file,
                )),
            },
            RuntimeValue::ClassInstance(inst) => match method {
                "constructor" => Ok(RuntimeValue::Str(StrValue::new(inst.class_name.clone()))),
                "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[instance of {}]",
                    inst.class_name
                )))),
                _ => Err(RaccoonError::new(
                    format!("Method '{}' not found on class instance", method),
                    position,
                    file,
                )),
            },
            _ => Err(RaccoonError::new(
                format!("Expected class or class instance, got {}", value.get_name()),
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
            format!("Static method '{}' not found on class type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "name" | "constructor" | "toString" | "toStr")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
