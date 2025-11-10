/// Type compatibility and validation module
/// Centralizes all type checking and compatibility logic
/// Consolidates logic from src/type_system/checker.rs
use crate::runtime::RuntimeValue;

/// Checks if a type is numeric (Int or Float)
pub fn is_numeric_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Int(_) | RuntimeValue::Float(_))
}

/// Checks if a type is an integer
pub fn is_integer_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Int(_))
}

/// Checks if a type is a float
pub fn is_float_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Float(_))
}

/// Checks if a type is a string
pub fn is_string_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Str(_))
}

/// Checks if a type is a boolean
pub fn is_bool_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Bool(_))
}

/// Checks if a value is null
pub fn is_null_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Null(_))
}

/// Checks if two types are compatible for an operation
/// Two types are compatible if they can be used together in operations
pub fn are_types_compatible(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    // Numeric compatibility
    if is_numeric_type(left) && is_numeric_type(right) {
        return true;
    }

    // String compatibility (for concatenation)
    if is_string_type(left) && is_string_type(right) {
        return true;
    }

    // Same type compatibility
    std::mem::discriminant(left) == std::mem::discriminant(right)
}

/// Gets the type name of a value for error messages
pub fn get_type_name(value: &RuntimeValue) -> &'static str {
    match value {
        RuntimeValue::Int(_) => "int",
        RuntimeValue::BigInt(_) => "bigint",
        RuntimeValue::Float(_) => "float",
        RuntimeValue::Decimal(_) => "decimal",
        RuntimeValue::Str(_) => "str",
        RuntimeValue::Bool(_) => "bool",
        RuntimeValue::Null(_) => "null",
        RuntimeValue::Array(_) => "list",
        RuntimeValue::Map(_) => "map",
        RuntimeValue::Object(_) => "object",
        RuntimeValue::Class(_) => "class",
        RuntimeValue::ClassInstance(_) => "instance",
        RuntimeValue::Function(_) => "function",
        RuntimeValue::NativeFunction(_) => "native_function",
        RuntimeValue::NativeAsyncFunction(_) => "native_async_function",
        RuntimeValue::Future(_) => "future",
        RuntimeValue::Enum(_) => "enum",
        RuntimeValue::PrimitiveTypeObject(_) => "type",
        RuntimeValue::EnumObject(_) => "enum_object",
        RuntimeValue::Type(_) => "type",
        RuntimeValue::Dynamic(_) => "dynamic",
    }
}

/// Checks if an operation is supported for specific types
pub fn supports_operation(op_name: &str, left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match op_name {
        "add" => {
            // Int+Int, Float+Float, Int+Float, Float+Int, Str+Str
            (is_numeric_type(left) && is_numeric_type(right))
                || (is_string_type(left) && is_string_type(right))
        }
        "subtract" | "multiply" | "divide" | "exponent" => {
            // Only numeric types
            is_numeric_type(left) && is_numeric_type(right)
        }
        "modulo" => {
            // Only integers
            is_integer_type(left) && is_integer_type(right)
        }
        "bitwise_and"
        | "bitwise_or"
        | "bitwise_xor"
        | "left_shift"
        | "right_shift"
        | "unsigned_right_shift" => {
            // Only integers
            is_integer_type(left) && is_integer_type(right)
        }
        "less_than" | "less_or_equal" | "greater_than" | "greater_or_equal" => {
            // Numeric or string types
            (is_numeric_type(left) && is_numeric_type(right))
                || (is_string_type(left) && is_string_type(right))
        }
        "equal" | "not_equal" => {
            // All types support equality
            true
        }
        "and" | "or" => {
            // All types support logical operations (through truthiness)
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{FloatValue, IntValue, StrValue};

    #[test]
    fn test_type_checks() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));
        let str_val = RuntimeValue::Str(StrValue::new("hello".to_string()));

        assert!(is_numeric_type(&int_val));
        assert!(is_numeric_type(&float_val));
        assert!(!is_numeric_type(&str_val));

        assert!(is_integer_type(&int_val));
        assert!(!is_integer_type(&float_val));

        assert!(is_string_type(&str_val));
        assert!(!is_string_type(&int_val));
    }

    #[test]
    fn test_compatibility() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));
        let str_val = RuntimeValue::Str(StrValue::new("hello".to_string()));

        assert!(are_types_compatible(&int_val, &float_val));
        assert!(are_types_compatible(&str_val, &str_val));
        assert!(!are_types_compatible(&int_val, &str_val));
    }

    #[test]
    fn test_operation_support() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));
        let str_val = RuntimeValue::Str(StrValue::new("hello".to_string()));

        assert!(supports_operation("add", &int_val, &float_val));
        assert!(supports_operation("add", &str_val, &str_val));
        assert!(!supports_operation("add", &int_val, &str_val));

        assert!(supports_operation("modulo", &int_val, &int_val));
        assert!(!supports_operation("modulo", &float_val, &float_val));
    }
}
