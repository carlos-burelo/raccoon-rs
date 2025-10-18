use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{NullValue, RuntimeValue};
use std::collections::HashMap;

pub struct ConsoleModule;

impl ConsoleModule {
    pub fn name() -> &'static str {
        "std:console"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        let log_fn = Self::create_log_fn();
        exports.insert("log".to_string(), log_fn.clone());
        exports.insert("error".to_string(), Self::create_error_fn());
        exports.insert("warn".to_string(), Self::create_warn_fn());
        exports.insert("default".to_string(), log_fn);

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "log" => Some(Self::create_log_fn()),
            "error" => Some(Self::create_error_fn()),
            "warn" => Some(Self::create_warn_fn()),
            "default" => Some(Self::create_log_fn()),
            _ => None,
        }
    }

    fn create_log_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        ))
    }

    fn create_error_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                eprintln!("ERROR: {}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        ))
    }

    fn create_warn_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                eprintln!("WARN: {}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        ))
    }
}
