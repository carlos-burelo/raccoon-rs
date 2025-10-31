use crate::error::RaccoonError;
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
                ));
            }
        };

        match method {
            "toStr" | "toString" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "toFloat" => Ok(RuntimeValue::Float(FloatValue::new(num))),
            "add" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "add requires 1 argument (decimal)".to_string(),
                        position,
                        file,
                    ));
                }
                match &_args[0] {
                    RuntimeValue::Decimal(other) => {
                        Ok(RuntimeValue::Decimal(DecimalValue::new(num + other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "add requires decimal argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "subtract" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "subtract requires 1 argument (decimal)".to_string(),
                        position,
                        file,
                    ));
                }
                match &_args[0] {
                    RuntimeValue::Decimal(other) => {
                        Ok(RuntimeValue::Decimal(DecimalValue::new(num - other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "subtract requires decimal argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "multiply" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "multiply requires 1 argument (decimal)".to_string(),
                        position,
                        file,
                    ));
                }
                match &_args[0] {
                    RuntimeValue::Decimal(other) => {
                        Ok(RuntimeValue::Decimal(DecimalValue::new(num * other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "multiply requires decimal argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "divide" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "divide requires 1 argument (decimal)".to_string(),
                        position,
                        file,
                    ));
                }
                match &_args[0] {
                    RuntimeValue::Decimal(other) => {
                        if other.value == 0.0 {
                            return Err(RaccoonError::new(
                                "Division by zero".to_string(),
                                position,
                                file,
                            ));
                        }
                        Ok(RuntimeValue::Decimal(DecimalValue::new(num / other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "divide requires decimal argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "round" => {
                let places = if _args.is_empty() {
                    0
                } else {
                    match &_args[0] {
                        RuntimeValue::Int(i) => i.value as u32,
                        _ => {
                            return Err(RaccoonError::new(
                                "round requires int argument for decimal places".to_string(),
                                position,
                                file,
                            ))
                        }
                    }
                };
                let multiplier = 10_f64.powi(places as i32);
                let rounded = (num * multiplier).round() / multiplier;
                Ok(RuntimeValue::Decimal(DecimalValue::new(rounded)))
            }
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<f64>() {
                        Ok(num) => Ok(RuntimeValue::Decimal(DecimalValue::new(num))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as decimal", s.value),
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
                format!("Static method '{}' not found on decimal type", method),
                position,
                file,
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
