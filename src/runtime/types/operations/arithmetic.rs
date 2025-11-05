/// Arithmetic operations module
/// Centralizes all arithmetic operations: add, subtract, multiply, divide, modulo, exponent

use crate::error::RaccoonError;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;

/// Addition operation: handles Int+Int, Float+Float, Int+Float, Float+Int, Str+Any
pub fn add(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (&left, &right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value + r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value)))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 + r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value as f64)))
        }
        (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
            Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.value, r.value))))
        }
        // String concatenation with any type on either side
        (RuntimeValue::Str(l), r) => {
            Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.value, r.to_string()))))
        }
        (l, RuntimeValue::Str(r)) => {
            Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.to_string(), r.value))))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for addition".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Subtraction operation: handles Int-Int, Float-Float, Int-Float, Float-Int
pub fn subtract(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value - r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value)))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 - r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value as f64)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for subtraction".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Multiplication operation: handles Int*Int, Float*Float, Int*Float, Float*Int
pub fn multiply(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Int(IntValue::new(l.value * r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value)))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 * r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value as f64)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for multiplication".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Division operation: handles Int/Int, Float/Float, Int/Float, Float/Int
/// Returns Float, checks for division by zero
pub fn divide(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value == 0 {
                return Err(RaccoonError::new(
                    "Division by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 / r.value as f64)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            if r.value == 0.0 {
                return Err(RaccoonError::new(
                    "Division by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value)))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            if r.value == 0.0 {
                return Err(RaccoonError::new(
                    "Division by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 / r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            if r.value == 0 {
                return Err(RaccoonError::new(
                    "Division by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value as f64)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for division".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Modulo operation: Int % Int, Float % Float, Int % Float, Float % Int
/// Checks for modulo by zero
pub fn modulo(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value == 0 {
                return Err(RaccoonError::new(
                    "Modulo by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Int(IntValue::new(l.value % r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            if r.value == 0.0 {
                return Err(RaccoonError::new(
                    "Modulo by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value % r.value)))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            if r.value == 0.0 {
                return Err(RaccoonError::new(
                    "Modulo by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 % r.value)))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            if r.value == 0 {
                return Err(RaccoonError::new(
                    "Modulo by zero".to_string(),
                    position,
                    file.clone(),
                ));
            }
            Ok(RuntimeValue::Float(FloatValue::new(l.value % r.value as f64)))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for modulo".to_string(),
            position,
            file.clone(),
        )),
    }
}

/// Exponentiation operation: x^y
/// Supports Int^Int, Float^Float, Int^Float, Float^Int
pub fn exponent(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match (left, right) {
        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
            if r.value < 0 {
                // Negative exponent returns float
                Ok(RuntimeValue::Float(FloatValue::new(
                    (l.value as f64).powf(r.value as f64),
                )))
            } else {
                Ok(RuntimeValue::Int(IntValue::new(
                    l.value.pow(r.value as u32),
                )))
            }
        }
        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(l.value.powf(r.value))))
        }
        (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(
                (l.value as f64).powf(r.value),
            )))
        }
        (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
            Ok(RuntimeValue::Float(FloatValue::new(
                l.value.powf(r.value as f64),
            )))
        }
        _ => Err(RaccoonError::new(
            "Invalid operands for exponentiation".to_string(),
            position,
            file.clone(),
        )),
    }
}
