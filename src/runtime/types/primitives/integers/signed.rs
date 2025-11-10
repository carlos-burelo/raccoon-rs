use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{
    BoolValue, DecimalValue, FloatValue, IntValue, NullValue, RuntimeValue, StrValue,
};
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

pub struct I8Type;

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

pub struct I16Type;

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

pub struct I32Type;

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

pub struct I64Type;

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
