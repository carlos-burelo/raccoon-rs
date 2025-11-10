use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::native::{FromRaccoon, NativeRegistry, ToRaccoon};
use crate::runtime::values::RuntimeValue;

fn native_add(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
        (Ok(a), Ok(b)) => (a + b).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_subtract(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
        (Ok(a), Ok(b)) => (a - b).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_multiply(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
        (Ok(a), Ok(b)) => (a * b).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_divide(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
        (Ok(a), Ok(b)) => {
            if b == 0 {
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            } else {
                (a / b).to_runtime()
            }
        }
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_modulo(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
        (Ok(a), Ok(b)) => {
            if b == 0 {
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            } else {
                (a % b).to_runtime()
            }
        }
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_string_length(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 1 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match String::from_runtime(&args[0]) {
        Ok(s) => (s.len() as i64).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_string_uppercase(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 1 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match String::from_runtime(&args[0]) {
        Ok(s) => s.to_uppercase().to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_string_lowercase(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 1 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match String::from_runtime(&args[0]) {
        Ok(s) => s.to_lowercase().to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_string_concat(args: Vec<RuntimeValue>) -> RuntimeValue {
    let mut result = String::new();

    for arg in args {
        match String::from_runtime(&arg) {
            Ok(s) => result.push_str(&s),
            _ => return RuntimeValue::Null(crate::runtime::values::NullValue::new()),
        }
    }

    result.to_runtime()
}

fn native_string_contains(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match (
        String::from_runtime(&args[0]),
        String::from_runtime(&args[1]),
    ) {
        (Ok(s), Ok(substr)) => s.contains(&substr).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_list_length(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 1 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match &args[0] {
        RuntimeValue::Array(list) => (list.elements.len() as i64).to_runtime(),
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

fn native_list_push(args: Vec<RuntimeValue>) -> RuntimeValue {
    if args.len() != 2 {
        return RuntimeValue::Null(crate::runtime::values::NullValue::new());
    }

    match &args[0] {
        RuntimeValue::Array(list) => {
            let mut elements = list.elements.clone();
            elements.push(args[1].clone());
            RuntimeValue::Array(crate::runtime::values::ArrayValue::new(
                elements,
                list.element_type.clone(),
            ))
        }
        _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
    }
}

pub fn register_all_native_functions(registry: &NativeRegistry) {
    register_native_fn(
        registry,
        "add",
        native_add,
        vec![PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "subtract",
        native_subtract,
        vec![PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "multiply",
        native_multiply,
        vec![PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "divide",
        native_divide,
        vec![PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "modulo",
        native_modulo,
        vec![PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::int(),
    );

    register_native_fn(
        registry,
        "string_length",
        native_string_length,
        vec![PrimitiveType::str()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "string_uppercase",
        native_string_uppercase,
        vec![PrimitiveType::str()],
        PrimitiveType::str(),
    );
    register_native_fn(
        registry,
        "string_lowercase",
        native_string_lowercase,
        vec![PrimitiveType::str()],
        PrimitiveType::str(),
    );
    register_native_fn(
        registry,
        "string_concat",
        native_string_concat,
        vec![],
        PrimitiveType::str(),
    );
    register_native_fn(
        registry,
        "string_contains",
        native_string_contains,
        vec![PrimitiveType::str(), PrimitiveType::str()],
        PrimitiveType::bool(),
    );

    register_native_fn(
        registry,
        "list_length",
        native_list_length,
        vec![PrimitiveType::any()],
        PrimitiveType::int(),
    );
    register_native_fn(
        registry,
        "list_push",
        native_list_push,
        vec![PrimitiveType::any(), PrimitiveType::any()],
        PrimitiveType::any(),
    );
}

fn register_native_fn(
    registry: &NativeRegistry,
    name: &str,
    function: crate::runtime::values::NativeFn,
    param_types: Vec<Type>,
    return_type: Type,
) {
    registry.register(name, function, param_types, return_type);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_add() {
        let args = vec![
            RuntimeValue::Int(crate::runtime::values::IntValue::new(5)),
            RuntimeValue::Int(crate::runtime::values::IntValue::new(3)),
        ];
        let result = native_add(args);
        assert_eq!(result.to_string(), "8");
    }

    #[test]
    fn test_native_string_length() {
        let args = vec![RuntimeValue::Str(crate::runtime::values::StrValue::new(
            "hello".to_string(),
        ))];
        let result = native_string_length(args);
        assert_eq!(result.to_string(), "5");
    }
}
