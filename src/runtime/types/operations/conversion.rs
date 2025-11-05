/// Type conversion module
/// Provides comprehensive type conversion utilities with proper error handling
/// Extends casting.rs with more conversion helpers

use crate::error::RaccoonError;
use crate::runtime::{FloatValue, IntValue, RuntimeValue, StrValue, BoolValue};
use crate::tokens::Position;

/// Result of a type conversion attempt
pub type ConversionResult = Result<RuntimeValue, ConversionError>;

/// Detailed error information for type conversions
#[derive(Debug, Clone)]
pub enum ConversionError {
    InvalidType { from: String, to: String },
    ParseError { value: String, target_type: String },
    RangeError { value: String, reason: String },
    UnsupportedConversion { from: String, to: String },
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::InvalidType { from, to } => {
                write!(f, "Cannot convert {} to {}", from, to)
            }
            ConversionError::ParseError { value, target_type } => {
                write!(f, "Failed to parse '{}' as {}", value, target_type)
            }
            ConversionError::RangeError { value, reason } => {
                write!(f, "Value {} is out of range: {}", value, reason)
            }
            ConversionError::UnsupportedConversion { from, to } => {
                write!(f, "Conversion from {} to {} is not supported", from, to)
            }
        }
    }
}

/// Converts a RuntimeValue to a string representation
/// Similar to JavaScript's String() constructor
pub fn to_string(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Str(s) => s.value.clone(),
        RuntimeValue::Int(i) => i.value.to_string(),
        RuntimeValue::BigInt(i) => i.value.to_string(),
        RuntimeValue::Float(f) => {
            if f.value.is_nan() {
                "NaN".to_string()
            } else if f.value.is_infinite() {
                if f.value.is_sign_positive() {
                    "Infinity".to_string()
                } else {
                    "-Infinity".to_string()
                }
            } else {
                f.value.to_string()
            }
        }
        RuntimeValue::Decimal(d) => d.value.to_string(),
        RuntimeValue::Bool(b) => b.value.to_string(),
        RuntimeValue::Null(_) => "null".to_string(),
        RuntimeValue::List(l) => {
            let elements: Vec<String> = l
                .elements
                .iter()
                .map(to_string)
                .collect();
            format!("[{}]", elements.join(", "))
        }
        RuntimeValue::Map(m) => {
            let entries: Vec<String> = m
                .entries
                .iter()
                .map(|(k, v)| format!("{}: {}", k, to_string(v)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
        RuntimeValue::Object(o) => {
            o.to_string()
        }
        RuntimeValue::Class(c) => format!("class {}", c.class_name),
        RuntimeValue::ClassInstance(i) => format!("{}instance", i.class_name),
        RuntimeValue::Function(_) => "[function]".to_string(),
        RuntimeValue::NativeFunction(_) => "[native function]".to_string(),
        RuntimeValue::NativeAsyncFunction(_) => "[async native function]".to_string(),
        RuntimeValue::Future(_) => "[Future]".to_string(),
        RuntimeValue::Enum(e) => format!("enum {}", e.enum_name),
        RuntimeValue::PrimitiveTypeObject(p) => format!("type {}", p.type_name),
        RuntimeValue::EnumObject(e) => format!("enum {}", e.enum_name),
    }
}

/// Converts a RuntimeValue to a number (int or float)
/// Similar to JavaScript's Number() constructor
pub fn to_number(value: &RuntimeValue) -> ConversionResult {
    match value {
        RuntimeValue::Int(i) => Ok(RuntimeValue::Int(IntValue::new(i.value))),
        RuntimeValue::BigInt(i) => Ok(RuntimeValue::Int(IntValue::new(i.value as i64))),
        RuntimeValue::Float(f) => Ok(RuntimeValue::Float(FloatValue::new(f.value))),
        RuntimeValue::Decimal(d) => Ok(RuntimeValue::Float(FloatValue::new(d.value))),
        RuntimeValue::Bool(b) => Ok(RuntimeValue::Int(IntValue::new(if b.value { 1 } else { 0 }))),
        RuntimeValue::Str(s) => {
            let trimmed = s.value.trim();
            if trimmed.is_empty() {
                Ok(RuntimeValue::Int(IntValue::new(0)))
            } else if let Ok(i) = trimmed.parse::<i64>() {
                Ok(RuntimeValue::Int(IntValue::new(i)))
            } else if let Ok(f) = trimmed.parse::<f64>() {
                Ok(RuntimeValue::Float(FloatValue::new(f)))
            } else {
                Err(ConversionError::ParseError {
                    value: s.value.clone(),
                    target_type: "number".to_string(),
                })
            }
        }
        RuntimeValue::Null(_) => Ok(RuntimeValue::Int(IntValue::new(0))),
        _ => Err(ConversionError::UnsupportedConversion {
            from: value.get_name().to_string(),
            to: "number".to_string(),
        }),
    }
}

/// Converts a RuntimeValue to a boolean
/// Similar to JavaScript's Boolean() constructor
pub fn to_boolean(value: &RuntimeValue) -> RuntimeValue {
    use super::type_narrowing::is_truthy;
    RuntimeValue::Bool(BoolValue::new(is_truthy(value)))
}

/// Converts a RuntimeValue to an integer, with optional range checking
pub fn to_integer(value: &RuntimeValue) -> ConversionResult {
    match to_number(value)? {
        RuntimeValue::Int(i) => Ok(RuntimeValue::Int(i)),
        RuntimeValue::Float(f) => {
            if f.value.is_nan() || f.value.is_infinite() {
                Err(ConversionError::RangeError {
                    value: f.value.to_string(),
                    reason: "Cannot convert NaN or Infinity to integer".to_string(),
                })
            } else {
                Ok(RuntimeValue::Int(IntValue::new(f.value as i64)))
            }
        }
        _ => Err(ConversionError::InvalidType {
            from: value.get_name().to_string(),
            to: "int".to_string(),
        }),
    }
}

/// Converts a RuntimeValue to a float
pub fn to_float(value: &RuntimeValue) -> ConversionResult {
    match to_number(value)? {
        RuntimeValue::Float(f) => Ok(RuntimeValue::Float(f)),
        RuntimeValue::Int(i) => Ok(RuntimeValue::Float(FloatValue::new(i.value as f64))),
        _ => Err(ConversionError::InvalidType {
            from: value.get_name().to_string(),
            to: "float".to_string(),
        }),
    }
}

/// Converts a Raccoon error to a standard error message
pub fn error_to_raccoon_error(
    err: ConversionError,
    position: Position,
    file: Option<String>,
) -> RaccoonError {
    RaccoonError::new(err.to_string(), position, file)
}

/// Safely attempts a conversion and returns a RaccoonError on failure
pub fn safe_convert(
    value: RuntimeValue,
    target_type: &str,
    position: Position,
    file: Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    let result = match target_type {
        "string" | "str" => Ok(RuntimeValue::Str(StrValue::new(to_string(&value)))),
        "number" | "int" => to_integer(&value),
        "float" | "f64" => to_float(&value),
        "boolean" | "bool" => Ok(to_boolean(&value)),
        _ => Err(ConversionError::UnsupportedConversion {
            from: value.get_name().to_string(),
            to: target_type.to_string(),
        }),
    };

    result.map_err(|e| error_to_raccoon_error(e, position, file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        assert_eq!(to_string(&int_val), "42");

        let bool_val = RuntimeValue::Bool(BoolValue::new(true));
        assert_eq!(to_string(&bool_val), "true");
    }

    #[test]
    fn test_to_number() {
        let str_val = RuntimeValue::Str(StrValue::new("42".to_string()));
        let result = to_number(&str_val);
        assert!(result.is_ok());

        let invalid_str = RuntimeValue::Str(StrValue::new("not a number".to_string()));
        let result = to_number(&invalid_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_boolean() {
        let zero = RuntimeValue::Int(IntValue::new(0));
        match to_boolean(&zero) {
            RuntimeValue::Bool(b) => assert!(!b.value),
            _ => panic!("Expected bool"),
        }

        let one = RuntimeValue::Int(IntValue::new(1));
        match to_boolean(&one) {
            RuntimeValue::Bool(b) => assert!(b.value),
            _ => panic!("Expected bool"),
        }
    }
}
