use super::TypeHandler;
use crate::error::RaccoonError;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;

pub struct DecimalType;

impl TypeHandler for DecimalType {
    fn type_name(&self) -> &str {
        "decimal"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = match value {
            RuntimeValue::Decimal(d) => d.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected decimal, got {}", value.get_name()),
                    position,
                    file,
                ))
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "toFloat" => Ok(RuntimeValue::Float(FloatValue::new(num))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on decimal", method),
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
            format!("Static method '{}' not found on decimal type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "toFloat")
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
