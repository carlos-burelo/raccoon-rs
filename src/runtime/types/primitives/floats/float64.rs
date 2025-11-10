/// Refactored Float64Type using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{
    MethodMetadata, ParamMetadata, PropertyMetadata, TypeMetadata,
};
use crate::runtime::types::TypeHandler;
use crate::runtime::{
    BoolValue, DecimalValue, FloatValue, IntValue, NullValue, RuntimeValue, StrValue,
};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// Float64Type - 64-bit floating point (default float)
// ============================================================================

pub struct Float64Type;

impl Float64Type {
    /// Returns complete type metadata with all methods and properties
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new(
            "float",
            "64-bit floating point number type with mathematical methods",
        )
        .with_instance_methods(vec![
            // Conversion methods
            MethodMetadata::new("toStr", "str", "Convert to string"),
            MethodMetadata::new("toInt", "int", "Convert to int (truncate)"),
            MethodMetadata::new("toI8", "int", "Convert to 8-bit signed integer"),
            MethodMetadata::new("toI16", "int", "Convert to 16-bit signed integer"),
            MethodMetadata::new("toI32", "int", "Convert to 32-bit signed integer"),
            MethodMetadata::new("toI64", "int", "Convert to 64-bit signed integer"),
            MethodMetadata::new("toU8", "int", "Convert to 8-bit unsigned integer"),
            MethodMetadata::new("toU16", "int", "Convert to 16-bit unsigned integer"),
            MethodMetadata::new("toU32", "int", "Convert to 32-bit unsigned integer"),
            MethodMetadata::new("toU64", "int", "Convert to 64-bit unsigned integer"),
            MethodMetadata::new("toF32", "float", "Convert to 32-bit float"),
            MethodMetadata::new("toF64", "float", "Convert to 64-bit float (identity)"),
            MethodMetadata::new("toFloat", "float", "Convert to float (identity)"),
            MethodMetadata::new("toDecimal", "decimal", "Convert to decimal"),
            // Rounding methods
            MethodMetadata::new("floor", "int", "Round down to nearest integer"),
            MethodMetadata::new("ceil", "int", "Round up to nearest integer"),
            MethodMetadata::new("round", "int", "Round to nearest integer"),
            // Mathematical methods
            MethodMetadata::new("abs", "float", "Absolute value"),
            MethodMetadata::new("sqrt", "float", "Square root"),
            MethodMetadata::new("sign", "int", "Sign of number (-1, 0, or 1)"),
            MethodMetadata::new("pow", "float", "Raise to power")
                .with_params(vec![ParamMetadata::new("exponent", "float|int")]),
            MethodMetadata::new("clamp", "float", "Clamp value between min and max").with_params(
                vec![
                    ParamMetadata::new("min", "float|int"),
                    ParamMetadata::new("max", "float|int"),
                ],
            ),
            // Trigonometric methods
            MethodMetadata::new("sin", "float", "Sine"),
            MethodMetadata::new("cos", "float", "Cosine"),
            MethodMetadata::new("tan", "float", "Tangent"),
            MethodMetadata::new("asin", "float", "Arc sine"),
            MethodMetadata::new("acos", "float", "Arc cosine"),
            MethodMetadata::new("atan", "float", "Arc tangent"),
            // Logarithmic/exponential methods
            MethodMetadata::new("exp", "float", "Exponential (e^x)"),
            MethodMetadata::new("ln", "float", "Natural logarithm"),
            MethodMetadata::new("log10", "float", "Base-10 logarithm"),
            MethodMetadata::new("log2", "float", "Base-2 logarithm"),
            // Predicate methods
            MethodMetadata::new("isNaN", "bool", "Check if value is NaN"),
            MethodMetadata::new("isInfinite", "bool", "Check if value is infinite"),
            MethodMetadata::new("isFinite", "bool", "Check if value is finite"),
            MethodMetadata::new("isPositive", "bool", "Check if value is positive"),
            MethodMetadata::new("isNegative", "bool", "Check if value is negative"),
            MethodMetadata::new("isZero", "bool", "Check if value is zero"),
        ])
        .with_static_methods(vec![
            MethodMetadata::new("parse", "float", "Parse string to float")
                .with_params(vec![ParamMetadata::new("value", "str")]),
            MethodMetadata::new(
                "tryParse",
                "float|null",
                "Try to parse string, returns null on failure",
            )
            .with_params(vec![ParamMetadata::new("value", "str")]),
            MethodMetadata::new("isNaN", "bool", "Check if value is NaN")
                .with_params(vec![ParamMetadata::new("value", "float")]),
            MethodMetadata::new("isInfinite", "bool", "Check if value is infinite")
                .with_params(vec![ParamMetadata::new("value", "float")]),
            MethodMetadata::new("isFinite", "bool", "Check if value is finite")
                .with_params(vec![ParamMetadata::new("value", "float")]),
        ])
        .with_static_properties(vec![
            PropertyMetadata::new("maxValue", "float", "Maximum float value (f64::MAX)").readonly(),
            PropertyMetadata::new("minValue", "float", "Minimum float value (f64::MIN)").readonly(),
            PropertyMetadata::new("infinity", "float", "Positive infinity").readonly(),
            PropertyMetadata::new("negativeInfinity", "float", "Negative infinity").readonly(),
            PropertyMetadata::new("nan", "float", "Not a Number").readonly(),
            PropertyMetadata::new("epsilon", "float", "Machine epsilon").readonly(),
        ])
    }
}

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
            // Conversion methods
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

            // Rounding methods
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

            // Mathematical methods
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

            // Trigonometric methods
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

            // Logarithmic/exponential methods
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

            // Predicate methods
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

// Alias for backwards compatibility
pub type FloatType = Float64Type;
