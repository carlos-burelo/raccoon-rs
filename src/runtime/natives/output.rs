/// Output functions: print, eprint
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};
use crate::output_style;
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    let print_fn = NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let output = args
                .iter()
                .map(|arg| {
                    let plain = arg.to_string();
                    // Apply syntax highlighting to all output
                    output_style::format_value(&plain)
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
                    // Apply syntax highlighting if the output looks like JSON/objects/arrays
                    if plain.contains('{') || plain.contains('[') || plain.starts_with('"') {
                        output_style::format_value(&plain)
                    } else {
                        plain
                    }
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

    // Single convention: native_*
    // No more duplication with native_* aliases
    functions.insert("native_print".to_string(), print_fn);
    functions.insert("native_eprint".to_string(), eprint_fn);
}
