/// Stdlib native functions - inmutable operations only for now
/// Mutable array operations require architectural changes to ListValue

use crate::ast::types::PrimitiveType;
use crate::runtime::{BoolValue, Environment, FloatValue, IntValue, ListValue, StrValue};
use crate::runtime::RuntimeValue;
use crate::{fn_type, native_fn, native_fn_variadic, native_functions, null_return};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_all_stdlib_natives(env: &mut Environment) {
    native_functions! {
        env,

        "native_print" => native_fn_variadic!(|args: Vec<RuntimeValue>| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 { print!(" "); }
                print!("{}", arg.to_string());
            }
            println!();
            null_return!()
        }),

        // MATH
        "native_sqrt" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            let val = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            RuntimeValue::Float(FloatValue::new(val.sqrt()))
        }, PrimitiveType::float() => PrimitiveType::float()),

        "native_pow" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let base = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            let exp = match &args[1] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            RuntimeValue::Float(FloatValue::new(base.powf(exp)))
        }, PrimitiveType::float() => PrimitiveType::float()),

        "native_sin" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            let val = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            RuntimeValue::Float(FloatValue::new(val.sin()))
        }, PrimitiveType::float() => PrimitiveType::float()),

        "native_cos" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            let val = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            RuntimeValue::Float(FloatValue::new(val.cos()))
        }, PrimitiveType::float() => PrimitiveType::float()),

        "native_tan" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            let val = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return null_return!()
            };
            RuntimeValue::Float(FloatValue::new(val.tan()))
        }, PrimitiveType::float() => PrimitiveType::float()),

        "native_random" => native_fn!(|_args: Vec<RuntimeValue>| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let nanos = now.as_nanos() as u128;
            let val = ((nanos % 1000000) as f64) / 1000000.0;
            RuntimeValue::Float(FloatValue::new(val))
        }, PrimitiveType::void() => PrimitiveType::float()),

        // STRING
        "native_str_length" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
                _ => null_return!()
            }
        }, PrimitiveType::str() => PrimitiveType::int()),

        "native_str_upper" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Str(StrValue::new(s.value.to_uppercase())),
                _ => null_return!()
            }
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_lower" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Str(StrValue::new(s.value.to_lowercase())),
                _ => null_return!()
            }
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_trim" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Str(StrValue::new(s.value.trim().to_string())),
                _ => null_return!()
            }
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_substring" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 3 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let start = match &args[1] {
                RuntimeValue::Int(i) => (i.value.max(0) as usize).min(s.len()),
                _ => return null_return!()
            };
            let end = match &args[2] {
                RuntimeValue::Int(i) => (i.value.max(0) as usize).min(s.len()),
                _ => return null_return!()
            };
            let result = if start <= end {
                s.chars().skip(start).take(end - start).collect::<String>()
            } else {
                String::new()
            };
            RuntimeValue::Str(StrValue::new(result))
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_char_at" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let index = match &args[1] {
                RuntimeValue::Int(i) => i.value as usize,
                _ => return null_return!()
            };
            let result = s.chars().nth(index).map(|c| c.to_string()).unwrap_or_default();
            RuntimeValue::Str(StrValue::new(result))
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_index_of" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let substr = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let index = s.find(substr).map(|i| i as i64).unwrap_or(-1);
            RuntimeValue::Int(IntValue::new(index))
        }, PrimitiveType::str() => PrimitiveType::int()),

        "native_str_replace" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 3 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let from = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let to = match &args[2] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            RuntimeValue::Str(StrValue::new(s.replace(from, to)))
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_split" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let delimiter = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let parts: Vec<RuntimeValue> = s.split(delimiter)
                .map(|part| RuntimeValue::Str(StrValue::new(part.to_string())))
                .collect();
            RuntimeValue::List(ListValue::new(parts, PrimitiveType::str()))
        }, PrimitiveType::str() => PrimitiveType::str()),

        "native_str_starts_with" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let prefix = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            RuntimeValue::Bool(BoolValue::new(s.starts_with(prefix)))
        }, PrimitiveType::str() => PrimitiveType::bool()),

        "native_str_ends_with" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let s = match &args[0] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let suffix = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            RuntimeValue::Bool(BoolValue::new(s.ends_with(suffix)))
        }, PrimitiveType::str() => PrimitiveType::bool()),

        "native_str_join" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 2 { return null_return!(); }
            let parts = match &args[0] {
                RuntimeValue::List(l) => &l.elements,
                _ => return null_return!()
            };
            let delimiter = match &args[1] {
                RuntimeValue::Str(s) => &s.value,
                _ => return null_return!()
            };
            let strings: Vec<String> = parts.iter().map(|v| v.to_string()).collect();
            RuntimeValue::Str(StrValue::new(strings.join(delimiter)))
        }, PrimitiveType::any() => PrimitiveType::str()),

        // ARRAY (immutable operations only)
        "native_array_length" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::List(l) => RuntimeValue::Int(IntValue::new(l.elements.len() as i64)),
                _ => null_return!()
            }
        }, PrimitiveType::any() => PrimitiveType::int()),

        "native_array_slice" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.len() < 3 { return null_return!(); }
            let arr = match &args[0] {
                RuntimeValue::List(l) => l,
                _ => return null_return!()
            };
            let start = match &args[1] {
                RuntimeValue::Int(i) => (i.value.max(0) as usize).min(arr.elements.len()),
                _ => return null_return!()
            };
            let end = match &args[2] {
                RuntimeValue::Int(i) => (i.value.max(0) as usize).min(arr.elements.len()),
                _ => return null_return!()
            };
            let sliced = if start <= end {
                arr.elements[start..end].to_vec()
            } else {
                vec![]
            };
            RuntimeValue::List(ListValue::new(sliced, arr.element_type.clone()))
        }, PrimitiveType::any() => PrimitiveType::any()),

        // Note: Mutable array operations (push, pop, shift, unshift, reverse, sort)
        // require ListValue.elements to use Arc<RwLock<>> for shared mutability.
        // These are not implemented yet to avoid architectural changes.
        // Use list.push(), list.pop() methods instead which have &mut access.

        "native_array_push" => native_fn!(|_args: Vec<RuntimeValue>| {
            eprintln!("Warning: native_array_push requires ListValue refactor - use list.push() method instead");
            null_return!()
        }, PrimitiveType::any() => PrimitiveType::void()),

        "native_array_pop" => native_fn!(|_args: Vec<RuntimeValue>| {
            eprintln!("Warning: native_array_pop requires ListValue refactor - use list.pop() method instead");
            null_return!()
        }, PrimitiveType::any() => PrimitiveType::any()),

        "native_array_shift" => native_fn!(|_args: Vec<RuntimeValue>| {
            eprintln!("Warning: native_array_shift not yet implemented");
            null_return!()
        }, PrimitiveType::any() => PrimitiveType::any()),

        "native_array_unshift" => native_fn!(|_args: Vec<RuntimeValue>| {
            eprintln!("Warning: native_array_unshift not yet implemented");
            null_return!()
        }, PrimitiveType::any() => PrimitiveType::void()),

        "native_array_reverse" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::List(l) => {
                    let mut elements = l.elements.clone();
                    elements.reverse();
                    RuntimeValue::List(ListValue::new(elements, l.element_type.clone()))
                }
                _ => null_return!()
            }
        }, PrimitiveType::any() => PrimitiveType::any()),

        "native_array_sort" => native_fn!(|args: Vec<RuntimeValue>| {
            if args.is_empty() { return null_return!(); }
            match &args[0] {
                RuntimeValue::List(l) => {
                    let mut elements = l.elements.clone();
                    elements.sort_by(|a, b| {
                        match (a, b) {
                            (RuntimeValue::Int(x), RuntimeValue::Int(y)) => x.value.cmp(&y.value),
                            (RuntimeValue::Float(x), RuntimeValue::Float(y)) => {
                                x.value.partial_cmp(&y.value).unwrap_or(std::cmp::Ordering::Equal)
                            }
                            (RuntimeValue::Str(x), RuntimeValue::Str(y)) => x.value.cmp(&y.value),
                            _ => std::cmp::Ordering::Equal
                        }
                    });
                    RuntimeValue::List(ListValue::new(elements, l.element_type.clone()))
                }
                _ => null_return!()
            }
        }, PrimitiveType::any() => PrimitiveType::any())
    }
}
