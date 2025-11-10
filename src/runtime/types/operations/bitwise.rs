use crate::error::RaccoonError;
use crate::runtime::{IntValue, RuntimeValue};
use crate::tokens::Position;

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
