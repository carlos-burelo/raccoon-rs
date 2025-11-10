/// Bitwise operations module
/// Centralizes bitwise operations: and, or, xor, left shift, right shift, unsigned right shift
use crate::error::RaccoonError;
use crate::runtime::{IntValue, RuntimeValue};
use crate::tokens::Position;

/// Bitwise AND operation: Int & Int
pub fn bitwise_and(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value & r.value)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for bitwise AND (only Int & Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Bitwise OR operation: Int | Int
pub fn bitwise_or(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value | r.value)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for bitwise OR (only Int | Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Bitwise XOR operation: Int ^ Int
pub fn bitwise_xor(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value ^ r.value)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for bitwise XOR (only Int ^ Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Left shift operation: Int << Int
pub fn left_shift(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value < 0 || r.value > 63 {
                return Err(RaccoonError::new(
                    "Shift amount must be between 0 and 63".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Int(IntValue::new(l.value << r.value)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for left shift (only Int << Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Right shift operation: Int >> Int (arithmetic, sign-extending)
pub fn right_shift(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value < 0 || r.value > 63 {
                return Err(RaccoonError::new(
                    "Shift amount must be between 0 and 63".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Int(IntValue::new(l.value >> r.value)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for right shift (only Int >> Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Unsigned right shift operation: Int >>> Int (zero-filling)
pub fn unsigned_right_shift(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value < 0 || r.value > 63 {
                return Err(RaccoonError::new(
                    "Shift amount must be between 0 and 63".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Int(IntValue::new(
                (l.value as u64 >> r.value) as i64,
            )))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for unsigned right shift (only Int >>> Int is supported)".to_string(),
            position,
            file.clone(),
        )),
    }
}
