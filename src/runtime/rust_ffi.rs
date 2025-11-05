use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{
    BoolValue, FloatValue, IntValue, ListValue, NativeFunctionValue, RuntimeValue, StrValue,
};
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
        RuntimeValue::Int(IntValue::new(self))
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
        RuntimeValue::Int(IntValue::new(self as i64))
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
        RuntimeValue::Float(FloatValue::new(self))
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
        RuntimeValue::Bool(BoolValue::new(self))
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
        RuntimeValue::Str(StrValue::new(self))
    }
}

impl ToRaccoon for &str {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Str(StrValue::new(self.to_string()))
    }
}

impl<T: ToRaccoon> ToRaccoon for Vec<T> {
    fn to_runtime(self) -> RuntimeValue {
        let elements: Vec<RuntimeValue> = self.into_iter().map(|v| v.to_runtime()).collect();
        RuntimeValue::List(ListValue::new(elements, PrimitiveType::any()))
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

impl ToRaccoon for () {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Null(crate::runtime::values::NullValue::new())
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

pub type NativeFnWrapper = fn(Vec<RuntimeValue>) -> RuntimeValue;

pub struct RustFunction {
    pub name: String,
    pub implementation: NativeFnWrapper,
    pub signature: Type,
}

impl RustFunction {
    pub fn new(name: impl Into<String>, func: NativeFnWrapper, signature: Type) -> Self {
        Self {
            name: name.into(),
            implementation: func,
            signature,
        }
    }

    pub fn to_native_function(&self) -> NativeFunctionValue {
        NativeFunctionValue::new(self.implementation, self.signature.clone())
    }
}

pub struct RustFFIRegistry {
    functions: Arc<RwLock<HashMap<String, RustFunction>>>,
}

impl RustFFIRegistry {
    pub fn new() -> Self {
        Self {
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_raw(&self, name: impl Into<String>, func: NativeFnWrapper, signature: Type) {
        let name = name.into();
        let rust_func = RustFunction::new(name.clone(), func, signature);
        self.functions.write().unwrap().insert(name, rust_func);
    }

    pub fn get(&self, name: &str) -> Option<NativeFunctionValue> {
        let funcs = self.functions.read().unwrap();
        funcs.get(name).map(|f| f.to_native_function())
    }

    pub fn list(&self) -> Vec<String> {
        let funcs = self.functions.read().unwrap();
        funcs.keys().cloned().collect()
    }
}

impl Default for RustFFIRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FromRaccoonArgs: Sized {
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String>;
}

impl FromRaccoonArgs for () {
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Expected 0 arguments, got {}", args.len()));
        }
        Ok(())
    }
}

impl<T: FromRaccoon> FromRaccoonArgs for T {
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String> {
        if args.len() != 1 {
            return Err(format!("Expected 1 argument, got {}", args.len()));
        }
        T::from_runtime(&args[0])
    }
}

impl<T1: FromRaccoon, T2: FromRaccoon> FromRaccoonArgs for (T1, T2) {
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String> {
        if args.len() != 2 {
            return Err(format!("Expected 2 arguments, got {}", args.len()));
        }
        Ok((T1::from_runtime(&args[0])?, T2::from_runtime(&args[1])?))
    }
}

impl<T1: FromRaccoon, T2: FromRaccoon, T3: FromRaccoon> FromRaccoonArgs for (T1, T2, T3) {
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!("Expected 3 arguments, got {}", args.len()));
        }
        Ok((
            T1::from_runtime(&args[0])?,
            T2::from_runtime(&args[1])?,
            T3::from_runtime(&args[2])?,
        ))
    }
}

impl<T1: FromRaccoon, T2: FromRaccoon, T3: FromRaccoon, T4: FromRaccoon> FromRaccoonArgs
    for (T1, T2, T3, T4)
{
    fn from_runtime_args(args: &[RuntimeValue]) -> Result<Self, String> {
        if args.len() != 4 {
            return Err(format!("Expected 4 arguments, got {}", args.len()));
        }
        Ok((
            T1::from_runtime(&args[0])?,
            T2::from_runtime(&args[1])?,
            T3::from_runtime(&args[2])?,
            T4::from_runtime(&args[3])?,
        ))
    }
}
