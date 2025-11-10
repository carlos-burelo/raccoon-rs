use crate::error::RaccoonError;
use crate::runtime::{BoolValue, RuntimeValue};
use crate::tokens::Position;

pub fn equal(
    left: RuntimeValue,
    right: RuntimeValue,
    _position: Position,
    _file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match (&left, &right) {
        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value == b.value,
        (RuntimeValue::Int(a), RuntimeValue::Float(b)) => a.value as f64 == b.value,
        (RuntimeValue::Float(a), RuntimeValue::Int(b)) => a.value == b.value as f64,
        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value == b.value,
        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value == b.value,
        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a.value == b.value,
        (RuntimeValue::Null(_), RuntimeValue::Null(_)) => true,
        _ => false,
    };
    Ok(RuntimeValue::Bool(BoolValue::new(result)))
}

pub fn not_equal(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let equal_result = equal(left, right, position, file)?;
    match equal_result {
        RuntimeValue::Bool(b) => Ok(RuntimeValue::Bool(BoolValue::new(!b.value))),
        _ => unreachable!("equal() should always return bool"),
    }
}

pub fn less_than(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match (&left, &right) {
        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value < b.value,
        (RuntimeValue::Int(a), RuntimeValue::Float(b)) => (a.value as f64) < b.value,
        (RuntimeValue::Float(a), RuntimeValue::Int(b)) => a.value < b.value as f64,
        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value < b.value,
        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value < b.value,
        _ => {
            return Err(RaccoonError::new(
                "Invalid operands for less than comparison".to_string(),
                position,
                file.clone(),
            ))
        }
    };
    Ok(RuntimeValue::Bool(BoolValue::new(result)))
}

pub fn less_or_equal(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match (&left, &right) {
        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value <= b.value,
        (RuntimeValue::Int(a), RuntimeValue::Float(b)) => (a.value as f64) <= b.value,
        (RuntimeValue::Float(a), RuntimeValue::Int(b)) => a.value <= b.value as f64,
        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value <= b.value,
        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value <= b.value,
        _ => {
            return Err(RaccoonError::new(
                "Invalid operands for less or equal comparison".to_string(),
                position,
                file.clone(),
            ))
        }
    };
    Ok(RuntimeValue::Bool(BoolValue::new(result)))
}

pub fn greater_than(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match (&left, &right) {
        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value > b.value,
        (RuntimeValue::Int(a), RuntimeValue::Float(b)) => (a.value as f64) > b.value,
        (RuntimeValue::Float(a), RuntimeValue::Int(b)) => a.value > b.value as f64,
        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value > b.value,
        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value > b.value,
        _ => {
            return Err(RaccoonError::new(
                "Invalid operands for greater than comparison".to_string(),
                position,
                file.clone(),
            ))
        }
    };
    Ok(RuntimeValue::Bool(BoolValue::new(result)))
}

pub fn greater_or_equal(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match (&left, &right) {
        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value >= b.value,
        (RuntimeValue::Int(a), RuntimeValue::Float(b)) => (a.value as f64) >= b.value,
        (RuntimeValue::Float(a), RuntimeValue::Int(b)) => a.value >= b.value as f64,
        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value >= b.value,
        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value >= b.value,
        _ => {
            return Err(RaccoonError::new(
                "Invalid operands for greater or equal comparison".to_string(),
                position,
                file.clone(),
            ))
        }
    };
    Ok(RuntimeValue::Bool(BoolValue::new(result)))
}
