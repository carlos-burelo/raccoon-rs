use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{BoolValue, FloatValue, IntValue, NullValue, RuntimeValue};
use std::collections::HashMap;
use std::sync::Mutex;

// Simple LCG (Linear Congruential Generator) for random number generation
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: seed }
    }

    fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // LCG parameters (from Numerical Recipes)
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    fn next_float(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX as f64)
    }

    fn next_range(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next() % range) as i64
    }
}

lazy_static::lazy_static! {
    static ref RNG: Mutex<SimpleRng> = Mutex::new(SimpleRng::new());
}

pub struct RandomModule;

impl RandomModule {
    pub fn name() -> &'static str {
        "std:random"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("random".to_string(), Self::create_random_fn());
        exports.insert("randomInt".to_string(), Self::create_random_int_fn());
        exports.insert("randomRange".to_string(), Self::create_random_range_fn());
        exports.insert("randomBool".to_string(), Self::create_random_bool_fn());
        exports.insert("setSeed".to_string(), Self::create_set_seed_fn());
        exports.insert("choice".to_string(), Self::create_choice_fn());
        exports.insert("shuffle".to_string(), Self::create_shuffle_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "random" => Some(Self::create_random_fn()),
            "randomInt" => Some(Self::create_random_int_fn()),
            "randomRange" => Some(Self::create_random_range_fn()),
            "randomBool" => Some(Self::create_random_bool_fn()),
            "setSeed" => Some(Self::create_set_seed_fn()),
            "choice" => Some(Self::create_choice_fn()),
            "shuffle" => Some(Self::create_shuffle_fn()),
            _ => None,
        }
    }

    fn create_random_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let mut rng = RNG.lock().unwrap();
                let value = rng.next_float();
                RuntimeValue::Float(FloatValue::new(value))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ))
    }

    fn create_random_int_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let max = if args.is_empty() {
                    100
                } else {
                    match &args[0] {
                        RuntimeValue::Int(i) => i.value,
                        _ => 100,
                    }
                };

                let mut rng = RNG.lock().unwrap();
                let value = rng.next_range(0, max);
                RuntimeValue::Int(IntValue::new(value))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::int()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_random_range_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Int(IntValue::new(0));
                }

                let min = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => 0,
                };

                let max = match &args[1] {
                    RuntimeValue::Int(i) => i.value,
                    _ => 100,
                };

                let mut rng = RNG.lock().unwrap();
                let value = rng.next_range(min, max);
                RuntimeValue::Int(IntValue::new(value))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::int(), PrimitiveType::int()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_random_bool_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let mut rng = RNG.lock().unwrap();
                let value = rng.next() % 2 == 0;
                RuntimeValue::Bool(BoolValue::new(value))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_set_seed_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::Int(i) => {
                        let mut rng = RNG.lock().unwrap();
                        *rng = SimpleRng::with_seed(i.value as u64);
                        RuntimeValue::Null(NullValue::new())
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::int()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ))
    }

    fn create_choice_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        if list.elements.is_empty() {
                            return RuntimeValue::Null(NullValue::new());
                        }

                        let mut rng = RNG.lock().unwrap();
                        let index = rng.next_range(0, list.elements.len() as i64) as usize;
                        list.elements[index].clone()
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::any()],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ))
    }

    fn create_shuffle_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut elements = list.elements.clone();
                        let mut rng = RNG.lock().unwrap();

                        // Fisher-Yates shuffle
                        for i in (1..elements.len()).rev() {
                            let j = rng.next_range(0, (i + 1) as i64) as usize;
                            elements.swap(i, j);
                        }

                        RuntimeValue::List(crate::runtime::ListValue::new(
                            elements,
                            list.element_type.clone(),
                        ))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::any()],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ))
    }
}
