/// System utility functions
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, RuntimeValue, StrValue};
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // getOS - returns current operating system
    functions.insert(
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
