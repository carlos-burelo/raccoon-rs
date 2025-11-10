use super::{CallbackExecutor, TypeHandler};
// New modular imports - Primitives
use super::primitives::{BigIntType, BoolType, DecimalType, IntType, StrType};
use super::primitives::{Float32Type, CharType, NullType, UnitType};
use super::primitives::floats::Float64Type; // Import Float64Type directly
// Collections
use super::collections::{ListType, MapType, SetType, TupleType, RangeType, OptionalType};
// Objects
use super::objects::{ClassType, FunctionType, InterfaceType, ObjectType};
// Async/Control
use super::async_control::{FutureType, ResultType, StreamType, EitherType};
// Special
use super::special::{EnumType, VoidType, NeverType, SymbolType, UnionType, IntersectionType, NullableType, ReadonlyType};
// Type reflection
use super::type_type::TypeType;
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use std::collections::HashMap;

pub struct TypeRegistry {
    handlers: HashMap<String, Box<dyn TypeHandler>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };

        // Primitive types - Integers (using unified NumericHandler)
        registry.register(Box::new(IntType));
        registry.register(Box::new(super::primitives::numeric_trait::I8Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::I16Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::I32Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::I64Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::U8Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::U16Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::U32Handler::new()));
        registry.register(Box::new(super::primitives::numeric_trait::U64Handler::new()));
        registry.register(Box::new(BigIntType));

        // Primitive types - Floats
        registry.register(Box::new(Float64Type)); // Default float
        registry.register(Box::new(Float32Type));
        registry.register(Box::new(DecimalType));

        // Primitive types - Other
        registry.register(Box::new(StrType));
        registry.register(Box::new(BoolType));
        registry.register(Box::new(CharType));
        registry.register(Box::new(NullType));
        registry.register(Box::new(UnitType));

        // Collection types
        registry.register(Box::new(ListType));
        registry.register(Box::new(MapType));
        registry.register(Box::new(SetType));
        registry.register(Box::new(TupleType));
        registry.register(Box::new(RangeType));
        registry.register(Box::new(OptionalType));

        // Object types
        registry.register(Box::new(ObjectType));
        registry.register(Box::new(ClassType));
        registry.register(Box::new(FunctionType));
        registry.register(Box::new(InterfaceType));

        // Async/Control types
        registry.register(Box::new(FutureType));
        registry.register(Box::new(ResultType));
        registry.register(Box::new(StreamType));
        registry.register(Box::new(EitherType));

        // Special types
        registry.register(Box::new(EnumType));
        registry.register(Box::new(VoidType));
        registry.register(Box::new(NeverType));
        registry.register(Box::new(SymbolType));
        registry.register(Box::new(UnionType));
        registry.register(Box::new(IntersectionType));
        registry.register(Box::new(NullableType));
        registry.register(Box::new(ReadonlyType));

        // Type reflection
        registry.register(Box::new(TypeType::new()));

        registry
    }

    pub fn register(&mut self, handler: Box<dyn TypeHandler>) {
        self.handlers
            .insert(handler.type_name().to_string(), handler);
    }

    pub fn get_handler(&self, type_name: &str) -> Option<&dyn TypeHandler> {
        self.handlers.get(type_name).map(|h| h.as_ref())
    }

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

    pub fn has_instance_method(&self, type_name: &str, method: &str) -> bool {
        self.get_handler(type_name)
            .map(|h| h.has_instance_method(method))
            .unwrap_or(false)
    }

    pub fn has_static_method(&self, type_name: &str, method: &str) -> bool {
        self.get_handler(type_name)
            .map(|h| h.has_static_method(method))
            .unwrap_or(false)
    }

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
