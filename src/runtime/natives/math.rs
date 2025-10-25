/// Math functions: sqrt, pow, sin, cos, tan
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::*;
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // sqrt
    functions.insert(
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
    functions.insert(
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
    functions.insert(
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
    functions.insert(
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
    functions.insert(
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
}
