use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};

use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    let print_fn = NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let output = args
                .iter()
                .map(|arg| {
                    let plain = arg.to_string();

                    plain
                })
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
                .map(|arg| {
                    let plain = arg.to_string();
                    plain
                })
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

    functions.insert("native_print".to_string(), print_fn);
    functions.insert("native_eprint".to_string(), eprint_fn);
}
