use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{
    BoolValue, DecimalValue, FloatValue, IntValue, NullValue, RuntimeValue, StrValue,
};
use crate::tokens::Position;
use async_trait::async_trait;

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
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_float(value, "this", position, file.clone())?;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "toI8" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i8 as i64)))
            }
            "toI16" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i16 as i64)))
            }
            "toI32" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i32 as i64)))
            }
            "toI64" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "toU8" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as u8 as i64)))
            }
            "toU16" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as u16 as i64)))
            }
            "toU32" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as u32 as i64)))
            }
            "toU64" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as u64 as i64)))
            }
            "toF32" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num as f32 as f64)))
            }
            "toF64" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num)))
            }
            "toFloat" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num)))
            }
            "toDecimal" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Decimal(DecimalValue::new(num)))
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
                Ok(RuntimeValue::Float(FloatValue::new(num.abs())))
            }
            "sqrt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.sqrt())))
            }
            "sign" => {
                require_args(&args, 0, method, position, file)?;
                let sign = if num > 0.0 {
                    1
                } else if num < 0.0 {
                    -1
                } else {
                    0
                };
                Ok(RuntimeValue::Int(IntValue::new(sign)))
            }
            "pow" => {
                require_args(&args, 1, method, position, file.clone())?;
                let exp = extract_numeric(&args[0], "exponent", position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.powf(exp))))
            }
            "clamp" => {
                require_args(&args, 2, method, position, file.clone())?;
                let min_val = extract_numeric(&args[0], "min", position, file.clone())?;
                let max_val = extract_numeric(&args[1], "max", position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(
                    num.max(min_val).min(max_val),
                )))
            }

            "sin" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.sin())))
            }
            "cos" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.cos())))
            }
            "tan" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.tan())))
            }
            "asin" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.asin())))
            }
            "acos" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.acos())))
            }
            "atan" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.atan())))
            }

            "exp" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.exp())))
            }
            "ln" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.ln())))
            }
            "log10" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.log10())))
            }
            "log2" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num.log2())))
            }

            "isNaN" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num.is_nan())))
            }
            "isInfinite" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num.is_infinite())))
            }
            "isFinite" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num.is_finite())))
            }
            "isPositive" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num > 0.0)))
            }
            "isNegative" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num < 0.0)))
            }
            "isZero" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num == 0.0)))
            }

            _ => Err(method_not_found_error("float", method, position, file)),
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
                    Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as float", s),
                        position,
                        file,
                    )),
                }
            }
            "tryParse" => {
                require_args(&args, 1, method, position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file)?;
                match s.trim().parse::<f64>() {
                    Ok(num) => Ok(RuntimeValue::Float(FloatValue::new(num))),
                    Err(_) => Ok(RuntimeValue::Null(NullValue::new())),
                }
            }
            "isNaN" => {
                require_args(&args, 1, method, position, file.clone())?;
                match &args[0] {
                    RuntimeValue::Float(f) => {
                        Ok(RuntimeValue::Bool(BoolValue::new(f.value.is_nan())))
                    }
                    _ => Ok(RuntimeValue::Bool(BoolValue::new(false))),
                }
            }
            "isInfinite" => {
                require_args(&args, 1, method, position, file.clone())?;
                match &args[0] {
                    RuntimeValue::Float(f) => {
                        Ok(RuntimeValue::Bool(BoolValue::new(f.value.is_infinite())))
                    }
                    _ => Ok(RuntimeValue::Bool(BoolValue::new(false))),
                }
            }
            "isFinite" => {
                require_args(&args, 1, method, position, file)?;
                match &args[0] {
                    RuntimeValue::Float(f) => {
                        Ok(RuntimeValue::Bool(BoolValue::new(f.value.is_finite())))
                    }
                    _ => Ok(RuntimeValue::Bool(BoolValue::new(false))),
                }
            }
            _ => Err(static_method_not_found_error(
                "float", method, position, file,
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
            _ => Err(property_not_found_error("float", property, position, file)),
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

pub type FloatType = Float64Type;
