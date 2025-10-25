/// Output functions: print, eprint
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
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

    // Single convention: native_*
    // No more duplication with _*_native aliases
    functions.insert("native_print".to_string(), print_fn);
    functions.insert("native_eprint".to_string(), eprint_fn);
}
