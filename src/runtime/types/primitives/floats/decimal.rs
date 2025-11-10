/// Refactored DecimalType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{DecimalValue, FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// DecimalType - Fixed precision decimal
// ============================================================================

pub struct DecimalType;

#[async_trait]
impl TypeHandler for DecimalType {
    fn type_name(&self) -> &str {
        "decimal"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_decimal(value, "this", position, file.clone())?;

        match method {
            // Conversion methods
            "toStr" | "toString" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "toFloat" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num)))
            }

            // Arithmetic methods
            "add" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_decimal(&args[0], "other", position, file)?;
                Ok(RuntimeValue::Decimal(DecimalValue::new(num + other)))
            }
            "subtract" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_decimal(&args[0], "other", position, file)?;
                Ok(RuntimeValue::Decimal(DecimalValue::new(num - other)))
            }
            "multiply" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_decimal(&args[0], "other", position, file)?;
                Ok(RuntimeValue::Decimal(DecimalValue::new(num * other)))
            }
            "divide" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_decimal(&args[0], "other", position, file.clone())?;
                if other == 0.0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(RuntimeValue::Decimal(DecimalValue::new(num / other)))
            }

            // Rounding methods
            "round" => {
                let places = if args.is_empty() {
                    0
                } else {
                    extract_int(&args[0], "places", position, file.clone())? as u32
                };
                let multiplier = 10_f64.powi(places as i32);
                let rounded = (num * multiplier).round() / multiplier;
                Ok(RuntimeValue::Decimal(DecimalValue::new(rounded)))
            }

            _ => Err(method_not_found_error("decimal", method, position, file)),
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
                match s.trim().parse::<f64>() {
                    Ok(num) => Ok(RuntimeValue::Decimal(DecimalValue::new(num))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as decimal", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error(
                "decimal", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toStr"
                | "toString"
                | "toInt"
                | "toFloat"
                | "add"
                | "subtract"
                | "multiply"
                | "divide"
                | "round"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
