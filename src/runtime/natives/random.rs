/// Random number functions: native_random
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{FloatValue, NativeFunctionValue, RuntimeValue};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    functions.insert(
        "native_random".to_string(),
        NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default();
                let nanos = now.as_nanos() as u128;
                let val = (nanos as f64) / 1_000_000_000.0;
                RuntimeValue::Float(FloatValue::new(val))
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::float(),
                is_variadic: false,
            })),
        ),
    );
}
