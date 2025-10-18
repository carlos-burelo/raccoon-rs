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

/// Trait that all type handlers must implement
/// This allows types to define their own instance and static methods
pub trait TypeHandler {
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
}
