use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// U8Type - 8-bit unsigned integer
// ============================================================================

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
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = match value {
            RuntimeValue::Int(i) => i.value as u8,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected u8, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on u8", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<u8>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as u8", s.value),
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
                format!("Static method '{}' not found on u8 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u8::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u8::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on u8 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for U16Type {
    fn type_name(&self) -> &str {
        "u16"
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
            RuntimeValue::Int(i) => i.value as u16,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected u16, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on u16", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<u16>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as u16", s.value),
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
                format!("Static method '{}' not found on u16 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u16::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u16::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on u16 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for U32Type {
    fn type_name(&self) -> &str {
        "u32"
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
            RuntimeValue::Int(i) => i.value as u32,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected u32, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on u32", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<u32>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as u32", s.value),
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
                format!("Static method '{}' not found on u32 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u32::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u32::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on u32 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for U64Type {
    fn type_name(&self) -> &str {
        "u64"
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
            RuntimeValue::Int(i) => i.value as u64,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected u64, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on u64", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<u64>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as u64", s.value),
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
                format!("Static method '{}' not found on u64 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(u64::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(u64::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on u64 type", property),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
