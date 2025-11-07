use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{DecimalValue, FloatValue, IntValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// IntType - Generic signed integer (i64 by default)
// ============================================================================

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
        let num = match value {
            RuntimeValue::Int(i) => i.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected int, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num))),
            "toI8" => Ok(RuntimeValue::Int(IntValue::new(num as i8 as i64))),
            "toI16" => Ok(RuntimeValue::Int(IntValue::new(num as i16 as i64))),
            "toI32" => Ok(RuntimeValue::Int(IntValue::new(num as i32 as i64))),
            "toI64" => Ok(RuntimeValue::Int(IntValue::new(num))),
            "toU8" => Ok(RuntimeValue::Int(IntValue::new(num as u8 as i64))),
            "toU16" => Ok(RuntimeValue::Int(IntValue::new(num as u16 as i64))),
            "toU32" => Ok(RuntimeValue::Int(IntValue::new(num as u32 as i64))),
            "toU64" => Ok(RuntimeValue::Int(IntValue::new(num as u64 as i64))),
            "toF32" => Ok(RuntimeValue::Float(FloatValue::new(num as f32 as f64))),
            "toF64" => Ok(RuntimeValue::Float(FloatValue::new(num as f64))),
            "toDecimal" => Ok(RuntimeValue::Decimal(DecimalValue::new(num as f64))),
            "toFloat" => Ok(RuntimeValue::Float(FloatValue::new(num as f64))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs()))),
            "sign" => Ok(RuntimeValue::Int(IntValue::new(
                if num > 0 {
                    1
                } else if num < 0 {
                    -1
                } else {
                    0
                },
            ))),
            "isEven" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                num % 2 == 0,
            ))),
            "isOdd" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                num % 2 != 0,
            ))),
            "isPositive" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num > 0))),
            "isNegative" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num < 0))),
            "isZero" => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(num == 0))),
            "clamp" => {
                if args.len() != 2 {
                    return Err(RaccoonError::new(
                        "clamp requires 2 arguments (min, max)".to_string(),
                        position,
                        file,
                    ));
                }
                let min_val = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "clamp requires integer arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let max_val = match &args[1] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "clamp requires integer arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                Ok(RuntimeValue::Int(IntValue::new(num.max(min_val).min(max_val))))
            }
            "pow" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "pow requires 1 argument (exponent)".to_string(),
                        position,
                        file,
                    ));
                }
                let exp = match &args[0] {
                    RuntimeValue::Int(i) => {
                        if i.value < 0 {
                            return Err(RaccoonError::new(
                                "pow exponent must be non-negative".to_string(),
                                position,
                                file,
                            ));
                        }
                        i.value as u32
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "pow requires integer argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                Ok(RuntimeValue::Int(IntValue::new(num.pow(exp))))
            }
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i64>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as int", s.value),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i64>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                        Err(_) => Ok(RuntimeValue::Null(NullValue::new())),
                    },
                    _ => Ok(RuntimeValue::Null(NullValue::new())),
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
                    _ => {
                        return Err(RaccoonError::new(
                            "compare requires int arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let b = match &args[1] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "compare requires int arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                Ok(RuntimeValue::Int(IntValue::new(if a < b {
                    -1
                } else if a > b {
                    1
                } else {
                    0
                })))
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on int type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MAX))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MIN))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on int type", property),
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

#[async_trait]
impl TypeHandler for I8Type {
    fn type_name(&self) -> &str {
        "i8"
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
            RuntimeValue::Int(i) => i.value as i8,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected i8, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on i8", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i8>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as i8", s.value),
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
                format!("Static method '{}' not found on i8 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i8::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i8::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on i8 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for I16Type {
    fn type_name(&self) -> &str {
        "i16"
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
            RuntimeValue::Int(i) => i.value as i16,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected i16, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on i16", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i16>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as i16", s.value),
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
                format!("Static method '{}' not found on i16 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i16::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i16::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on i16 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for I32Type {
    fn type_name(&self) -> &str {
        "i32"
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
            RuntimeValue::Int(i) => i.value as i32,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected i32, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs() as i64))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on i32", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i32>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num as i64))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as i32", s.value),
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
                format!("Static method '{}' not found on i32 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i32::MAX as i64))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i32::MIN as i64))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on i32 type", property),
                position,
                file,
            )),
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

#[async_trait]
impl TypeHandler for I64Type {
    fn type_name(&self) -> &str {
        "i64"
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
            RuntimeValue::Int(i) => i.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected i64, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on i64", method),
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
                    RuntimeValue::Str(s) => match s.value.trim().parse::<i64>() {
                        Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num))),
                        Err(_) => Err(RaccoonError::new(
                            format!("Failed to parse '{}' as i64", s.value),
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
                format!("Static method '{}' not found on i64 type", method),
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
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MAX))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(i64::MIN))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on i64 type", property),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
