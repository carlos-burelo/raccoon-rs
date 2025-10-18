use crate::ast::types::{ListType, PrimitiveType, Type};
use crate::runtime::values::{IntValue, ListValue, NullValue, RuntimeValue, StrValue};
use std::collections::HashMap;

pub struct StringModule;

impl StringModule {
    pub fn name() -> &'static str {
        "std:string"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("toUpperCase".to_string(), Self::create_to_upper_case_fn());
        exports.insert("toLowerCase".to_string(), Self::create_to_lower_case_fn());
        exports.insert("trim".to_string(), Self::create_trim_fn());
        exports.insert("split".to_string(), Self::create_split_fn());
        exports.insert("join".to_string(), Self::create_join_fn());
        exports.insert("concat".to_string(), Self::create_concat_fn());
        exports.insert("replace".to_string(), Self::create_replace_fn());
        exports.insert("substring".to_string(), Self::create_substring_fn());
        exports.insert("charAt".to_string(), Self::create_char_at_fn());
        exports.insert("indexOf".to_string(), Self::create_index_of_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "toUpperCase" => Some(Self::create_to_upper_case_fn()),
            "toLowerCase" => Some(Self::create_to_lower_case_fn()),
            "trim" => Some(Self::create_trim_fn()),
            "split" => Some(Self::create_split_fn()),
            "join" => Some(Self::create_join_fn()),
            "concat" => Some(Self::create_concat_fn()),
            "replace" => Some(Self::create_replace_fn()),
            "substring" => Some(Self::create_substring_fn()),
            "charAt" => Some(Self::create_char_at_fn()),
            "indexOf" => Some(Self::create_index_of_fn()),
            _ => None,
        }
    }

    fn create_to_upper_case_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        RuntimeValue::Str(StrValue::new(s.value.to_uppercase()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_to_lower_case_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        RuntimeValue::Str(StrValue::new(s.value.to_lowercase()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_trim_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(s) => {
                        RuntimeValue::Str(StrValue::new(s.value.trim().to_string()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_split_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let string = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let delimiter = match &args[1] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let parts: Vec<RuntimeValue> = string
                    .split(delimiter)
                    .map(|s| RuntimeValue::Str(StrValue::new(s.to_string())))
                    .collect();
                RuntimeValue::List(ListValue::new(parts, PrimitiveType::str()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::str(),
                })),
                is_variadic: false,
            })),
        ))
    }

    fn create_join_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let list = match &args[0] {
                    RuntimeValue::List(l) => &l.elements,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let delimiter = match &args[1] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let strings: Vec<String> = list.iter().map(|v| v.to_string()).collect();
                RuntimeValue::Str(StrValue::new(strings.join(delimiter)))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::str(),
                ],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_concat_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let str1 = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let str2 = match &args[1] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Str(StrValue::new(format!("{}{}", str1, str2)))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_replace_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 3 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let string = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let from = match &args[1] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let to = match &args[2] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let replaced = string.replace(from, to);
                RuntimeValue::Str(StrValue::new(replaced))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    PrimitiveType::str(),
                    PrimitiveType::str(),
                    PrimitiveType::str(),
                ],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_substring_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 3 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let string = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let start = match &args[1] {
                    RuntimeValue::Int(i) => i.value as usize,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let end = match &args[2] {
                    RuntimeValue::Int(i) => i.value as usize,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let substring = string
                    .chars()
                    .skip(start)
                    .take(end - start)
                    .collect::<String>();
                RuntimeValue::Str(StrValue::new(substring))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    PrimitiveType::str(),
                    PrimitiveType::int(),
                    PrimitiveType::int(),
                ],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_char_at_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let string = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let index = match &args[1] {
                    RuntimeValue::Int(i) => i.value as usize,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let ch = string.chars().nth(index);
                match ch {
                    Some(c) => RuntimeValue::Str(StrValue::new(c.to_string())),
                    None => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::int()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_index_of_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let string = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let search = match &args[1] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                match string.find(search) {
                    Some(index) => RuntimeValue::Int(IntValue::new(index as i64)),
                    None => RuntimeValue::Int(IntValue::new(-1)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }
}
