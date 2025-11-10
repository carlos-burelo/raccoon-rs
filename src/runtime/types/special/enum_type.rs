use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct EnumType;

#[async_trait]
impl TypeHandler for EnumType {
    fn type_name(&self) -> &str {
        "enum"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let enum_val = match value {
            RuntimeValue::Enum(e) => e,
            RuntimeValue::EnumObject(e) => match method {
                "name" => return Ok(RuntimeValue::Str(StrValue::new(e.enum_name.clone()))),
                "toString" | "toStr" => {
                    return Ok(RuntimeValue::Str(StrValue::new(format!(
                        "[Enum: {}]",
                        e.enum_name
                    ))))
                }
                _ => {
                    return Err(RaccoonError::new(
                        format!("Method '{}' not found on enum object", method),
                        position,
                        file,
                    ))
                }
            },
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected enum, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "name" => Ok(RuntimeValue::Str(StrValue::new(format!(
                "{}.{}",
                enum_val.enum_name, enum_val.member_name
            )))),
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(
                enum_val.member_name.clone(),
            ))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on enum", method),
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
            format!("Static method '{}' not found on enum type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "name" | "toString" | "toStr")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
