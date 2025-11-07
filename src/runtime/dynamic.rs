use crate::ast::types::Type;
use crate::runtime::RuntimeValue;
use std::fmt;

/// Trait for dynamic runtime values that can be extended without modifying the RuntimeValue enum
/// This allows for user-defined types, async functions, and other extensible features
/// without hardcoding them as enum variants
pub trait DynamicValue: Send + Sync + fmt::Debug + DynamicValueClone {
    /// Get the type of this dynamic value
    fn get_type(&self) -> Type;

    /// Get the string representation of this value
    fn to_string(&self) -> String;

    /// Get a property from this value by name
    fn get_property(&self, name: &str) -> Option<RuntimeValue> {
        let _ = name;
        None
    }

    /// Set a property on this value by name
    fn set_property(&mut self, name: &str, value: RuntimeValue) -> Result<(), String> {
        let _ = (name, value);
        Err("This value does not support property assignment".to_string())
    }

    /// Call this value as a function with the given arguments
    fn call(&self, args: Vec<RuntimeValue>) -> Result<RuntimeValue, String> {
        let _ = args;
        Err("This value is not callable".to_string())
    }

    /// Clone this dynamic value into a boxed trait object
    fn clone_boxed(&self) -> Box<dyn DynamicValue>;

    /// Get the name of this dynamic value's type
    fn type_name(&self) -> &str;
}

/// Trait to enable cloning of trait objects
pub trait DynamicValueClone {
    fn clone_box(&self) -> Box<dyn DynamicValue>;
}

impl<T> DynamicValueClone for T
where
    T: 'static + DynamicValue + Clone,
{
    fn clone_box(&self) -> Box<dyn DynamicValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DynamicValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Convenience type alias for boxed dynamic values
pub type DynamicRuntimeValue = Box<dyn DynamicValue>;
