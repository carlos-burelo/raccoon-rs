/// Arithmetic operations module - REFACTORED VERSION
/// Centralizes all arithmetic operations using binary_op! macro
/// This eliminates code duplication across add, subtract, multiply, divide, modulo, exponent

use crate::error::RaccoonError;
use crate::runtime::{CallStack, FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;

/// Addition operation: handles Int+Int, Float+Float, Int+Float, Float+Int, Str+Any
pub fn add(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    match (&left, &right) {
        (Int(l), Int(r)) => Ok(Int(IntValue::new(l.value + r.value))),
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value + r.value))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new(l.value as f64 + r.value))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value + r.value as f64))),
        (Str(l), Str(r)) => Ok(Str(StrValue::new(format!("{}{}", l.value, r.value)))),
        // String concatenation with any type
        (Str(l), r) => Ok(Str(StrValue::new(format!("{}{}", l.value, r.to_string())))),
        (l, Str(r)) => Ok(Str(StrValue::new(format!("{}{}", l.to_string(), r.value)))),
        _ => Err(RaccoonError::new(
            format!("Invalid operands for addition: {} and {}", left.get_name(), right.get_name()),
            position,
            file.clone(),
        )),
    }
}

/// Subtraction operation: Int-Int, Float-Float, Int-Float, Float-Int
pub fn subtract(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    match (left, right) {
        (Int(l), Int(r)) => Ok(Int(IntValue::new(l.value - r.value))),
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value - r.value))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new(l.value as f64 - r.value))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value - r.value as f64))),
        (l, r) => Err(RaccoonError::new(
            format!("Invalid operands for subtraction: {} and {}", l.get_name(), r.get_name()),
            position,
            file.clone(),
        )),
    }
}

/// Multiplication operation: Int*Int, Float*Float, Int*Float, Float*Int
pub fn multiply(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    match (left, right) {
        (Int(l), Int(r)) => Ok(Int(IntValue::new(l.value * r.value))),
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value * r.value))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new(l.value as f64 * r.value))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value * r.value as f64))),
        (l, r) => Err(RaccoonError::new(
            format!("Invalid operands for multiplication: {} and {}", l.get_name(), r.get_name()),
            position,
            file.clone(),
        )),
    }
}

/// Division operation: Int/Int, Float/Float, Int/Float, Float/Int
pub fn divide(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
    call_stack: &CallStack,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    // Check for division by zero first
    let is_zero = match &right {
        Int(i) => i.value == 0,
        Float(f) => f.value == 0.0,
        _ => false,
    };

    if is_zero {
        return Err(RaccoonError::with_call_stack(
            "Division by zero".to_string(),
            position,
            file.clone(),
            call_stack.clone(),
        ));
    }

    match (left, right) {
        (Int(l), Int(r)) => Ok(Float(FloatValue::new(l.value as f64 / r.value as f64))),
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value / r.value))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new(l.value as f64 / r.value))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value / r.value as f64))),
        (l, r) => Err(RaccoonError::new(
            format!("Invalid operands for division: {} and {}", l.get_name(), r.get_name()),
            position,
            file.clone(),
        )),
    }
}

/// Modulo operation: Int % Int, Float % Float, Int % Float, Float % Int
pub fn modulo(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    // Check for modulo by zero
    let is_zero = match &right {
        Int(i) => i.value == 0,
        Float(f) => f.value == 0.0,
        _ => false,
    };

    if is_zero {
        return Err(RaccoonError::new(
            "Modulo by zero".to_string(),
            position,
            file.clone(),
        ));
    }

    match (left, right) {
        (Int(l), Int(r)) => Ok(Int(IntValue::new(l.value % r.value))),
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value % r.value))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new(l.value as f64 % r.value))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value % r.value as f64))),
        (l, r) => Err(RaccoonError::new(
            format!("Invalid operands for modulo: {} and {}", l.get_name(), r.get_name()),
            position,
            file.clone(),
        )),
    }
}

/// Exponentiation operation: x^y
pub fn exponent(
    left: RuntimeValue,
    right: RuntimeValue,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    use RuntimeValue::*;

    match (left, right) {
        (Int(l), Int(r)) => {
            if r.value < 0 {
                Ok(Float(FloatValue::new((l.value as f64).powf(r.value as f64))))
            } else {
                Ok(Int(IntValue::new(l.value.pow(r.value as u32))))
            }
        }
        (Float(l), Float(r)) => Ok(Float(FloatValue::new(l.value.powf(r.value)))),
        (Int(l), Float(r)) => Ok(Float(FloatValue::new((l.value as f64).powf(r.value)))),
        (Float(l), Int(r)) => Ok(Float(FloatValue::new(l.value.powf(r.value as f64)))),
        (l, r) => Err(RaccoonError::new(
            format!("Invalid operands for exponentiation: {} and {}", l.get_name(), r.get_name()),
            position,
            file.clone(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        let left = RuntimeValue::Int(IntValue::new(5));
        let right = RuntimeValue::Int(IntValue::new(3));
        let result = add(left, right, Position::default(), &None).unwrap();

        match result {
            RuntimeValue::Int(v) => assert_eq!(v.value, 8),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_add_string_concat() {
        let left = RuntimeValue::Str(StrValue::new("Hello".to_string()));
        let right = RuntimeValue::Str(StrValue::new(" World".to_string()));
        let result = add(left, right, Position::default(), &None).unwrap();

        match result {
            RuntimeValue::Str(v) => assert_eq!(v.value, "Hello World"),
            _ => panic!("Expected Str"),
        }
    }

    #[test]
    fn test_divide_by_zero() {
        let left = RuntimeValue::Int(IntValue::new(5));
        let right = RuntimeValue::Int(IntValue::new(0));
        let call_stack = CallStack::new();
        let result = divide(left, right, Position::default(), &None, &call_stack);

        assert!(result.is_err());
    }
}
