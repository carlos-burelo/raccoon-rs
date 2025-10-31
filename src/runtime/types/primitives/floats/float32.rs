use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// Float32Type - 32-bit floating point
// ============================================================================

pub struct Float32Type;

#[async_trait]
impl TypeHandler for Float32Type {
    fn type_name(&self) -> &str {
        "f32"
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
            RuntimeValue::Float(f) => f.value as f32,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected f32, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "toFloat" | "toF64" => Ok(RuntimeValue::Float(FloatValue::new(num as f64))),
            "floor" => Ok(RuntimeValue::Int(IntValue::new(num.floor() as i64))),
            "ceil" => Ok(RuntimeValue::Int(IntValue::new(num.ceil() as i64))),
            "round" => Ok(RuntimeValue::Int(IntValue::new(num.round() as i64))),
            "abs" => Ok(RuntimeValue::Float(FloatValue::new(num.abs() as f64))),
            "sqrt" => Ok(RuntimeValue::Float(FloatValue::new(num.sqrt() as f64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on f32", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "parse" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "parse requires 1 argument (string)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Str(s) => match s.value.trim().parse::<f32>() {
                        Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num as f64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as f32", s.value),
                            position,
                            file,
                        )),
                    },
                    _ => Err(RaccoonError::new(
                        "parse requires string argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on f32 type", method),
                position,
                file,
            )),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Float(FloatValue::new(f32::MAX as f64))),
            "minValue" => Ok(RuntimeValue::Float(FloatValue::new(f32::MIN as f64))),
            "infinity" => Ok(RuntimeValue::Float(FloatValue::new(f32::INFINITY as f64))),
            "nan" => Ok(RuntimeValue::Float(FloatValue::new(f32::NAN as f64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on f32 type", property),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toStr" | "toInt" | "toFloat" | "toF64" | "floor" | "ceil" | "round" | "abs" | "sqrt"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
