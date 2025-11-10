use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::Registrar;
/// Wrapper functions that expose native module functions to stdlib .rcc files
/// These are registered with simple names that can be called from within the stdlib modules
use crate::runtime::{Environment, NativeFunctionValue, RuntimeValue};
use std::sync::{Arc, Mutex};

pub fn register_stdlib_wrappers(env: &mut Environment, registrar: Arc<Mutex<Registrar>>) {
    // Math wrappers
    register_math_wrappers(env, registrar.clone());
    // JSON wrappers
    register_json_wrappers(env, registrar.clone());
    // HTTP wrappers
    register_http_wrappers(env, registrar);
}

fn register_math_wrappers(env: &mut Environment, _registrar: Arc<Mutex<Registrar>>) {
    // sqrt wrapper
    let sqrt_fn = NativeFunctionValue::new(
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Float(f) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new(f.value.sqrt()))
                }
                RuntimeValue::Int(i) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new((i.value as f64).sqrt()))
                }
                _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            ))],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_sqrt".to_string(),
        RuntimeValue::NativeFunction(sqrt_fn),
    );

    // pow wrapper
    let pow_fn = NativeFunctionValue::new(
        |args| {
            if args.len() < 2 {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            let base = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return RuntimeValue::Null(crate::runtime::NullValue::new()),
            };
            let exp = match &args[1] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return RuntimeValue::Null(crate::runtime::NullValue::new()),
            };
            RuntimeValue::Float(crate::runtime::FloatValue::new(base.powf(exp)))
        },
        Type::Function(Box::new(FunctionType {
            params: vec![
                Type::Primitive(PrimitiveType::new(
                    crate::ast::types::TypeKind::Float,
                    "float",
                )),
                Type::Primitive(PrimitiveType::new(
                    crate::ast::types::TypeKind::Float,
                    "float",
                )),
            ],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_pow".to_string(),
        RuntimeValue::NativeFunction(pow_fn),
    );

    // sin, cos, tan wrappers
    let sin_fn = NativeFunctionValue::new(
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Float(f) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new(f.value.sin()))
                }
                RuntimeValue::Int(i) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new((i.value as f64).sin()))
                }
                _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            ))],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_sin".to_string(),
        RuntimeValue::NativeFunction(sin_fn),
    );

    let cos_fn = NativeFunctionValue::new(
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Float(f) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new(f.value.cos()))
                }
                RuntimeValue::Int(i) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new((i.value as f64).cos()))
                }
                _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            ))],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_cos".to_string(),
        RuntimeValue::NativeFunction(cos_fn),
    );

    let tan_fn = NativeFunctionValue::new(
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Float(f) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new(f.value.tan()))
                }
                RuntimeValue::Int(i) => {
                    RuntimeValue::Float(crate::runtime::FloatValue::new((i.value as f64).tan()))
                }
                _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            ))],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_tan".to_string(),
        RuntimeValue::NativeFunction(tan_fn),
    );

    let log_fn = NativeFunctionValue::new(
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(crate::runtime::NullValue::new());
            }
            let x = match &args[0] {
                RuntimeValue::Float(f) => f.value,
                RuntimeValue::Int(i) => i.value as f64,
                _ => return RuntimeValue::Null(crate::runtime::NullValue::new()),
            };
            let base = if args.len() > 1 {
                match &args[1] {
                    RuntimeValue::Float(f) => f.value,
                    RuntimeValue::Int(i) => i.value as f64,
                    _ => std::f64::consts::E,
                }
            } else {
                std::f64::consts::E
            };
            RuntimeValue::Float(crate::runtime::FloatValue::new(x.log(base)))
        },
        Type::Function(Box::new(FunctionType {
            params: vec![Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            ))],
            return_type: Type::Primitive(PrimitiveType::new(
                crate::ast::types::TypeKind::Float,
                "float",
            )),
            is_variadic: false,
        })),
    );
    let _ = env.declare(
        "_native_log".to_string(),
        RuntimeValue::NativeFunction(log_fn),
    );
}

fn register_json_wrappers(_env: &mut Environment, _registrar: Arc<Mutex<Registrar>>) {
    // JSON functions will use native implementations directly
    // For now, we'll skip these and implement them in the .rcc file differently
}

fn register_http_wrappers(_env: &mut Environment, _registrar: Arc<Mutex<Registrar>>) {
    // HTTP functions will use native implementations directly
    // For now, we'll skip these and implement them in the .rcc file differently
}
