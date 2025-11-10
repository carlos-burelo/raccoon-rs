use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

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
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_float(value, "this", position, file.clone())? as f32;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "toFloat" | "toF64" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num as f64)))
            }

            "floor" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.floor() as i64)))
            }
            "ceil" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.ceil() as i64)))
            }
            "round" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.round() as i64)))
            }

            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.abs() as f64)))
            }
            "sqrt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.sqrt() as f64)))
            }

            _ => Err(method_not_found_error("f32", method, position, file)),
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
                require_args(&args, 1, method, position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file.clone())?;
                match s.trim().parse::<f32>() {
                    Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num as f64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as f32", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("f32", method, position, file)),
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
            _ => Err(property_not_found_error("f32", property, position, file)),
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
