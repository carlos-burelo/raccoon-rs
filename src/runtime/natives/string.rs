/// String functions: length, upper, lower, trim, substring, etc.
///
/// Uses declarative macros to reduce registration boilerplate by ~60%
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::*;
use std::collections::HashMap;

// Macro to eliminate repetitive registration code
macro_rules! register_string_fn {
    (
        $functions:expr,
        $name:expr,
        $invoke:expr,
        $params:expr,
        $return_type:expr
    ) => {
        $functions.insert(
            $name.to_string(),
            NativeFunctionValue::new(
                $invoke,
                Type::Function(Box::new(FunctionType {
                    params: $params,
                    return_type: $return_type,
                    is_variadic: false,
                })),
            ),
        );
    };
}

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // String length
    register_string_fn!(
        functions,
        "native_str_length",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::Str(s)) = args.first() {
                RuntimeValue::Int(IntValue::new(s.value.len() as i64))
            } else {
                RuntimeValue::Int(IntValue::new(0))
            }
        },
        vec![PrimitiveType::str()],
        PrimitiveType::int()
    );

    // String upper
    register_string_fn!(
        functions,
        "native_str_upper",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::Str(s)) = args.first() {
                RuntimeValue::Str(StrValue::new(s.value.to_uppercase()))
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str()],
        PrimitiveType::str()
    );

    // String lower
    register_string_fn!(
        functions,
        "native_str_lower",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::Str(s)) = args.first() {
                RuntimeValue::Str(StrValue::new(s.value.to_lowercase()))
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str()],
        PrimitiveType::str()
    );

    // String trim
    register_string_fn!(
        functions,
        "native_str_trim",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::Str(s)) = args.first() {
                RuntimeValue::Str(StrValue::new(s.value.trim().to_string()))
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str()],
        PrimitiveType::str()
    );

    // String substring
    register_string_fn!(
        functions,
        "native_str_substring",
        |args: Vec<RuntimeValue>| {
            if args.len() < 3 {
                return RuntimeValue::Str(StrValue::new(String::new()));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Int(start), RuntimeValue::Int(end)) =
                (&args[0], &args[1], &args[2])
            {
                let start = (start.value as usize).min(s.value.len());
                let end = (end.value as usize).min(s.value.len());
                if start <= end {
                    let substr = s.value[start..end].to_string();
                    RuntimeValue::Str(StrValue::new(substr))
                } else {
                    RuntimeValue::Str(StrValue::new(String::new()))
                }
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::int(), PrimitiveType::int()],
        PrimitiveType::str()
    );

    // String charAt
    register_string_fn!(
        functions,
        "native_str_char_at",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Str(StrValue::new(String::new()));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Int(index)) = (&args[0], &args[1]) {
                if index.value >= 0 && (index.value as usize) < s.value.len() {
                    let ch = s.value.chars().nth(index.value as usize).unwrap_or(' ');
                    RuntimeValue::Str(StrValue::new(ch.to_string()))
                } else {
                    RuntimeValue::Str(StrValue::new(String::new()))
                }
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::int()],
        PrimitiveType::str()
    );

    // String indexOf
    register_string_fn!(
        functions,
        "native_str_index_of",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Int(IntValue::new(-1));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Str(substr)) = (&args[0], &args[1]) {
                match s.value.find(&substr.value) {
                    Some(index) => RuntimeValue::Int(IntValue::new(index as i64)),
                    None => RuntimeValue::Int(IntValue::new(-1)),
                }
            } else {
                RuntimeValue::Int(IntValue::new(-1))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::str()],
        PrimitiveType::int()
    );

    // String replace
    register_string_fn!(
        functions,
        "native_str_replace",
        |args: Vec<RuntimeValue>| {
            if args.len() < 3 {
                return RuntimeValue::Str(StrValue::new(String::new()));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Str(from), RuntimeValue::Str(to)) =
                (&args[0], &args[1], &args[2])
            {
                let result = s.value.replace(&from.value, &to.value);
                RuntimeValue::Str(StrValue::new(result))
            } else {
                RuntimeValue::Str(StrValue::new(String::new()))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::str(), PrimitiveType::str()],
        PrimitiveType::str()
    );

    // String split
    register_string_fn!(
        functions,
        "native_str_split",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Str(delimiter)) = (&args[0], &args[1]) {
                let parts: Vec<RuntimeValue> = s
                    .value
                    .split(&delimiter.value)
                    .map(|part| RuntimeValue::Str(StrValue::new(part.to_string())))
                    .collect();
                RuntimeValue::List(ListValue::new(parts, PrimitiveType::str()))
            } else {
                RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::str()],
        Type::List(Box::new(crate::ast::types::ListType {
            element_type: PrimitiveType::str(),
        }))
    );

    // String startsWith
    register_string_fn!(
        functions,
        "native_str_starts_with",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Bool(BoolValue::new(false));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Str(prefix)) = (&args[0], &args[1]) {
                RuntimeValue::Bool(BoolValue::new(s.value.starts_with(&prefix.value)))
            } else {
                RuntimeValue::Bool(BoolValue::new(false))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::str()],
        PrimitiveType::bool()
    );

    // String endsWith
    register_string_fn!(
        functions,
        "native_str_ends_with",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Bool(BoolValue::new(false));
            }

            if let (RuntimeValue::Str(s), RuntimeValue::Str(suffix)) = (&args[0], &args[1]) {
                RuntimeValue::Bool(BoolValue::new(s.value.ends_with(&suffix.value)))
            } else {
                RuntimeValue::Bool(BoolValue::new(false))
            }
        },
        vec![PrimitiveType::str(), PrimitiveType::str()],
        PrimitiveType::bool()
    );
}
