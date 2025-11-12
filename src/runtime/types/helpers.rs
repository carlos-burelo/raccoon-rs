use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;

pub fn require_args(
    args: &[RuntimeValue],
    expected: usize,
    method: &str,
    position: Position,
    file: Option<String>,
) -> Result<(), RaccoonError> {
    if args.len() != expected {
        return Err(RaccoonError::new(
            format!(
                "{} requires {} argument(s), got {}",
                method,
                expected,
                args.len()
            ),
            position,
            file,
        ));
    }
    Ok(())
}

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

pub fn extract_decimal(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<f64, RaccoonError> {
    match value {
        RuntimeValue::Decimal(d) => Ok(d.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a decimal, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

pub fn extract_bigint(
    value: &RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<i128, RaccoonError> {
    match value {
        RuntimeValue::BigInt(b) => Ok(b.value),
        _ => Err(RaccoonError::new(
            format!("{} must be a bigint, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

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

pub fn extract_array<'a>(
    value: &'a RuntimeValue,
    arg_name: &str,
    position: Position,
    file: Option<String>,
) -> Result<&'a crate::runtime::ArrayValue, RaccoonError> {
    match value {
        RuntimeValue::Array(l) => Ok(l),
        _ => Err(RaccoonError::new(
            format!("{} must be a list, got {}", arg_name, value.get_name()),
            position,
            file,
        )),
    }
}

pub fn to_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Bool(b) => b.value,
        RuntimeValue::Null(_) => false,
        RuntimeValue::Int(i) => i.value != 0,
        RuntimeValue::Float(f) => f.value != 0.0,
        RuntimeValue::Str(s) => !s.value.is_empty(),
        RuntimeValue::Array(l) => !l.elements.is_empty(),
        _ => true,
    }
}

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

pub fn static_method_not_found_error(
    type_name: &str,
    method: &str,
    position: Position,
    file: Option<String>,
) -> RaccoonError {
    RaccoonError::new(
        format!(
            "Static method '{}' not found on type '{}'",
            method, type_name
        ),
        position,
        file,
    )
}

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
