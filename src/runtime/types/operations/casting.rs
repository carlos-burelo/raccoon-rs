/// Casting and type conversion module
/// Centralizes all type coercion rules and conversion logic

use crate::error::RaccoonError;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue, BoolValue};
use crate::tokens::Position;

/// Type compatibility and widening rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidenResult {
    Same,
    WidenLeft,
    WidenRight,
    WidenBoth(&'static str),
    Incompatible,
}

/// Determines how two types can be widened for binary operations
pub fn determine_widening(left_type: &RuntimeValue, right_type: &RuntimeValue) -> WidenResult {
    use RuntimeValue::*;

    match (left_type, right_type) {
        (Int(_), Int(_)) => WidenResult::Same,
        (Float(_), Float(_)) => WidenResult::Same,
        (Str(_), Str(_)) => WidenResult::Same,
        (Bool(_), Bool(_)) => WidenResult::Same,
        (Int(_), Float(_)) => WidenResult::WidenLeft,
        (Float(_), Int(_)) => WidenResult::WidenRight,
        _ => WidenResult::Incompatible,
    }
}

/// Attempts to cast a value to a target type
pub fn try_cast(
    value: RuntimeValue,
    target_type_name: &str,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match value {
        RuntimeValue::Int(i) => match target_type_name {
            "int" => Ok(RuntimeValue::Int(i)),
            "float" => Ok(RuntimeValue::Float(FloatValue::new(i.value as f64))),
            "str" => Ok(RuntimeValue::Str(StrValue::new(i.value.to_string()))),
            "bool" => Ok(RuntimeValue::Bool(BoolValue::new(i.value != 0))),
            _ => Err(RaccoonError::new(
                format!("Cannot cast int to {}", target_type_name),
                position,
                file.clone(),
            )),
        },
        RuntimeValue::Float(f) => match target_type_name {
            "float" => Ok(RuntimeValue::Float(f)),
            "int" => Ok(RuntimeValue::Int(IntValue::new(f.value as i64))),
            "str" => Ok(RuntimeValue::Str(StrValue::new(f.value.to_string()))),
            "bool" => Ok(RuntimeValue::Bool(BoolValue::new(f.value != 0.0))),
            _ => Err(RaccoonError::new(
                format!("Cannot cast float to {}", target_type_name),
                position,
                file.clone(),
            )),
        },
        RuntimeValue::Str(s) => match target_type_name {
            "str" => Ok(RuntimeValue::Str(s)),
            "int" => {
                s.value.parse::<i64>()
                    .map(|v| RuntimeValue::Int(IntValue::new(v)))
                    .map_err(|_| RaccoonError::new(
                        format!("Cannot cast '{}' to int", s.value),
                        position,
                        file.clone(),
                    ))
            }
            "float" => {
                s.value.parse::<f64>()
                    .map(|v| RuntimeValue::Float(FloatValue::new(v)))
                    .map_err(|_| RaccoonError::new(
                        format!("Cannot cast '{}' to float", s.value),
                        position,
                        file.clone(),
                    ))
            }
            "bool" => Ok(RuntimeValue::Bool(BoolValue::new(!s.value.is_empty()))),
            _ => Err(RaccoonError::new(
                format!("Cannot cast str to {}", target_type_name),
                position,
                file.clone(),
            )),
        },
        RuntimeValue::Bool(b) => match target_type_name {
            "bool" => Ok(RuntimeValue::Bool(b)),
            "int" => Ok(RuntimeValue::Int(IntValue::new(if b.value { 1 } else { 0 }))),
            "float" => Ok(RuntimeValue::Float(FloatValue::new(if b.value { 1.0 } else { 0.0 }))),
            "str" => Ok(RuntimeValue::Str(StrValue::new(b.value.to_string()))),
            _ => Err(RaccoonError::new(
                format!("Cannot cast bool to {}", target_type_name),
                position,
                file.clone(),
            )),
        },
        RuntimeValue::Null(_) => match target_type_name {
            "str" => Ok(RuntimeValue::Str(StrValue::new("null".to_string()))),
            _ => Err(RaccoonError::new(
                format!("Cannot cast null to {}", target_type_name),
                position,
                file.clone(),
            )),
        },
        v => Err(RaccoonError::new(
            format!("Cannot cast {} to {}", v.get_name(), target_type_name),
            position,
            file.clone(),
        )),
    }
}

/// Widens a value to float
pub fn widen_to_float(value: RuntimeValue) -> Result<RuntimeValue, RaccoonError> {
    match value {
        RuntimeValue::Int(i) => Ok(RuntimeValue::Float(FloatValue::new(i.value as f64))),
        RuntimeValue::Float(f) => Ok(RuntimeValue::Float(f)),
        _ => Err(RaccoonError::new(
            "Cannot widen non-numeric type to float".to_string(),
            crate::tokens::Position::default(),
            None::<String>,
        )),
    }
}

/// Determines the common type for a binary operation
pub fn get_common_type(left: &RuntimeValue, right: &RuntimeValue) -> Option<&'static str> {
    use RuntimeValue::*;

    match (left, right) {
        (Int(_), Int(_)) => Some("int"),
        (Float(_), Float(_)) => Some("float"),
        (Int(_), Float(_)) | (Float(_), Int(_)) => Some("float"),
        (Str(_), Str(_)) => Some("str"),
        (Bool(_), Bool(_)) => Some("bool"),
        (Null(_), Null(_)) => Some("null"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widening_rules() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));

        assert_eq!(determine_widening(&int_val, &int_val), WidenResult::Same);
        assert_eq!(determine_widening(&int_val, &float_val), WidenResult::WidenLeft);
        assert_eq!(determine_widening(&float_val, &int_val), WidenResult::WidenRight);
    }

    #[test]
    fn test_get_common_type() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));
        let str_val = RuntimeValue::Str(StrValue::new("hello".to_string()));

        assert_eq!(get_common_type(&int_val, &int_val), Some("int"));
        assert_eq!(get_common_type(&float_val, &float_val), Some("float"));
        assert_eq!(get_common_type(&int_val, &float_val), Some("float"));
        assert_eq!(get_common_type(&str_val, &str_val), Some("str"));
        assert_eq!(get_common_type(&int_val, &str_val), None);
    }
}
