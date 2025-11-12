use crate::runtime::RuntimeValue;

pub fn is_numeric_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Int(_) | RuntimeValue::Float(_))
}

pub fn is_integer_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Int(_))
}

pub fn is_float_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Float(_))
}

pub fn is_string_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Str(_))
}

pub fn is_bool_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Bool(_))
}

pub fn is_null_type(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Null(_))
}

pub fn are_types_compatible(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    if is_numeric_type(left) && is_numeric_type(right) {
        return true;
    }

    if is_string_type(left) && is_string_type(right) {
        return true;
    }

    std::mem::discriminant(left) == std::mem::discriminant(right)
}

pub fn get_type_name(value: &RuntimeValue) -> &'static str {
    match value {
        RuntimeValue::Int(_) => "int",
        RuntimeValue::BigInt(_) => "bigint",
        RuntimeValue::Float(_) => "float",
        RuntimeValue::Decimal(_) => "decimal",
        RuntimeValue::Str(_) => "str",
        RuntimeValue::Bool(_) => "bool",
        RuntimeValue::Null(_) => "null",
        RuntimeValue::Array(_) => "array",
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

pub fn supports_operation(op_name: &str, left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match op_name {
        "add" => {
            (is_numeric_type(left) && is_numeric_type(right))
                || (is_string_type(left) && is_string_type(right))
        }
        "subtract" | "multiply" | "divide" | "exponent" => {
            is_numeric_type(left) && is_numeric_type(right)
        }
        "modulo" => is_integer_type(left) && is_integer_type(right),
        "bitwise_and"
        | "bitwise_or"
        | "bitwise_xor"
        | "left_shift"
        | "right_shift"
        | "unsigned_right_shift" => is_integer_type(left) && is_integer_type(right),
        "less_than" | "less_or_equal" | "greater_than" | "greater_or_equal" => {
            (is_numeric_type(left) && is_numeric_type(right))
                || (is_string_type(left) && is_string_type(right))
        }
        "equal" | "not_equal" => true,
        "and" | "or" => true,
        _ => false,
    }
}
