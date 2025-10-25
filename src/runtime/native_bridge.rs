use crate::ast::types::{FunctionType, FutureType, ListType, PrimitiveType, Type};
use crate::runtime::values::{
    BoolValue, FloatValue, IntValue, ListValue, MapValue, NativeAsyncFunctionValue,
    NativeFunctionValue, NullValue, ObjectValue, RuntimeValue, StrValue,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NativeBridge {
    functions: HashMap<String, NativeFunctionValue>,
    async_functions: HashMap<String, NativeAsyncFunctionValue>,
}

impl NativeBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            functions: HashMap::new(),
            async_functions: HashMap::new(),
        };
        bridge.register_all_natives();
        bridge
    }

    fn register_all_natives(&mut self) {
        self.register_io_primitives();
        self.register_time_primitives();
        self.register_random_primitives();
        self.register_json_primitives();
        self.register_http_primitives();
        self.register_string_primitives();
        self.register_array_primitives();
        self.register_math_primitives();
        self.register_ffi_primitives();
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        self.functions
            .get(name)
            .map(|f| RuntimeValue::NativeFunction(f.clone()))
    }

    pub fn get_async(&self, name: &str) -> Option<RuntimeValue> {
        self.async_functions
            .get(name)
            .map(|f| RuntimeValue::NativeAsyncFunction(f.clone()))
    }

    fn register_io_primitives(&mut self) {
        let print_fn = NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        let eprint_fn = NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                eprintln!("{}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        self.functions
            .insert("native_print".to_string(), print_fn.clone());
        self.functions.insert("_print_native".to_string(), print_fn);
        self.functions
            .insert("native_eprint".to_string(), eprint_fn.clone());
        self.functions
            .insert("_eprint_native".to_string(), eprint_fn);
    }

    fn register_time_primitives(&mut self) {
        self.functions.insert(
            "native_time_now".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();
                    RuntimeValue::Int(IntValue::new(now.as_millis() as i64))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::int(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_time_now_secs".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();
                    RuntimeValue::Int(IntValue::new(now.as_secs() as i64))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::int(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_random_primitives(&mut self) {
        self.functions.insert(
            "native_random".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();
                    let nanos = now.as_nanos() as u128;
                    let val = (nanos as f64) / 1_000_000_000.0;
                    RuntimeValue::Float(FloatValue::new(val))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_json_primitives(&mut self) {
        self.functions.insert(
            "native_json_parse".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.is_empty() {
                        return RuntimeValue::Null(NullValue::new());
                    }

                    let json_str = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Null(NullValue::new()),
                    };

                    match serde_json::from_str::<JsonValue>(&json_str) {
                        Ok(json) => convert_serde_to_runtime(&json),
                        Err(_) => RuntimeValue::Null(NullValue::new()),
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str()],
                    return_type: PrimitiveType::any(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_json_stringify".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.is_empty() {
                        return RuntimeValue::Str(StrValue::new("null".to_string()));
                    }

                    let json = convert_runtime_to_serde(&args[0]);
                    match serde_json::to_string(&json) {
                        Ok(s) => RuntimeValue::Str(StrValue::new(s)),
                        Err(_) => RuntimeValue::Str(StrValue::new("null".to_string())),
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::any()],
                    return_type: PrimitiveType::str(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_json_stringify_pretty".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.is_empty() {
                        return RuntimeValue::Str(StrValue::new("null".to_string()));
                    }

                    let json = convert_runtime_to_serde(&args[0]);
                    match serde_json::to_string_pretty(&json) {
                        Ok(s) => RuntimeValue::Str(StrValue::new(s)),
                        Err(_) => RuntimeValue::Str(StrValue::new("null".to_string())),
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::any(), PrimitiveType::int()],
                    return_type: PrimitiveType::str(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_string_primitives(&mut self) {
        self.functions.insert(
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

        self.functions.insert(
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

        self.functions.insert(
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

        self.functions.insert(
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

        self.functions.insert(
            "native_str_substring".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 3 {
                        return RuntimeValue::Str(StrValue::new(String::new()));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Str(StrValue::new(String::new())),
                    };

                    let start = match &args[1] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => return RuntimeValue::Str(StrValue::new(String::new())),
                    };

                    let end = match &args[2] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => s.len(),
                    };

                    let chars: Vec<char> = s.chars().collect();
                    let end = end.min(chars.len());
                    let start = start.min(end);

                    let result: String = chars[start..end].iter().collect();
                    RuntimeValue::Str(StrValue::new(result))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        PrimitiveType::str(),
                        PrimitiveType::int(),
                        PrimitiveType::int(),
                    ],
                    return_type: PrimitiveType::str(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_str_char_at".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::Str(StrValue::new(String::new()));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Str(StrValue::new(String::new())),
                    };

                    let index = match &args[1] {
                        RuntimeValue::Int(i) => i.value as usize,
                        _ => return RuntimeValue::Str(StrValue::new(String::new())),
                    };

                    let chars: Vec<char> = s.chars().collect();
                    if index < chars.len() {
                        RuntimeValue::Str(StrValue::new(chars[index].to_string()))
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

        self.functions.insert(
            "native_str_index_of".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::Int(IntValue::new(-1));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Int(IntValue::new(-1)),
                    };

                    let substr = match &args[1] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Int(IntValue::new(-1)),
                    };

                    match s.find(&substr) {
                        Some(pos) => RuntimeValue::Int(IntValue::new(pos as i64)),
                        None => RuntimeValue::Int(IntValue::new(-1)),
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str(), PrimitiveType::str()],
                    return_type: PrimitiveType::int(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_str_replace".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 3 {
                        return RuntimeValue::Str(StrValue::new(String::new()));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Str(StrValue::new(String::new())),
                    };

                    let from = match &args[1] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Str(StrValue::new(s)),
                    };

                    let to = match &args[2] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Str(StrValue::new(s)),
                    };

                    RuntimeValue::Str(StrValue::new(s.replace(&from, &to)))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        PrimitiveType::str(),
                        PrimitiveType::str(),
                        PrimitiveType::str(),
                    ],
                    return_type: PrimitiveType::str(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_str_split".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => {
                            return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()));
                        }
                    };

                    let delimiter = match &args[1] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => {
                            return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()));
                        }
                    };

                    let parts: Vec<RuntimeValue> = s
                        .split(&delimiter)
                        .map(|part| RuntimeValue::Str(StrValue::new(part.to_string())))
                        .collect();

                    RuntimeValue::List(ListValue::new(parts, PrimitiveType::str()))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str(), PrimitiveType::str()],
                    return_type: Type::List(Box::new(ListType {
                        element_type: PrimitiveType::str(),
                    })),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_str_starts_with".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::Bool(BoolValue::new(false));
                    }

                    let s = match &args[0] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Bool(BoolValue::new(false)),
                    };

                    let prefix = match &args[1] {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => return RuntimeValue::Bool(BoolValue::new(false)),
                    };

                    RuntimeValue::Bool(BoolValue::new(s.starts_with(&prefix)))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str(), PrimitiveType::str()],
                    return_type: PrimitiveType::bool(),
                    is_variadic: false,
                })),
            ),
        );

        let ends_with_fn = NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }

                let s = match &args[0] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Bool(BoolValue::new(false)),
                };

                let suffix = match &args[1] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Bool(BoolValue::new(false)),
                };

                RuntimeValue::Bool(BoolValue::new(s.ends_with(&suffix)))
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        );

        self.functions
            .insert("native_str_ends_with".to_string(), ends_with_fn.clone());
        self.functions
            .insert("_ends_with_native".to_string(), ends_with_fn);

        self.add_string_aliases();
    }

    fn add_string_aliases(&mut self) {
        let aliases = vec![
            ("native_str_length", "_length_native"),
            ("native_str_upper", "_upper_native"),
            ("native_str_lower", "_lower_native"),
            ("native_str_trim", "_trim_native"),
            ("native_str_substring", "_substring_native"),
            ("native_str_char_at", "_char_at_native"),
            ("native_str_index_of", "_index_of_native"),
            ("native_str_replace", "_replace_native"),
            ("native_str_split", "_split_native"),
            ("native_str_starts_with", "_starts_with_native"),
            ("native_json_parse", "_parse_native"),
            ("native_json_stringify", "_stringify_native"),
            ("native_json_stringify_pretty", "_stringify_pretty_native"),
            ("native_array_length", "_length_native"),
            ("native_array_push", "_push_native"),
            ("native_array_pop", "_pop_native"),
            ("native_array_shift", "_shift_native"),
            ("native_array_unshift", "_unshift_native"),
            ("native_array_slice", "_slice_native"),
            ("native_array_reverse", "_reverse_native"),
            ("native_array_sort", "_sort_native"),
        ];

        for (original, alias) in aliases {
            if let Some(func) = self.functions.get(original) {
                self.functions.insert(alias.to_string(), func.clone());
            }
        }
    }

    fn register_math_primitives(&mut self) {
        // sqrt
        self.functions.insert(
            "_sqrt_native".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::Float(f)) = args.first() {
                        RuntimeValue::Float(FloatValue::new(f.value.sqrt()))
                    } else if let Some(RuntimeValue::Int(i)) = args.first() {
                        RuntimeValue::Float(FloatValue::new((i.value as f64).sqrt()))
                    } else {
                        RuntimeValue::Float(FloatValue::new(0.0))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::float()],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );

        // pow
        self.functions.insert(
            "_pow_native".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::Float(FloatValue::new(0.0));
                    }
                    let base = match &args[0] {
                        RuntimeValue::Float(f) => f.value,
                        RuntimeValue::Int(i) => i.value as f64,
                        _ => return RuntimeValue::Float(FloatValue::new(0.0)),
                    };
                    let exp = match &args[1] {
                        RuntimeValue::Float(f) => f.value,
                        RuntimeValue::Int(i) => i.value as f64,
                        _ => return RuntimeValue::Float(FloatValue::new(0.0)),
                    };
                    RuntimeValue::Float(FloatValue::new(base.powf(exp)))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::float(), PrimitiveType::float()],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );

        // sin
        self.functions.insert(
            "_sin_native".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::Float(f)) = args.first() {
                        RuntimeValue::Float(FloatValue::new(f.value.sin()))
                    } else if let Some(RuntimeValue::Int(i)) = args.first() {
                        RuntimeValue::Float(FloatValue::new((i.value as f64).sin()))
                    } else {
                        RuntimeValue::Float(FloatValue::new(0.0))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::float()],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );

        // cos
        self.functions.insert(
            "_cos_native".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::Float(f)) = args.first() {
                        RuntimeValue::Float(FloatValue::new(f.value.cos()))
                    } else if let Some(RuntimeValue::Int(i)) = args.first() {
                        RuntimeValue::Float(FloatValue::new((i.value as f64).cos()))
                    } else {
                        RuntimeValue::Float(FloatValue::new(0.0))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::float()],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );

        // tan
        self.functions.insert(
            "_tan_native".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::Float(f)) = args.first() {
                        RuntimeValue::Float(FloatValue::new(f.value.tan()))
                    } else if let Some(RuntimeValue::Int(i)) = args.first() {
                        RuntimeValue::Float(FloatValue::new((i.value as f64).tan()))
                    } else {
                        RuntimeValue::Float(FloatValue::new(0.0))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::float()],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );

        // random (better implementation)
        self.functions.insert(
            "_random_native".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    use std::collections::hash_map::RandomState;
                    use std::hash::{BuildHasher, Hasher};
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default();
                    let mut hasher = RandomState::new().build_hasher();
                    hasher.write_u128(now.as_nanos());
                    let hash = hasher.finish();
                    let val = (hash as f64) / (u64::MAX as f64);
                    RuntimeValue::Float(FloatValue::new(val))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::float(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_array_primitives(&mut self) {
        self.functions.insert(
            "native_array_length".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::List(list)) = args.first() {
                        RuntimeValue::Int(IntValue::new(list.elements.len() as i64))
                    } else {
                        RuntimeValue::Int(IntValue::new(0))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    }))],
                    return_type: PrimitiveType::int(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_array_push".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 2 {
                        return RuntimeValue::Null(NullValue::new());
                    }

                    RuntimeValue::Null(NullValue::new())
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        Type::List(Box::new(ListType {
                            element_type: PrimitiveType::any(),
                        })),
                        PrimitiveType::any(),
                    ],
                    return_type: PrimitiveType::void(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_array_slice".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if args.len() < 3 {
                        return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
                    }

                    let list = match &args[0] {
                        RuntimeValue::List(l) => l.clone(),
                        _ => {
                            return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
                        }
                    };

                    let start = match &args[1] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => 0,
                    };

                    let end = match &args[2] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => list.elements.len(),
                    };

                    let end = end.min(list.elements.len());
                    let start = start.min(end);

                    let sliced = list.elements[start..end].to_vec();
                    RuntimeValue::List(ListValue::new(sliced, list.element_type))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        Type::List(Box::new(ListType {
                            element_type: PrimitiveType::any(),
                        })),
                        PrimitiveType::int(),
                        PrimitiveType::int(),
                    ],
                    return_type: Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_array_reverse".to_string(),
            NativeFunctionValue::new(
                |args: Vec<RuntimeValue>| {
                    if let Some(RuntimeValue::List(list)) = args.first() {
                        let mut reversed = list.elements.clone();
                        reversed.reverse();
                        RuntimeValue::List(ListValue::new(reversed, list.element_type.clone()))
                    } else {
                        RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()))
                    }
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    }))],
                    return_type: Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_ffi_primitives(&mut self) {
        self.functions.insert(
            "native_ffi_load_library".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str(), PrimitiveType::str()],
                    return_type: PrimitiveType::void(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_ffi_register_function".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
                Type::Function(Box::new(FunctionType {
                    params: vec![PrimitiveType::str(), PrimitiveType::any()],
                    return_type: PrimitiveType::void(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "native_ffi_call".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        PrimitiveType::str(),
                        PrimitiveType::str(),
                        Type::List(Box::new(ListType {
                            element_type: PrimitiveType::any(),
                        })),
                    ],
                    return_type: PrimitiveType::any(),
                    is_variadic: false,
                })),
            ),
        );

        self.functions.insert(
            "getOS".to_string(),
            NativeFunctionValue::new(
                |_args: Vec<RuntimeValue>| {
                    let os = if cfg!(target_os = "windows") {
                        "windows"
                    } else if cfg!(target_os = "linux") {
                        "linux"
                    } else if cfg!(target_os = "macos") {
                        "darwin"
                    } else {
                        "unknown"
                    };
                    RuntimeValue::Str(StrValue::new(os.to_string()))
                },
                Type::Function(Box::new(FunctionType {
                    params: vec![],
                    return_type: PrimitiveType::str(),
                    is_variadic: false,
                })),
            ),
        );
    }

    fn register_http_primitives(&mut self) {
        let client = reqwest::Client::new();

        self.async_functions.insert(
            "native_http_request".to_string(),
            NativeAsyncFunctionValue::new(
                Arc::new(move |args: Vec<RuntimeValue>| {
                    let client = client.clone();

                    Box::pin(async move {
                        if args.len() < 2 {
                            return RuntimeValue::Null(NullValue::new());
                        }

                        let method_str = match &args[0] {
                            RuntimeValue::Str(s) => s.value.clone(),
                            _ => return RuntimeValue::Null(NullValue::new()),
                        };

                        let url = match &args[1] {
                            RuntimeValue::Str(s) => s.value.clone(),
                            _ => return RuntimeValue::Null(NullValue::new()),
                        };

                        let method = match method_str.as_str() {
                            "GET" => reqwest::Method::GET,
                            "POST" => reqwest::Method::POST,
                            "PUT" => reqwest::Method::PUT,
                            "DELETE" => reqwest::Method::DELETE,
                            _ => reqwest::Method::GET,
                        };

                        match client.request(method, url.clone()).send().await {
                            Ok(resp) => {
                                let status = resp.status().as_u16() as i64;
                                let status_text =
                                    resp.status().canonical_reason().unwrap_or("").to_string();

                                let mut headers_map_val = HashMap::new();
                                for (key, value) in resp.headers().iter() {
                                    if let Ok(val_str) = value.to_str() {
                                        headers_map_val.insert(
                                            key.to_string(),
                                            RuntimeValue::Str(StrValue::new(val_str.to_string())),
                                        );
                                    }
                                }

                                let body_val = match resp.json::<JsonValue>().await {
                                    Ok(json_body) => convert_serde_to_runtime(&json_body),
                                    Err(_) => RuntimeValue::Null(NullValue::new()),
                                };

                                let mut response_map = HashMap::new();
                                response_map.insert(
                                    "status".to_string(),
                                    RuntimeValue::Int(IntValue::new(status)),
                                );
                                response_map.insert(
                                    "statusText".to_string(),
                                    RuntimeValue::Str(StrValue::new(status_text)),
                                );
                                response_map.insert("body".to_string(), body_val);
                                response_map.insert(
                                    "url".to_string(),
                                    RuntimeValue::Str(StrValue::new(url)),
                                );
                                let headers_obj = RuntimeValue::Map(MapValue::new(
                                    headers_map_val,
                                    PrimitiveType::str(),
                                    PrimitiveType::str(),
                                ));
                                response_map.insert("headers".to_string(), headers_obj);

                                RuntimeValue::Object(ObjectValue::new(
                                    response_map,
                                    PrimitiveType::any(),
                                ))
                            }
                            Err(e) => {
                                let mut response_map = HashMap::new();
                                response_map.insert(
                                    "status".to_string(),
                                    RuntimeValue::Int(IntValue::new(500)),
                                );
                                response_map.insert(
                                    "statusText".to_string(),
                                    RuntimeValue::Str(StrValue::new(e.to_string())),
                                );
                                response_map.insert(
                                    "body".to_string(),
                                    RuntimeValue::Null(NullValue::new()),
                                );
                                response_map.insert(
                                    "url".to_string(),
                                    RuntimeValue::Str(StrValue::new(url)),
                                );
                                let headers_obj = RuntimeValue::Map(MapValue::new(
                                    HashMap::new(),
                                    PrimitiveType::str(),
                                    PrimitiveType::str(),
                                ));
                                response_map.insert("headers".to_string(), headers_obj);

                                RuntimeValue::Object(ObjectValue::new(
                                    response_map,
                                    PrimitiveType::any(),
                                ))
                            }
                        }
                    })
                }),
                Type::Function(Box::new(FunctionType {
                    params: vec![
                        PrimitiveType::str(),
                        PrimitiveType::str(),
                        PrimitiveType::any(),
                        PrimitiveType::any(),
                        PrimitiveType::int(),
                    ],
                    return_type: Type::Future(Box::new(FutureType {
                        inner_type: PrimitiveType::any(),
                    })),
                    is_variadic: false,
                })),
            ),
        );
    }

    pub fn register_all_in_env(&self, interp: &mut crate::interpreter::Interpreter) {
        // Register all synchronous native functions
        for (name, func) in &self.functions {
            let value = RuntimeValue::NativeFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }

        // Register all asynchronous native functions
        for (name, func) in &self.async_functions {
            let value = RuntimeValue::NativeAsyncFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }
    }
}

impl Default for NativeBridge {
    fn default() -> Self {
        Self::new()
    }
}

fn convert_serde_to_runtime(value: &JsonValue) -> RuntimeValue {
    match value {
        JsonValue::Null => RuntimeValue::Null(NullValue::new()),
        JsonValue::Bool(b) => RuntimeValue::Bool(BoolValue::new(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                RuntimeValue::Int(IntValue::new(i))
            } else if let Some(f) = n.as_f64() {
                RuntimeValue::Float(FloatValue::new(f))
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        }
        JsonValue::String(s) => RuntimeValue::Str(StrValue::new(s.clone())),
        JsonValue::Array(arr) => {
            let elements = arr.iter().map(convert_serde_to_runtime).collect();
            RuntimeValue::List(ListValue::new(elements, PrimitiveType::any()))
        }
        JsonValue::Object(obj) => {
            let properties = obj
                .iter()
                .map(|(k, v)| (k.clone(), convert_serde_to_runtime(v)))
                .collect();
            RuntimeValue::Object(ObjectValue::new(properties, PrimitiveType::any()))
        }
    }
}

fn convert_runtime_to_serde(value: &RuntimeValue) -> JsonValue {
    match value {
        RuntimeValue::Null(_) => JsonValue::Null,
        RuntimeValue::Bool(b) => JsonValue::Bool(b.value),
        RuntimeValue::Int(i) => JsonValue::Number(serde_json::Number::from(i.value)),
        RuntimeValue::Float(f) => serde_json::Number::from_f64(f.value)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        RuntimeValue::Str(s) => JsonValue::String(s.value.clone()),
        RuntimeValue::List(list) => {
            let arr = list.elements.iter().map(convert_runtime_to_serde).collect();
            JsonValue::Array(arr)
        }
        RuntimeValue::Object(obj) => {
            let map = obj
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), convert_runtime_to_serde(v)))
                .collect();
            JsonValue::Object(map)
        }
        RuntimeValue::Map(map) => {
            let obj = map
                .entries
                .iter()
                .map(|(k, v)| (k.clone(), convert_runtime_to_serde(v)))
                .collect();
            JsonValue::Object(obj)
        }
        _ => JsonValue::Null,
    }
}
