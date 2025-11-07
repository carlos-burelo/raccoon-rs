use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{DecimalValue, FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// Float64Type - 64-bit floating point (default float)
// ============================================================================

pub struct Float64Type;

#[async_trait]
impl TypeHandler for Float64Type {
    fn type_name(&self) -> &str {
        "float"
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
            RuntimeValue::Float(f) => f.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected float, got {}", value.get_name()),
                    position,
                    file,
                ));
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
            "floor" => Ok(RuntimeValue::Int(IntValue::new(num.floor() as i64))),
            "ceil" => Ok(RuntimeValue::Int(IntValue::new(num.ceil() as i64))),
            "round" => Ok(RuntimeValue::Int(IntValue::new(num.round() as i64))),
            "abs" => Ok(RuntimeValue::Float(FloatValue::new(num.abs()))),
            "sqrt" => Ok(RuntimeValue::Float(FloatValue::new(num.sqrt()))),
            "sign" => Ok(RuntimeValue::Int(IntValue::new(
                if num > 0.0 {
                    1
                } else if num < 0.0 {
                    -1
                } else {
                    0
                },
            ))),
            "isNaN" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num.is_nan()))),
            "isInfinite" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num.is_infinite()))),
            "isFinite" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num.is_finite()))),
            "isPositive" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num > 0.0))),
            "isNegative" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num < 0.0))),
            "isZero" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num == 0.0))),
            "clamp" => {
                if _args.len() != 2 {
                    return Err(RaccoonError::new(
                        "clamp requires 2 arguments (min, max)".to_string(),
                        position,
                        file,
                    ));
                }
                let min_val = match &_args[0] {
                    RuntimeValue::Float(f) => f.value,
                    RuntimeValue::Int(i) => i.value as f64,
                    _ => {
                        return Err(RaccoonError::new(
                            "clamp requires numeric arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let max_val = match &_args[1] {
                    RuntimeValue::Float(f) => f.value,
                    RuntimeValue::Int(i) => i.value as f64,
                    _ => {
                        return Err(RaccoonError::new(
                            "clamp requires numeric arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                Ok(RuntimeValue::Float(FloatValue::new(num.max(min_val).min(max_val))))
            }
            "pow" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "pow requires 1 argument (exponent)".to_string(),
                        position,
                        file,
                    ));
                }
                let exp = match &_args[0] {
                    RuntimeValue::Float(f) => f.value,
                    RuntimeValue::Int(i) => i.value as f64,
                    _ => {
                        return Err(RaccoonError::new(
                            "pow requires numeric argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                Ok(RuntimeValue::Float(FloatValue::new(num.powf(exp))))
            }
            "sin" => Ok(RuntimeValue::Float(FloatValue::new(num.sin()))),
            "cos" => Ok(RuntimeValue::Float(FloatValue::new(num.cos()))),
            "tan" => Ok(RuntimeValue::Float(FloatValue::new(num.tan()))),
            "asin" => Ok(RuntimeValue::Float(FloatValue::new(num.asin()))),
            "acos" => Ok(RuntimeValue::Float(FloatValue::new(num.acos()))),
            "atan" => Ok(RuntimeValue::Float(FloatValue::new(num.atan()))),
            "exp" => Ok(RuntimeValue::Float(FloatValue::new(num.exp()))),
            "ln" => Ok(RuntimeValue::Float(FloatValue::new(num.ln()))),
            "log10" => Ok(RuntimeValue::Float(FloatValue::new(num.log10()))),
            "log2" => Ok(RuntimeValue::Float(FloatValue::new(num.log2()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on float", method),
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
                        Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as float", s.value),
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
            "tryParse" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "tryParse requires 1 argument (string)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Str(s) => match s.value.trim().parse::<f64>() {
                        Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num))),
                        Err(_) => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                    },
                    _ => Ok(RuntimeValue::Null(crate::runtime::NullValue::new())),
                }
            }
            "isNaN" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "isNaN requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Float(f) => Ok(RuntimeValue::Bool(
                        crate::runtime::BoolValue::new(f.value.is_nan()),
                    )),
                    _ => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                }
            }
            "isInfinite" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "isInfinite requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Float(f) => Ok(RuntimeValue::Bool(
                        crate::runtime::BoolValue::new(f.value.is_infinite()),
                    )),
                    _ => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                }
            }
            "isFinite" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "isFinite requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Float(f) => Ok(RuntimeValue::Bool(
                        crate::runtime::BoolValue::new(f.value.is_finite()),
                    )),
                    _ => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on float type", method),
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
            "maxValue" => Ok(RuntimeValue::Float(FloatValue::new(f64::MAX))),
            "minValue" => Ok(RuntimeValue::Float(FloatValue::new(f64::MIN))),
            "infinity" => Ok(RuntimeValue::Float(FloatValue::new(f64::INFINITY))),
            "negativeInfinity" => Ok(RuntimeValue::Float(FloatValue::new(f64::NEG_INFINITY))),
            "nan" => Ok(RuntimeValue::Float(FloatValue::new(f64::NAN))),
            "epsilon" => Ok(RuntimeValue::Float(FloatValue::new(f64::EPSILON))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on float type", property),
                position,
                file,
            )),
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
                | "floor"
                | "ceil"
                | "round"
                | "abs"
                | "sqrt"
                | "sign"
                | "isNaN"
                | "isInfinite"
                | "isFinite"
                | "isPositive"
                | "isNegative"
                | "isZero"
                | "clamp"
                | "pow"
                | "sin"
                | "cos"
                | "tan"
                | "asin"
                | "acos"
                | "atan"
                | "exp"
                | "ln"
                | "log10"
                | "log2"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(
            method,
            "parse" | "tryParse" | "isNaN" | "isInfinite" | "isFinite"
        )
    }
}

// Alias for backwards compatibility
pub type FloatType = Float64Type;
