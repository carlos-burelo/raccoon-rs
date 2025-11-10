use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct U8Type;

#[async_trait]
impl TypeHandler for U8Type {
    fn type_name(&self) -> &str {
        "u8"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as u8;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            _ => Err(method_not_found_error("u8", method, position, file)),
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
                match s.trim().parse::<u8>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as u8", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("u8", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u8::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u8::MIN as i64))),
            _ => Err(property_not_found_error("u8", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

pub struct U16Type;

#[async_trait]
impl TypeHandler for U16Type {
    fn type_name(&self) -> &str {
        "u16"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as u16;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            _ => Err(method_not_found_error("u16", method, position, file)),
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
                match s.trim().parse::<u16>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as u16", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("u16", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u16::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u16::MIN as i64))),
            _ => Err(property_not_found_error("u16", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

pub struct U32Type;

#[async_trait]
impl TypeHandler for U32Type {
    fn type_name(&self) -> &str {
        "u32"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as u32;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            _ => Err(method_not_found_error("u32", method, position, file)),
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
                match s.trim().parse::<u32>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as u32", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("u32", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u32::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u32::MIN as i64))),
            _ => Err(property_not_found_error("u32", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

pub struct U64Type;

#[async_trait]
impl TypeHandler for U64Type {
    fn type_name(&self) -> &str {
        "u64"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = extract_int(value, "this", position, file.clone())? as u64;

        match method {
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(num.to_string())))
            }
            "toInt" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(num as i64)))
            }
            _ => Err(method_not_found_error("u64", method, position, file)),
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
                match s.trim().parse::<u64>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as u64", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error("u64", method, position, file)),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u64::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u64::MIN as i64))),
            _ => Err(property_not_found_error("u64", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
