use crate::ast::types::Type;
use crate::runtime::RuntimeValue;
use std::fmt;

pub trait DynamicValue: Send + Sync + fmt::Debug + DynamicValueClone {
    fn get_type(&self) -> Type;

    fn to_string(&self) -> String;

    fn get_property(&self, name: &str) -> Option<RuntimeValue> {
        let _ = name;
        None
    }

    fn set_property(&mut self, name: &str, value: RuntimeValue) -> Result<(), String> {
        let _ = (name, value);
        Err("This value does not support property assignment".to_string())
    }

    fn call(&self, args: Vec<RuntimeValue>) -> Result<RuntimeValue, String> {
        let _ = args;
        Err("This value is not callable".to_string())
    }

    fn clone_boxed(&self) -> Box<dyn DynamicValue>;

    fn type_name(&self) -> &str;
}

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

pub type DynamicRuntimeValue = Box<dyn DynamicValue>;
