/// Helper functions for type system operations
/// Provides reusable utilities for argument validation, type extraction, and error handling

use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;

/// Validates that the number of arguments matches the expected count
pub fn require_args(
    args: &[RuntimeValue],
    expected: usize,
    method: &str,
    position: Position,
    file: Option<String>,
) -> Result<(), RaccoonError> {
    if args.len() != expected {
        return Err(RaccoonError::new(
            format!("{} requires {} argument(s), got {}", method, expected, args.len()),
            position,
            file,
        ));
    }
    Ok(())
}

/// Validates that the number of arguments is within a range
pub fn require_args_range(
    args: &[RuntimeValue],
    min: usize,
    max: usize,
    method: &str,
    position: Position,
    file: Option<String>,
) -> Result<(), RaccoonError> {
    let count = args.len();
    if count < min || count > max {
        return Err(RaccoonError::new(
            format!(
                "{} requires {}-{} argument(s), got {}",
                method, min, max, count
            ),
            position,
            file,
        ));
    }
    Ok(())
}

/// Validates minimum number of arguments
pub fn require_min_args(
    args: &[RuntimeValue],
    min: usize,
    method: &str,
    position: Position,
    file: Option<String>,
) -> Result<(), RaccoonError> {
    if args.len() < min {
        return Err(RaccoonError::new(
            format!(
                "{} requires at least {} argument(s), got {}",
                method,
                min,
                args.len()
            ),
            position,
            file,
        ));
    }
    Ok(())
}

/// Extracts a string value from a RuntimeValue
pub fn extract_str<'a>(
    value: &'a RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<&'a str, RaccoonError> {
    match value {
        RuntimeValue::Str(s) => Ok(&s.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a string, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Extracts an integer value from a RuntimeValue
pub fn extract_int(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<i64, RaccoonError> {
    match value {
        RuntimeValue::Int(i) => Ok(i.value),
        _ => Err(RaccoonError::new(
            format!("{} must be an integer, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Extracts a float value from a RuntimeValue
pub fn extract_float(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<f64, RaccoonError> {
    match value {
        RuntimeValue::Float(f) => Ok(f.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a float, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Extracts a boolean value from a RuntimeValue
pub fn extract_bool(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<bool, RaccoonError> {
    match value {
        RuntimeValue::Bool(b) => Ok(b.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a boolean, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Extracts a numeric value (int or float) as f64
pub fn extract_numeric(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<f64, RaccoonError> {
    match value {
        RuntimeValue::Int(i) => Ok(i.value as f64),
        RuntimeValue::Float(f) => Ok(f.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a number, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Extracts a list value from a RuntimeValue
pub fn extract_list<'a>(
    value: &'a RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<&'a crate::runtime::ListValue, RaccoonError> {
    match value {
        RuntimeValue::List(l) => Ok(l),
        _ => Err(RaccoonError::new(
            format!("{} must be a list, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

/// Converts a value to boolean (truthy/falsy)
pub fn to_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Bool(b) => b.value,
        RuntimeValue::Null(_) => false,
        RuntimeValue::Int(i) => i.value != 0,
        RuntimeValue::Float(f) => f.value != 0.0,
        RuntimeValue::Str(s) => !s.value.is_empty(),
        RuntimeValue::List(l) => !l.elements.is_empty(),
        _ => true,
    }
}

/// Creates a method not found error
pub fn method_not_found_error(
    type_name: &str,
    method: &str,
    position: Position,
    file: Option<String>,
) -> RaccoonError {
    RaccoonError::new(
        format!("Method '{}' not found on type '{}'", method, type_name),
        position,
        file,
    )
}

/// Creates a static method not found error
pub fn static_method_not_found_error(
    type_name: &str,
    method: &str,
    position: Position,
    file: Option<String>,
) -> RaccoonError {
    RaccoonError::new(
        format!("Static method '{}' not found on type '{}'", method, type_name),
        position,
        file,
    )
}

/// Creates a property not found error
pub fn property_not_found_error(
    type_name: &str,
    property: &str,
    position: Position,
    file: Option<String>,
) -> RaccoonError {
    RaccoonError::new(
        format!("Property '{}' not found on type '{}'", property, type_name),
        position,
        file,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{IntValue, StrValue, BoolValue};

    #[test]
    fn test_require_args() {
        let args = vec![RuntimeValue::Int(IntValue::new(42))];
        assert!(require_args(&args, 1, "test", Position::default(), None).is_ok());
        assert!(require_args(&args, 2, "test", Position::default(), None).is_err());
    }

    #[test]
    fn test_extract_str() {
        let val = RuntimeValue::Str(StrValue::new("hello".to_string()));
        assert_eq!(extract_str(&val, "arg", Position::default(), None).unwrap(), "hello");

        let int_val = RuntimeValue::Int(IntValue::new(42));
        assert!(extract_str(&int_val, "arg", Position::default(), None).is_err());
    }

    #[test]
    fn test_to_truthy() {
        assert!(to_truthy(&RuntimeValue::Bool(BoolValue::new(true))));
        assert!(!to_truthy(&RuntimeValue::Bool(BoolValue::new(false))));
        assert!(to_truthy(&RuntimeValue::Int(IntValue::new(1))));
        assert!(!to_truthy(&RuntimeValue::Int(IntValue::new(0))));
    }
}
