/// Auto-registration system for types
/// Uses the `inventory` crate to collect types at compile time

use super::TypeHandler;

/// Trait for types that can be automatically registered
pub trait AutoRegister: Send + Sync {
    fn create_handler() -> Box<dyn TypeHandler>;
}

/// Registry entry for automatic type registration
pub struct TypeRegistryEntry {
    pub constructor: fn() -> Box<dyn TypeHandler>,
}

impl TypeRegistryEntry {
    pub const fn new(constructor: fn() -> Box<dyn TypeHandler>) -> Self {
        Self { constructor }
    }
}

// Use inventory to collect all types at compile time
inventory::collect!(TypeRegistryEntry);

/// Macro to register a type automatically
///
/// # Example
/// ```ignore
/// #[register_type]
/// pub struct MyType;
///
/// impl TypeHandler for MyType {
///     // ... implementation
/// }
/// ```
#[macro_export]
macro_rules! register_type {
    ($type:ty) => {
        inventory::submit! {
            $crate::runtime::types::auto_register::TypeRegistryEntry::new(|| {
                Box::new(<$type>::default())
            })
        }
    };
}

/// Helper macro to register a type with a custom constructor
#[macro_export]
macro_rules! register_type_with {
    ($type:ty, $constructor:expr) => {
        inventory::submit! {
            $crate::runtime::types::auto_register::TypeRegistryEntry::new($constructor)
        }
    };
}

/// Get all registered type handlers
pub fn get_registered_types() -> Vec<Box<dyn TypeHandler>> {
    inventory::iter::<TypeRegistryEntry>
        .into_iter()
        .map(|entry| (entry.constructor)())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_register_infrastructure() {
        // This test just ensures the infrastructure compiles
        // Actual registration is tested in integration tests
        let _types = get_registered_types();
        // Infrastructure is working if this compiles and runs
    }
}
