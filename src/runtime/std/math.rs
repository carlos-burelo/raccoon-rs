use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{FloatValue, NullValue, RuntimeValue};
use std::collections::HashMap;
use std::f64::consts::{E, PI};

pub struct MathModule;

impl MathModule {
    pub fn name() -> &'static str {
        "std:math"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("PI".to_string(), RuntimeValue::Float(FloatValue::new(PI)));
        exports.insert("E".to_string(), RuntimeValue::Float(FloatValue::new(E)));

        exports.insert("sqrt".to_string(), Self::create_sqrt_fn());
        exports.insert("pow".to_string(), Self::create_pow_fn());
        exports.insert("abs".to_string(), Self::create_abs_fn());
        exports.insert("floor".to_string(), Self::create_floor_fn());
        exports.insert("ceil".to_string(), Self::create_ceil_fn());
        exports.insert("round".to_string(), Self::create_round_fn());
        exports.insert("min".to_string(), Self::create_min_fn());
        exports.insert("max".to_string(), Self::create_max_fn());
        exports.insert("sin".to_string(), Self::create_sin_fn());
        exports.insert("cos".to_string(), Self::create_cos_fn());
        exports.insert("tan".to_string(), Self::create_tan_fn());
        exports.insert("asin".to_string(), Self::create_asin_fn());
        exports.insert("acos".to_string(), Self::create_acos_fn());
        exports.insert("atan".to_string(), Self::create_atan_fn());
        exports.insert("atan2".to_string(), Self::create_atan2_fn());
        exports.insert("random".to_string(), Self::create_random_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "PI" => Some(RuntimeValue::Float(FloatValue::new(PI))),
            "E" => Some(RuntimeValue::Float(FloatValue::new(E))),
            "sqrt" => Some(Self::create_sqrt_fn()),
            "pow" => Some(Self::create_pow_fn()),
            "abs" => Some(Self::create_abs_fn()),
            "floor" => Some(Self::create_floor_fn()),
            "ceil" => Some(Self::create_ceil_fn()),
            "round" => Some(Self::create_round_fn()),
            "min" => Some(Self::create_min_fn()),
            "max" => Some(Self::create_max_fn()),
            "sin" => Some(Self::create_sin_fn()),
            "cos" => Some(Self::create_cos_fn()),
            "tan" => Some(Self::create_tan_fn()),
            "asin" => Some(Self::create_asin_fn()),
            "acos" => Some(Self::create_acos_fn()),
            "atan" => Some(Self::create_atan_fn()),
            "atan2" => Some(Self::create_atan2_fn()),
            "random" => Some(Self::create_random_fn()),
            _ => None,
        }
    }

    fn create_sqrt_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.sqrt()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_pow_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let base = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let exp = match &args[1] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(base.powf(exp)))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float(), PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_abs_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Int(i) => {
                        RuntimeValue::Float(FloatValue::new((i.value as f64).abs()))
                    }
                    RuntimeValue::Float(f) => RuntimeValue::Float(FloatValue::new(f.value.abs())),
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_floor_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => {
                        return RuntimeValue::Float(FloatValue::new(i.value as f64));
                    }
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.floor()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_ceil_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => {
                        return RuntimeValue::Float(FloatValue::new(i.value as f64));
                    }
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.ceil()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_round_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => {
                        return RuntimeValue::Float(FloatValue::new(i.value as f64));
                    }
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.round()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_min_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let mut min_val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                for arg in &args[1..] {
                    let val = match arg {
                        RuntimeValue::Int(i) => i.value as f64,
                        RuntimeValue::Float(f) => f.value,
                        _ => continue,
                    };
                    if val < min_val {
                        min_val = val;
                    }
                }
                RuntimeValue::Float(FloatValue::new(min_val))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float(), PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: true,
            })),
        ))
    }

    fn create_max_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let mut max_val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                for arg in &args[1..] {
                    let val = match arg {
                        RuntimeValue::Int(i) => i.value as f64,
                        RuntimeValue::Float(f) => f.value,
                        _ => continue,
                    };
                    if val > max_val {
                        max_val = val;
                    }
                }
                RuntimeValue::Float(FloatValue::new(max_val))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float(), PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: true,
            })),
        ))
    }

    fn create_sin_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.sin()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_cos_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.cos()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_tan_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.tan()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_asin_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.asin()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_acos_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.acos()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_atan_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let val = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(val.atan()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_atan2_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                let y = match &args[0] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                let x = match &args[1] {
                    RuntimeValue::Int(i) => i.value as f64,
                    RuntimeValue::Float(f) => f.value,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(y.atan2(x)))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::float(), PrimitiveType::float()],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_random_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let nanos = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos();
                let val = (nanos as f64) / 1_000_000_000.0;
                RuntimeValue::Float(FloatValue::new(val))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }
}
