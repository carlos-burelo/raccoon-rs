pub mod bool_type;
pub mod decimal_type;
pub mod float_type;
pub mod int_type;
pub mod list_type;
pub mod map_type;
pub mod registry;
pub mod str_type;

use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

/// Type alias for async callback executor
/// Takes a function value and arguments, returns a future that resolves to a result
pub type CallbackExecutor = Box<
    dyn Fn(
            RuntimeValue,
            Vec<RuntimeValue>,
            Position,
        ) -> Pin<Box<dyn Future<Output = Result<RuntimeValue, RaccoonError>> + Send>>
        + Send
        + Sync,
>;

/// Trait that all type handlers must implement
/// This allows types to define their own instance and static methods
#[async_trait]
pub trait TypeHandler: Send + Sync {
    /// Get the name of the type (e.g., "str", "int", "bool")
    fn type_name(&self) -> &str;

    /// Call an instance method on a value of this type
    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError>;

    /// Call a static method on this type
    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError>;

    /// Get a static property of this type
    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!(
                "Static property '{}' not found on type '{}'",
                property,
                self.type_name()
            ),
            position,
            file,
        ))
    }

    /// Check if this type has an instance method
    fn has_instance_method(&self, method: &str) -> bool;

    /// Check if this type has a static method
    fn has_static_method(&self, method: &str) -> bool;

    /// Call an async instance method that requires callback execution
    /// This is used for methods like map, filter, reduce that need to call user functions
    async fn call_async_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        _callback_executor: &CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        // Default implementation: delegate to sync method
        self.call_instance_method(value, method, args, position, file)
    }

    /// Check if this type has an async instance method
    fn has_async_instance_method(&self, _method: &str) -> bool {
        // Default: no async methods
        false
    }
}
