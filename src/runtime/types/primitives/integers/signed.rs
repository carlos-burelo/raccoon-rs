/// Refactored integer types using helpers and metadata system
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
// IntType - Generic signed integer (i64 by default)
// ============================================================================

pub struct IntType;

impl IntType {
    /// Returns complete type metadata with all methods and properties
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("int", "64-bit signed integer type with arithmetic methods")
            .with_instance_methods(vec![
                // Conversion methods
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (identity)"),
                MethodMetadata::new("toI8", "int", "Convert to 8-bit signed integer"),
                MethodMetadata::new("toI16", "int", "Convert to 16-bit signed integer"),
                MethodMetadata::new("toI32", "int", "Convert to 32-bit signed integer"),
                MethodMetadata::new("toI64", "int", "Convert to 64-bit signed integer"),
                MethodMetadata::new("toU8", "int", "Convert to 8-bit unsigned integer"),
                MethodMetadata::new("toU16", "int", "Convert to 16-bit unsigned integer"),
                MethodMetadata::new("toU32", "int", "Convert to 32-bit unsigned integer"),
                MethodMetadata::new("toU64", "int", "Convert to 64-bit unsigned integer"),
                MethodMetadata::new("toF32", "float", "Convert to 32-bit float"),
                MethodMetadata::new("toF64", "float", "Convert to 64-bit float"),
                MethodMetadata::new("toFloat", "float", "Convert to float"),
                MethodMetadata::new("toDecimal", "decimal", "Convert to decimal"),
                // Mathematical methods
                MethodMetadata::new("abs", "int", "Absolute value"),
                MethodMetadata::new("sign", "int", "Sign of number (-1, 0, or 1)"),
                MethodMetadata::new("pow", "int", "Raise to power")
                    .with_params(vec![ParamMetadata::new("exponent", "int")]),
                MethodMetadata::new("clamp", "int", "Clamp value between min and max").with_params(
                    vec![
                        ParamMetadata::new("min", "int"),
                        ParamMetadata::new("max", "int"),
                    ],
                ),
                // Predicate methods
                MethodMetadata::new("isEven", "bool", "Check if number is even"),
                MethodMetadata::new("isOdd", "bool", "Check if number is odd"),
                MethodMetadata::new("isPositive", "bool", "Check if number is positive"),
                MethodMetadata::new("isNegative", "bool", "Check if number is negative"),
                MethodMetadata::new("isZero", "bool", "Check if number is zero"),
            ])
            .with_static_methods(vec![
                MethodMetadata::new("parse", "int", "Parse string to integer")
                    .with_params(vec![ParamMetadata::new("value", "str")]),
                MethodMetadata::new(
                    "tryParse",
                    "int|null",
                    "Try to parse string, returns null on failure",
                )
                .with_params(vec![ParamMetadata::new("value", "str")]),
                MethodMetadata::new("compare", "int", "Compare two integers").with_params(vec![
                    ParamMetadata::new("a", "int"),
                    ParamMetadata::new("b", "int"),
                ]),
            ])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum int value (i64::MAX)").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum int value (i64::MIN)").readonly(),
            ])
    }
}

#[async_trait]
impl TypeHandler for IntType {
    fn type_name(&self) -> &str {
        "int"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())?;

        match method {
            // Conversion methods
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num)))
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
                Ok(RuntimeValue::Int(IntValue::new(num)))
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
                Ok(RuntimeValue::Float(FloatValue::new(num as f64)))
            }
            "toFloat" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Float(FloatValue::new(num as f64)))
            }
            "toDecimal" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Decimal(DecimalValue::new(num as f64)))
            }

            // Mathematical methods
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.abs())))
            }
            "sign" => {
                require_args(&args, 0, method, position, file)?;
                let sign = if num > 0 {
                    1
                } else if num < 0 {
                    -1
                } else {
                    0
                };
                Ok(RuntimeValue::Int(IntValue::new(sign)))
            }
            "pow" => {
                require_args(&args, 1, method, position, file.clone())?;
                let exp = extract_int(&args[0], "exponent", position, file.clone())?;
                if exp < 0 {
                    return Err(RaccoonError::new(
                        "pow exponent must be non-negative".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(RuntimeValue::Int(IntValue::new(num.pow(exp as u32))))
            }
            "clamp" => {
                require_args(&args, 2, method, position, file.clone())?;
                let min_val = extract_int(&args[0], "min", position, file.clone())?;
                let max_val = extract_int(&args[1], "max", position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(
                    num.max(min_val).min(max_val),
                )))
            }

            // Predicate methods
            "isEven" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num % 2 == 0)))
            }
            "isOdd" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num % 2 != 0)))
            }
            "isPositive" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num > 0)))
            }
            "isNegative" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num < 0)))
            }
            "isZero" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(num == 0)))
            }

            _ => Err(method_not_found_error("int", method, position, file)),
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
                match s.trim().parse::<i64>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as int", s),
                        position,
                        file,
                    )),
                }
            }
            "tryParse" => {
                require_args(&args, 1, method, position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file)?;
                match s.trim().parse::<i64>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                    Err(_) => Ok(RuntimeValue::Null(NullValue::new())),
                }
            }
            "compare" => {
                require_args(&args, 2, method, position, file.clone())?;
                let a = extract_int(&args[0], "a", position, file.clone())?;
                let b = extract_int(&args[1], "b", position, file)?;
                let result = if a < b {
                    -1
                } else if a > b {
                    1
                } else {
                    0
                };
                Ok(RuntimeValue::Int(IntValue::new(result)))
            }
            _ => Err(static_method_not_found_error("int", method, position, file)),
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
            _ => Err(property_not_found_error("int", property, position, file)),
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
                | "sign"
                | "isEven"
                | "isOdd"
                | "isPositive"
                | "isNegative"
                | "isZero"
                | "clamp"
                | "pow"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse" | "tryParse" | "compare")
    }
}

// ============================================================================
// I8Type - 8-bit signed integer
// ============================================================================

pub struct I8Type;

impl I8Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("i8", "8-bit signed integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
                MethodMetadata::new("abs", "int", "Absolute value"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to i8",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum i8 value (127)").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum i8 value (-128)").readonly(),
            ])
    }
}

#[async_trait]
impl TypeHandler for I8Type {
    fn type_name(&self) -> &str {
        "i8"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as i8;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64)))
            }
            _ => Err(method_not_found_error("i8", method, position, file)),
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
                match s.trim().parse::<i8>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as i8", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("i8", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i8::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i8::MIN as i64))),
            _ => Err(property_not_found_error("i8", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

// ============================================================================
// I16Type - 16-bit signed integer
// ============================================================================

pub struct I16Type;

impl I16Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("i16", "16-bit signed integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
                MethodMetadata::new("abs", "int", "Absolute value"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to i16",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum i16 value (32767)").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum i16 value (-32768)").readonly(),
            ])
    }
}

#[async_trait]
impl TypeHandler for I16Type {
    fn type_name(&self) -> &str {
        "i16"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as i16;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64)))
            }
            _ => Err(method_not_found_error("i16", method, position, file)),
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
                match s.trim().parse::<i16>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as i16", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("i16", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i16::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i16::MIN as i64))),
            _ => Err(property_not_found_error("i16", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

// ============================================================================
// I32Type - 32-bit signed integer
// ============================================================================

pub struct I32Type;

impl I32Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("i32", "32-bit signed integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
                MethodMetadata::new("abs", "int", "Absolute value"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to i32",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum i32 value (2147483647)")
                    .readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum i32 value (-2147483648)")
                    .readonly(),
            ])
    }
}

#[async_trait]
impl TypeHandler for I32Type {
    fn type_name(&self) -> &str {
        "i32"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as i32;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64)))
            }
            _ => Err(method_not_found_error("i32", method, position, file)),
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
                match s.trim().parse::<i32>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as i32", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("i32", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i32::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i32::MIN as i64))),
            _ => Err(property_not_found_error("i32", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

// ============================================================================
// I64Type - 64-bit signed integer
// ============================================================================

pub struct I64Type;

impl I64Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("i64", "64-bit signed integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (identity)"),
                MethodMetadata::new("abs", "int", "Absolute value"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to i64",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum i64 value").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum i64 value").readonly(),
            ])
    }
}

#[async_trait]
impl TypeHandler for I64Type {
    fn type_name(&self) -> &str {
        "i64"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())?;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num)))
            }
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num.abs())))
            }
            _ => Err(method_not_found_error("i64", method, position, file)),
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
                match s.trim().parse::<i64>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as i64", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("i64", method, position, file)),
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
            _ => Err(property_not_found_error("i64", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
