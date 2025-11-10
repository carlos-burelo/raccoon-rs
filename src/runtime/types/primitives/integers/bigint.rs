/// Refactored BigIntType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
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
        let bigint = extract_bigint(value, "this", position, file.clone())?;

        match method {
            // Conversion methods
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(format!("{}n", bigint))))
            }
            "toString" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(format!("{}", bigint))))
            }

            // Arithmetic methods
            "add" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_bigint(&args[0], "other", position, file)?;
                Ok(RuntimeValue::BigInt(BigIntValue::new(bigint + other)))
            }
            "subtract" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_bigint(&args[0], "other", position, file)?;
                Ok(RuntimeValue::BigInt(BigIntValue::new(bigint - other)))
            }
            "multiply" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_bigint(&args[0], "other", position, file)?;
                Ok(RuntimeValue::BigInt(BigIntValue::new(bigint * other)))
            }
            "divide" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_bigint(&args[0], "other", position, file.clone())?;
                if other == 0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(RuntimeValue::BigInt(BigIntValue::new(bigint / other)))
            }

            // Mathematical methods
            "abs" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::BigInt(BigIntValue::new(bigint.abs())))
            }

            _ => Err(method_not_found_error("bigint", method, position, file)),
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
                let trimmed = s.trim().trim_end_matches('n');
                match trimmed.parse::<i128>() {
                    Ok(num) => Ok(RuntimeValue::BigInt(BigIntValue::new(num))),
                    Err(_) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as bigint", s),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error(
                "bigint", method, position, file,
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
