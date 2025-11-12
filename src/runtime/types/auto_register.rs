use super::TypeHandler;

pub trait AutoRegister: Send + Sync {
    fn create_handler() -> Box<dyn TypeHandler>;
}

pub struct TypeRegistryEntry {
    pub constructor: fn() -> Box<dyn TypeHandler>,
}

impl TypeRegistryEntry {
    pub const fn new(constructor: fn() -> Box<dyn TypeHandler>) -> Self {
        Self { constructor }
    }
}

inventory::collect!(TypeRegistryEntry);

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

#[macro_export]
macro_rules! register_type_with {
    ($type:ty, $constructor:expr) => {
        inventory::submit! {
            $crate::runtime::types::auto_register::TypeRegistryEntry::new($constructor)
        }
    };
}

pub fn get_registered_types() -> Vec<Box<dyn TypeHandler>> {
    inventory::iter::<TypeRegistryEntry>
        .into_iter()
        .map(|entry| (entry.constructor)())
        .collect()
}
