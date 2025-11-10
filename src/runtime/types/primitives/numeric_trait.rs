/// Generic numeric type handler
/// Unifies behavior for all numeric types (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64)
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{DecimalValue, FloatValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

/// Trait for numeric type bounds
pub trait NumericBounds: Copy + Display + FromStr + Send + Sync + 'static {
    const TYPE_NAME: &'static str;
    const DESCRIPTION: &'static str;
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;

    fn to_i64(self) -> i64;
    fn from_i64(val: i64) -> Self;
    fn to_f64(self) -> f64;
    fn abs_value(self) -> Self;
}

// Implement for signed integers
macro_rules! impl_numeric_bounds_signed {
    ($($t:ty => $name:expr, $desc:expr),*) => {
        $(
            impl NumericBounds for $t {
                const TYPE_NAME: &'static str = $name;
                const DESCRIPTION: &'static str = $desc;
                const MIN_VALUE: Self = <$t>::MIN;
                const MAX_VALUE: Self = <$t>::MAX;

                fn to_i64(self) -> i64 {
                    self as i64
                }

                fn from_i64(val: i64) -> Self {
                    val as Self
                }

                fn to_f64(self) -> f64 {
                    self as f64
                }

                fn abs_value(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

// Implement for unsigned integers
macro_rules! impl_numeric_bounds_unsigned {
    ($($t:ty => $name:expr, $desc:expr),*) => {
        $(
            impl NumericBounds for $t {
                const TYPE_NAME: &'static str = $name;
                const DESCRIPTION: &'static str = $desc;
                const MIN_VALUE: Self = <$t>::MIN;
                const MAX_VALUE: Self = <$t>::MAX;

                fn to_i64(self) -> i64 {
                    self as i64
                }

                fn from_i64(val: i64) -> Self {
                    val as Self
                }

                fn to_f64(self) -> f64 {
                    self as f64
                }

                fn abs_value(self) -> Self {
                    self
                }
            }
        )*
    };
}

// Implement for floats
macro_rules! impl_numeric_bounds_float {
    ($($t:ty => $name:expr, $desc:expr),*) => {
        $(
            impl NumericBounds for $t {
                const TYPE_NAME: &'static str = $name;
                const DESCRIPTION: &'static str = $desc;
                const MIN_VALUE: Self = <$t>::MIN;
                const MAX_VALUE: Self = <$t>::MAX;

                fn to_i64(self) -> i64 {
                    self as i64
                }

                fn from_i64(val: i64) -> Self {
                    val as Self
                }

                fn to_f64(self) -> f64 {
                    self as f64
                }

                fn abs_value(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

impl_numeric_bounds_signed! {
    i8 => "i8", "8-bit signed integer",
    i16 => "i16", "16-bit signed integer",
    i32 => "i32", "32-bit signed integer",
    i64 => "i64", "64-bit signed integer"
}

impl_numeric_bounds_unsigned! {
    u8 => "u8", "8-bit unsigned integer",
    u16 => "u16", "16-bit unsigned integer",
    u32 => "u32", "32-bit unsigned integer",
    u64 => "u64", "64-bit unsigned integer"
}

impl_numeric_bounds_float! {
    f32 => "f32", "32-bit floating point number",
    f64 => "f64", "64-bit floating point number"
}

/// Generic numeric type handler
pub struct NumericHandler<T: NumericBounds> {
    _phantom: PhantomData<T>,
}

impl<T: NumericBounds> NumericHandler<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    fn extract_value(&self, value: &RuntimeValue) -> Result<T, ()> {
        match value {
            RuntimeValue::Int(i) => Ok(T::from_i64(i.value)),
            RuntimeValue::Float(f) => Ok(T::from_i64(f.value as i64)),
            _ => Err(()),
        }
    }
}

impl<T: NumericBounds> Default for NumericHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T: NumericBounds> TypeHandler for NumericHandler<T>
where
    <T as FromStr>::Err: Display,
{
    fn type_name(&self) -> &str {
        T::TYPE_NAME
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let num = self.extract_value(value).map_err(|_| {
            RaccoonError::new(
                format!("Expected {}, got {}", T::TYPE_NAME, value.get_name()),
                position,
                file.clone(),
            )
        })?;

        match method {
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(num.to_string()))),
            "toInt" => Ok(RuntimeValue::Int(IntValue::new(num.to_i64()))),
            "toFloat" => Ok(RuntimeValue::Float(FloatValue::new(num.to_f64()))),
            "toDecimal" => Ok(RuntimeValue::Decimal(DecimalValue::new(num.to_f64()))),
            "abs" => Ok(RuntimeValue::Int(IntValue::new(num.abs_value().to_i64()))),
            _ => Err(method_not_found_error(T::TYPE_NAME, method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "parse" => {
                require_args(&args, 1, "parse", position, file.clone())?;
                let s = extract_str(&args[0], "value", position, file.clone())?;

                match s.trim().parse::<T>() {
                    Ok(num) => Ok(RuntimeValue::Int(IntValue::new(num.to_i64()))),
                    Err(e) => Err(RaccoonError::new(
                        format!("Failed to parse '{}' as {}: {}", s, T::TYPE_NAME, e),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(static_method_not_found_error(
                T::TYPE_NAME,
                method,
                position,
                file,
            )),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "maxValue" => Ok(RuntimeValue::Int(IntValue::new(T::MAX_VALUE.to_i64()))),
            "minValue" => Ok(RuntimeValue::Int(IntValue::new(T::MIN_VALUE.to_i64()))),
            _ => Err(property_not_found_error(
                T::TYPE_NAME,
                property,
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(method, "toStr" | "toInt" | "toFloat" | "toDecimal" | "abs")
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "parse")
    }
}

// Type aliases for each numeric type
pub type I8Handler = NumericHandler<i8>;
pub type I16Handler = NumericHandler<i16>;
pub type I32Handler = NumericHandler<i32>;
pub type I64Handler = NumericHandler<i64>;
pub type U8Handler = NumericHandler<u8>;
pub type U16Handler = NumericHandler<u16>;
pub type U32Handler = NumericHandler<u32>;
pub type U64Handler = NumericHandler<u64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_bounds() {
        assert_eq!(i8::TYPE_NAME, "i8");
        assert_eq!(i16::TYPE_NAME, "i16");
        assert_eq!(u8::MIN_VALUE, 0);
        assert_eq!(i8::MIN_VALUE, -128);
    }

    #[test]
    fn test_handler_creation() {
        let handler = I8Handler::new();
        assert_eq!(handler.type_name(), "i8");

        let handler = U32Handler::new();
        assert_eq!(handler.type_name(), "u32");
    }
}
