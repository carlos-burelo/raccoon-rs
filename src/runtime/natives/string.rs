/// String functions: length, upper, lower, trim, substring, etc.
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::*;
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // String length
    functions.insert(
        "native_str_length".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(s)) = args.first() {
                    RuntimeValue::Int(IntValue::new(s.value.len() as i64))
                } else {
                    RuntimeValue::Int(IntValue::new(0))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ),
    );

    // String upper
    functions.insert(
        "native_str_upper".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(s)) = args.first() {
                    RuntimeValue::Str(StrValue::new(s.value.to_uppercase()))
                } else {
                    RuntimeValue::Str(StrValue::new(String::new()))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String lower
    functions.insert(
        "native_str_lower".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(s)) = args.first() {
                    RuntimeValue::Str(StrValue::new(s.value.to_lowercase()))
                } else {
                    RuntimeValue::Str(StrValue::new(String::new()))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String trim
    functions.insert(
        "native_str_trim".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(s)) = args.first() {
                    RuntimeValue::Str(StrValue::new(s.value.trim().to_string()))
                } else {
                    RuntimeValue::Str(StrValue::new(String::new()))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String substring
    functions.insert(
        "native_str_substring".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::int(), PrimitiveType::int()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String charAt
    functions.insert(
        "native_str_char_at".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::int()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String indexOf
    functions.insert(
        "native_str_index_of".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ),
    );

    // String replace
    functions.insert(
        "native_str_replace".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // String split
    functions.insert(
        "native_str_split".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: Type::List(Box::new(crate::ast::types::ListType {
                    element_type: PrimitiveType::str(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // String startsWith
    functions.insert(
        "native_str_starts_with".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ),
    );

    // String endsWith
    functions.insert(
        "native_str_ends_with".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ),
    );
}
