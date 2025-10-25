/// Time functions: native_time_now, native_time_now_secs, native_time_sleep, native_time_format
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{IntValue, NativeFunctionValue, RuntimeValue, StrValue};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    functions.insert(
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

    functions.insert(
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

    functions.insert(
        "native_time_sleep".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Int(ms)) = args.first() {
                    let duration = Duration::from_millis(ms.value.max(0) as u64);
                    thread::sleep(duration);
                }
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::int()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ),
    );

    functions.insert(
        "native_time_format".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                // Simple timestamp to string format
                if args.len() < 2 {
                    return RuntimeValue::Str(StrValue::new("".to_string()));
                }

                let _timestamp = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => return RuntimeValue::Str(StrValue::new("".to_string())),
                };

                let _format_str = match &args[1] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Str(StrValue::new("".to_string())),
                };

                // For now, just return ISO-like format
                // In a real implementation, this would parse the format string
                RuntimeValue::Str(StrValue::new(format!("{}", _timestamp)))
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::int(), PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );
}
