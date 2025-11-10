use crate::runtime::{
    BoolValue, FloatValue, IntValue, ListValue, NullValue, RuntimeValue, StrValue,
};

/// Trait for converting Raccoon RuntimeValue to Rust types
pub trait FromRaccoon: Sized {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String>;
}

/// Trait for converting Rust types to Raccoon RuntimeValue
pub trait ToRaccoon {
    fn to_raccoon(self) -> RuntimeValue;
}

// ========== f64 ==========
impl FromRaccoon for f64 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Float(f) => Ok(f.value),
            RuntimeValue::Int(i) => Ok(i.value as f64),
            RuntimeValue::Bool(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            _ => Err("Expected number".into()),
        }
    }
}

impl ToRaccoon for f64 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Float(FloatValue::new(self))
    }
}

// ========== i32 ==========
impl FromRaccoon for i32 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Int(i) => Ok(i.value as i32),
            RuntimeValue::Float(f) => Ok(f.value as i32),
            RuntimeValue::Bool(b) => Ok(if b.value { 1 } else { 0 }),
            _ => Err("Expected integer".into()),
        }
    }
}

impl ToRaccoon for i32 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Int(IntValue::new(self as i64))
    }
}

// ========== i64 ==========
impl FromRaccoon for i64 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Int(i) => Ok(i.value),
            RuntimeValue::Float(f) => Ok(f.value as i64),
            RuntimeValue::Bool(b) => Ok(if b.value { 1 } else { 0 }),
            _ => Err("Expected integer".into()),
        }
    }
}

impl ToRaccoon for i64 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Int(IntValue::new(self))
    }
}

// ========== bool ==========
impl FromRaccoon for bool {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Bool(b) => Ok(b.value),
            RuntimeValue::Int(i) => Ok(i.value != 0),
            RuntimeValue::Float(f) => Ok(f.value != 0.0),
            _ => Err("Expected boolean".into()),
        }
    }
}

impl ToRaccoon for bool {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Bool(BoolValue::new(self))
    }
}

// ========== String ==========
impl FromRaccoon for String {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Str(s) => Ok(s.value.clone()),
            _ => Err("Expected string".into()),
        }
    }
}

impl ToRaccoon for String {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Str(StrValue::new(self))
    }
}

// ========== Vec<T> ==========
impl<T: FromRaccoon> FromRaccoon for Vec<T> {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::List(list) => list.elements.iter().map(|v| T::from_raccoon(v)).collect(),
            _ => Err("Expected list".into()),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Vec<T> {
    fn to_raccoon(self) -> RuntimeValue {
        let elements = self.into_iter().map(|v| v.to_raccoon()).collect();
        RuntimeValue::List(ListValue::new(
            elements,
            crate::ast::types::PrimitiveType::any(),
        ))
    }
}

// ========== Option<T> ==========
impl<T: FromRaccoon> FromRaccoon for Option<T> {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Null(_) => Ok(None),
            other => T::from_raccoon(other).map(Some),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Option<T> {
    fn to_raccoon(self) -> RuntimeValue {
        match self {
            Some(v) => v.to_raccoon(),
            None => RuntimeValue::Null(NullValue::new()),
        }
    }
}
