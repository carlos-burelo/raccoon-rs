/// Refactored unsigned integer types using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{
    MethodMetadata, ParamMetadata, PropertyMetadata, TypeMetadata,
};
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// U8Type - 8-bit unsigned integer
// ============================================================================

pub struct U8Type;

impl U8Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("u8", "8-bit unsigned integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to u8",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum u8 value (255)").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum u8 value (0)").readonly(),
            ])
    }
}

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

// ============================================================================
// U16Type - 16-bit unsigned integer
// ============================================================================

pub struct U16Type;

impl U16Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("u16", "16-bit unsigned integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to u16",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum u16 value (65535)").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum u16 value (0)").readonly(),
            ])
    }
}

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

// ============================================================================
// U32Type - 32-bit unsigned integer
// ============================================================================

pub struct U32Type;

impl U32Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("u32", "32-bit unsigned integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to u32",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum u32 value (4294967295)")
                    .readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum u32 value (0)").readonly(),
            ])
    }
}

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

// ============================================================================
// U64Type - 64-bit unsigned integer
// ============================================================================

pub struct U64Type;

impl U64Type {
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("u64", "64-bit unsigned integer type")
            .with_instance_methods(vec![
                MethodMetadata::new("toStr", "str", "Convert to string"),
                MethodMetadata::new("toInt", "int", "Convert to int (i64)"),
            ])
            .with_static_methods(vec![MethodMetadata::new(
                "parse",
                "int",
                "Parse string to u64",
            )
            .with_params(vec![ParamMetadata::new("value", "str")])])
            .with_static_properties(vec![
                PropertyMetadata::new("maxValue", "int", "Maximum u64 value").readonly(),
                PropertyMetadata::new("minValue", "int", "Minimum u64 value (0)").readonly(),
            ])
    }
}

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
