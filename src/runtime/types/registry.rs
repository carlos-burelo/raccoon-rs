use super::{CallbackExecutor, TypeHandler};
use super::{bool_type::BoolType, decimal_type::DecimalType, float_type::FloatType};
use super::{int_type::IntType, list_type::ListType, map_type::MapType, str_type::StrType};
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use std::collections::HashMap;

/// Central registry for all type handlers
/// This allows for dynamic lookup of methods based on runtime type
pub struct TypeRegistry {
    handlers: HashMap<String, Box<dyn TypeHandler>>,
}

impl TypeRegistry {
    /// Create a new type registry with all built-in types registered
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };

        // Register all built-in types
        registry.register(Box::new(IntType));
        registry.register(Box::new(FloatType));
        registry.register(Box::new(DecimalType));
        registry.register(Box::new(StrType));
        registry.register(Box::new(BoolType));
        registry.register(Box::new(ListType));
        registry.register(Box::new(MapType));

        registry
    }

    /// Register a new type handler
    pub fn register(&mut self, handler: Box<dyn TypeHandler>) {
        self.handlers
            .insert(handler.type_name().to_string(), handler);
    }

    /// Get a type handler by type name
    pub fn get_handler(&self, type_name: &str) -> Option<&dyn TypeHandler> {
        self.handlers.get(type_name).map(|h| h.as_ref())
    }

    /// Call an instance method on a runtime value
    pub fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let type_name = value.get_name();

        if let Some(handler) = self.get_handler(&type_name) {
            handler.call_instance_method(value, method, args, position, file)
        } else {
            Err(RaccoonError::new(
                format!("No type handler found for type '{}'", type_name),
                position,
                file,
            ))
        }
    }

    /// Call a static method on a type
    pub fn call_static_method(
        &self,
        type_name: &str,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let Some(handler) = self.get_handler(type_name) {
            handler.call_static_method(method, args, position, file)
        } else {
            Err(RaccoonError::new(
                format!("Type '{}' not found", type_name),
                position,
                file,
            ))
        }
    }

    /// Get a static property from a type
    pub fn get_static_property(
        &self,
        type_name: &str,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let Some(handler) = self.get_handler(type_name) {
            handler.get_static_property(property, position, file)
        } else {
            Err(RaccoonError::new(
                format!("Type '{}' not found", type_name),
                position,
                file,
            ))
        }
    }

    /// Check if a type has an instance method
    pub fn has_instance_method(&self, type_name: &str, method: &str) -> bool {
        self.get_handler(type_name)
            .map(|h| h.has_instance_method(method))
            .unwrap_or(false)
    }

    /// Check if a type has a static method
    pub fn has_static_method(&self, type_name: &str, method: &str) -> bool {
        self.get_handler(type_name)
            .map(|h| h.has_static_method(method))
            .unwrap_or(false)
    }

    /// Call an async instance method on a runtime value
    pub async fn call_async_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        callback_executor: &CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        let type_name = value.get_name();

        if let Some(handler) = self.get_handler(&type_name) {
            handler
                .call_async_instance_method(value, method, args, position, file, callback_executor)
                .await
        } else {
            Err(RaccoonError::new(
                format!("No type handler found for type '{}'", type_name),
                position,
                file,
            ))
        }
    }

    /// Check if a type has an async instance method
    pub fn has_async_instance_method(&self, type_name: &str, method: &str) -> bool {
        self.get_handler(type_name)
            .map(|h| h.has_async_instance_method(method))
            .unwrap_or(false)
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
