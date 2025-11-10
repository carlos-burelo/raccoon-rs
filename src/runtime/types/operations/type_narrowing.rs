use crate::runtime::RuntimeValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NarrowedType {
    Int,
    Float,
    Str,
    Bool,
    Null,
    Array,
    Map,
    Object,
    Function,
    Unknown,
}

pub fn narrow_type(value: &RuntimeValue) -> NarrowedType {
    match value {
        RuntimeValue::Int(_) => NarrowedType::Int,
        RuntimeValue::BigInt(_) => NarrowedType::Int,
        RuntimeValue::Float(_) => NarrowedType::Float,
        RuntimeValue::Decimal(_) => NarrowedType::Float,
        RuntimeValue::Str(_) => NarrowedType::Str,
        RuntimeValue::Bool(_) => NarrowedType::Bool,
        RuntimeValue::Null(_) => NarrowedType::Null,
        RuntimeValue::Array(_) => NarrowedType::Array,
        RuntimeValue::Map(_) => NarrowedType::Map,
        RuntimeValue::Object(_) => NarrowedType::Object,
        RuntimeValue::Class(_) => NarrowedType::Object,
        RuntimeValue::ClassInstance(_) => NarrowedType::Object,
        RuntimeValue::Function(_) => NarrowedType::Function,
        RuntimeValue::NativeFunction(_) => NarrowedType::Function,
        RuntimeValue::NativeAsyncFunction(_) => NarrowedType::Function,
        RuntimeValue::Future(_) => NarrowedType::Unknown,
        RuntimeValue::Enum(_) => NarrowedType::Unknown,
        RuntimeValue::PrimitiveTypeObject(_) => NarrowedType::Unknown,
        RuntimeValue::EnumObject(_) => NarrowedType::Unknown,
        RuntimeValue::Type(_) => NarrowedType::Unknown,
        RuntimeValue::Dynamic(_) => NarrowedType::Unknown,
    }
}

pub fn typeof_check(value: &RuntimeValue, type_name: &str) -> bool {
    match type_name {
        "int" | "number" => matches!(
            value,
            RuntimeValue::Int(_)
                | RuntimeValue::BigInt(_)
                | RuntimeValue::Float(_)
                | RuntimeValue::Decimal(_)
        ),
        "float" | "double" => matches!(value, RuntimeValue::Float(_) | RuntimeValue::Decimal(_)),
        "string" | "str" => matches!(value, RuntimeValue::Str(_)),
        "boolean" | "bool" => matches!(value, RuntimeValue::Bool(_)),
        "null" => matches!(value, RuntimeValue::Null(_)),
        "object" => matches!(
            value,
            RuntimeValue::Object(_)
                | RuntimeValue::Class(_)
                | RuntimeValue::ClassInstance(_)
                | RuntimeValue::Map(_)
                | RuntimeValue::Array(_)
        ),
        "array" => matches!(value, RuntimeValue::Array(_)),
        "map" | "dict" => matches!(value, RuntimeValue::Map(_)),
        "function" => matches!(
            value,
            RuntimeValue::Function(_)
                | RuntimeValue::NativeFunction(_)
                | RuntimeValue::NativeAsyncFunction(_)
        ),
        "class" => matches!(value, RuntimeValue::Class(_)),
        "instance" => matches!(value, RuntimeValue::ClassInstance(_)),
        "future" | "promise" => matches!(value, RuntimeValue::Future(_)),
        "enum" => matches!(value, RuntimeValue::Enum(_) | RuntimeValue::EnumObject(_)),
        _ => false,
    }
}

pub fn instanceof_check(value: &RuntimeValue, class_name: &str) -> bool {
    match value {
        RuntimeValue::ClassInstance(instance) => instance.class_name == class_name,
        RuntimeValue::Class(c) => c.class_name == class_name,
        _ => false,
    }
}

pub fn is_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Bool(b) => b.value,
        RuntimeValue::Null(_) => false,
        RuntimeValue::Int(i) => i.value != 0,
        RuntimeValue::BigInt(i) => i.value != 0,
        RuntimeValue::Float(f) => f.value != 0.0,
        RuntimeValue::Decimal(d) => d.value != 0.0,
        RuntimeValue::Str(s) => !s.value.is_empty(),
        _ => true,
    }
}

pub fn is_null_or_undefined(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Null(_))
}

pub fn is_numeric(value: &RuntimeValue) -> bool {
    matches!(
        value,
        RuntimeValue::Int(_)
            | RuntimeValue::BigInt(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Decimal(_)
    )
}

pub fn is_integer(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Int(_) | RuntimeValue::BigInt(_))
}

pub fn is_float(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Float(_) | RuntimeValue::Decimal(_))
}

pub fn is_string(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Str(_))
}

pub fn is_boolean(value: &RuntimeValue) -> bool {
    matches!(value, RuntimeValue::Bool(_))
}

pub fn is_collection(value: &RuntimeValue) -> bool {
    matches!(
        value,
        RuntimeValue::Array(_) | RuntimeValue::Map(_) | RuntimeValue::Object(_)
    )
}

pub fn is_callable(value: &RuntimeValue) -> bool {
    matches!(
        value,
        RuntimeValue::Function(_)
            | RuntimeValue::NativeFunction(_)
            | RuntimeValue::NativeAsyncFunction(_)
            | RuntimeValue::Class(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{FloatValue, IntValue};

    #[test]
    fn test_narrow_type() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));

        assert_eq!(narrow_type(&int_val), NarrowedType::Int);
        assert_eq!(narrow_type(&float_val), NarrowedType::Float);
    }

    #[test]
    fn test_typeof_check() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));

        assert!(typeof_check(&int_val, "number"));
        assert!(typeof_check(&int_val, "int"));
        assert!(typeof_check(&float_val, "float"));
        assert!(!typeof_check(&int_val, "string"));
    }

    #[test]
    fn test_type_predicates() {
        let int_val = RuntimeValue::Int(IntValue::new(42));
        let float_val = RuntimeValue::Float(FloatValue::new(3.14));

        assert!(is_numeric(&int_val));
        assert!(is_numeric(&float_val));
        assert!(is_integer(&int_val));
        assert!(!is_integer(&float_val));
        assert!(!is_numeric(&RuntimeValue::Str(
            crate::runtime::StrValue::new("test".to_string())
        )));
    }
}
