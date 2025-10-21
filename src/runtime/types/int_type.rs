use super::TypeHandler;
use crate::error::RaccoonError;
use crate::runtime::{DecimalValue, FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct IntType;

#[async_trait]
impl TypeHandler for IntType {
    fn type_name(&self) -> &str {
        "int"
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
            RuntimeValue::Int(i) => i.value as f64,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected int, got {}", value.get_name()),
                    position,
                    file,
                ))
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "toI8" => Ok(RuntimeValue::Int(IntValue::new(num as i8 as i64))),
            "toI16" => Ok(RuntimeValue::Int(IntValue::new(num as i16 as i64))),
            "toI32" => Ok(RuntimeValue::Int(IntValue::new(num as i32 as i64))),
            "toI64" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "toU8" => Ok(RuntimeValue::Int(IntValue::new(num as u8 as i64))),
            "toU16" => Ok(RuntimeValue::Int(IntValue::new(num as u16 as i64))),
            "toU32" => Ok(RuntimeValue::Int(IntValue::new(num as u32 as i64))),
            "toU64" => Ok(RuntimeValue::Int(IntValue::new(num as u64 as i64))),
            "toF32" => Ok(RuntimeValue::Float(FloatValue::new(num as f32 as f64))),
            "toF64" => Ok(RuntimeValue::Float(FloatValue::new(num))),
            "toDecimal" => Ok(RuntimeValue::Decimal(DecimalValue::new(num))),
            "toFloat" => Ok(RuntimeValue::Float(FloatValue::new(num))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new((num as i64).abs()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on int", method),
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
                    RuntimeValue::Str(s) => {
                        match s.value.trim().parse::<i64>() {
                            Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                            Err(_) => Err(RaccoonError::new(
                                format!("Failed to parse '{}' as int", s.value),
                                position,
                                file,
                            )),
                        }
                    }
                    _ => Err(RaccoonError::new(
                        "parse requires string argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "tryParse" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "tryParse requires 1 argument (string)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        match s.value.trim().parse::<i64>() {
                            Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                            Err(_) => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                        }
                    }
                    _ => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                }
            }
            "compare" => {
                if args.len() != 2 {
                    return Err(RaccoonError::new(
                        "compare requires 2 arguments (int, int)".to_string(),
                        position,
                        file,
                    ));
                }
                let a = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => return Err(RaccoonError::new(
                        "compare requires int arguments".to_string(),
                        position,
                        file,
                    )),
                };
                let b = match &args[1] {
                    RuntimeValue::Int(i) => i.value,
                    _ => return Err(RaccoonError::new(
                        "compare requires int arguments".to_string(),
                        position,
                        file,
                    )),
                };
                Ok(RuntimeValue::Int(IntValue::new(if a < b { -1 } else if a > b { 1 } else { 0 })))
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on int type", method),
                position,
                file,
            ))
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MAX))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MIN))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on int type", property),
                position,
                file,
            ))
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toStr"
                | "toInt"
                | "toI8"
                | "toI16"
                | "toI32"
                | "toI64"
                | "toU8"
                | "toU16"
                | "toU32"
                | "toU64"
                | "toF32"
                | "toF64"
                | "toDecimal"
                | "toFloat"
                | "abs"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse" | "tryParse" | "compare")
    }
}
