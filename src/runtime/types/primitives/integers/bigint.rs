use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BigIntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// BigIntType - Arbitrary precision integer
// ============================================================================

pub struct BigIntType;

#[async_trait]
impl TypeHandler for BigIntType {
    fn type_name(&self) -> &str {
        "bigint"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let bigint = match value {
            RuntimeValue::BigInt(b) => &b.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected bigint, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(format!("{}n", bigint)))),
            "toString" => Ok(RuntimeValue::Str(StrValue::new(format!("{}", bigint)))),
            "add" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "add requires 1 argument (bigint)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::BigInt(other) => {
                        Ok(RuntimeValue::BigInt(BigIntValue::new(bigint + other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "add requires bigint argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "subtract" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "subtract requires 1 argument (bigint)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::BigInt(other) => {
                        Ok(RuntimeValue::BigInt(BigIntValue::new(bigint - other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "subtract requires bigint argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "multiply" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "multiply requires 1 argument (bigint)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::BigInt(other) => {
                        Ok(RuntimeValue::BigInt(BigIntValue::new(bigint * other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "multiply requires bigint argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "divide" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "divide requires 1 argument (bigint)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::BigInt(other) => {
                        if other.value == 0 {
                            return Err(RaccoonError::new(
                                "Division by zero".to_string(),
                                position,
                                file,
                            ));
                        }
                        Ok(RuntimeValue::BigInt(BigIntValue::new(bigint / other.value)))
                    }
                    _ => Err(RaccoonError::new(
                        "divide requires bigint argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "abs" => Ok(RuntimeValue::BigInt(BigIntValue::new(bigint.abs()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on bigint", method),
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
                        let trimmed = s.value.trim().trim_end_matches('n');
                        match trimmed.parse::<i128>() {
                            Ok(num) => Ok(RuntimeValue::BigInt(BigIntValue::new(num))),
                            Err(_) => Err(RaccoonError::new(
                                format!("Failed to parse '{}' as bigint", s.value),
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
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on bigint type", method),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toStr" | "toString" | "add" | "subtract" | "multiply" | "divide" | "abs"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}
