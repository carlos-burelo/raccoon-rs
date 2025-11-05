use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, RuntimeValue};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait FromRaccoon: Sized {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String>;
}

pub trait ToRaccoon {
    fn to_runtime(self) -> RuntimeValue;
}

impl FromRaccoon for i64 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Int(i) => Ok(i.value),
            _ => Err(format!("Expected int, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for i64 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Int(crate::runtime::values::IntValue::new(self))
    }
}

impl FromRaccoon for i32 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Int(i) => Ok(i.value as i32),
            _ => Err(format!("Expected int, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for i32 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Int(crate::runtime::values::IntValue::new(self as i64))
    }
}

impl FromRaccoon for f64 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Float(f) => Ok(f.value),
            RuntimeValue::Int(i) => Ok(i.value as f64),
            _ => Err(format!("Expected float, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for f64 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Float(crate::runtime::values::FloatValue::new(self))
    }
}

impl FromRaccoon for bool {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Bool(b) => Ok(b.value),
            _ => Err(format!("Expected bool, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for bool {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Bool(crate::runtime::values::BoolValue::new(self))
    }
}

impl FromRaccoon for String {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Str(s) => Ok(s.value.clone()),
            _ => Err(format!("Expected string, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for String {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Str(crate::runtime::values::StrValue::new(self))
    }
}

impl ToRaccoon for &str {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Str(crate::runtime::values::StrValue::new(self.to_string()))
    }
}

impl FromRaccoon for Vec<RuntimeValue> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::List(list) => Ok(list.elements.clone()),
            _ => Err(format!("Expected list, got {}", value.to_string())),
        }
    }
}

impl<T: FromRaccoon> FromRaccoon for Vec<T> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::List(list) => list
                .elements
                .iter()
                .map(|v| T::from_runtime(v))
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(format!("Expected list, got {}", value.to_string())),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Vec<T> {
    fn to_runtime(self) -> RuntimeValue {
        let elements: Vec<RuntimeValue> = self.into_iter().map(|v| v.to_runtime()).collect();
        RuntimeValue::List(crate::runtime::values::ListValue::new(
            elements,
            PrimitiveType::any(),
        ))
    }
}

impl<T: ToRaccoon> ToRaccoon for Option<T> {
    fn to_runtime(self) -> RuntimeValue {
        match self {
            Some(v) => v.to_runtime(),
            None => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
        }
    }
}

impl<T: FromRaccoon> FromRaccoon for Option<T> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Null(_) => Ok(None),
            other => T::from_runtime(other).map(Some),
        }
    }
}

impl<T: ToRaccoon, E: ToString> ToRaccoon for Result<T, E> {
    fn to_runtime(self) -> RuntimeValue {
        match self {
            Ok(v) => v.to_runtime(),
            Err(e) => RuntimeValue::Str(crate::runtime::values::StrValue::new(e.to_string())),
        }
    }
}

impl ToRaccoon for () {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Null(crate::runtime::values::NullValue::new())
    }
}

pub struct NativeRegistry {
    functions: Arc<RwLock<HashMap<String, NativeFunctionValue>>>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        NativeRegistry {
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(
        &self,
        name: &str,
        function: crate::runtime::values::NativeFn,
        param_types: Vec<Type>,
        return_type: Type,
    ) {
        let fn_type = Type::Function(Box::new(FunctionType {
            params: param_types,
            return_type: return_type,
            is_variadic: false,
        }));

        let native_fn = NativeFunctionValue::new(function, fn_type);

        self.functions
            .write()
            .unwrap()
            .insert(name.to_string(), native_fn);
    }

    pub fn get(&self, name: &str) -> Option<NativeFunctionValue> {
        self.functions.read().unwrap().get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.functions.read().unwrap().keys().cloned().collect()
    }

    pub fn export_all(&self) -> HashMap<String, NativeFunctionValue> {
        self.functions.read().unwrap().clone()
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NativeDecoratorProcessor;

impl NativeDecoratorProcessor {
    pub fn has_native_decorator(_decorators: &[crate::ast::nodes::DecoratorDecl]) -> bool {
        false
    }
}

#[macro_export]
macro_rules! register_native {
    ($registry:expr, $name:expr, $function:expr, $params:expr => $return_type:expr) => {
        $registry.register($name, $function, $params, $return_type)
    };
}
