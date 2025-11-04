/// Native Rust FFI System - Clean Architecture with @native Decorators
///
/// This module provides a clean, decorator-based approach to Rust FFI:
///
/// ```raccoon
/// @native
/// fn add(a: int, b: int): int
///
/// @native
/// fn greet(name: string): string
/// ```
///
/// The @native decorator signals that the function body contains Rust code
/// that will be compiled and linked directly into the Raccoon runtime.

use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::values::{RuntimeValue, NativeFunctionValue};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ============================================================================
// TRAIT-BASED TYPE CONVERSION SYSTEM
// ============================================================================

/// Convert from Raccoon RuntimeValue to Rust types
pub trait FromRaccoon: Sized {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String>;
}

/// Convert from Rust types to Raccoon RuntimeValue
pub trait ToRaccoon {
    fn to_runtime(self) -> RuntimeValue;
}

// ============================================================================
// PRIMITIVE TYPES - FromRaccoon/ToRaccoon
// ============================================================================

impl FromRaccoon for i64 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Int(i) => Ok(i.value),
            _ => Err(format!("Expected int, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for i64 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Int(crate::runtime::values::IntValue::new(self))
    }
}

impl FromRaccoon for i32 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Int(i) => Ok(i.value as i32),
            _ => Err(format!("Expected int, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for i32 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Int(crate::runtime::values::IntValue::new(self as i64))
    }
}

impl FromRaccoon for f64 {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Float(f) => Ok(f.value),
            RuntimeValue::Int(i) => Ok(i.value as f64),
            _ => Err(format!("Expected float, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for f64 {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Float(crate::runtime::values::FloatValue::new(self))
    }
}

impl FromRaccoon for bool {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Bool(b) => Ok(b.value),
            _ => Err(format!("Expected bool, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for bool {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Bool(crate::runtime::values::BoolValue::new(self))
    }
}

impl FromRaccoon for String {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Str(s) => Ok(s.value.clone()),
            _ => Err(format!("Expected string, got {}", value.to_string())),
        }
    }
}

impl ToRaccoon for String {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Str(crate::runtime::values::StrValue::new(self))
    }
}

impl ToRaccoon for &str {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Str(crate::runtime::values::StrValue::new(self.to_string()))
    }
}

// ============================================================================
// COLLECTION TYPES
// ============================================================================

impl FromRaccoon for Vec<RuntimeValue> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::List(list) => Ok(list.elements.clone()),
            _ => Err(format!("Expected list, got {}", value.to_string())),
        }
    }
}

impl<T: FromRaccoon> FromRaccoon for Vec<T> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::List(list) => list
                .elements
                .iter()
                .map(|v| T::from_runtime(v))
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(format!("Expected list, got {}", value.to_string())),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Vec<T> {
    fn to_runtime(self) -> RuntimeValue {
        let elements: Vec<RuntimeValue> = self.into_iter().map(|v| v.to_runtime()).collect();
        RuntimeValue::List(crate::runtime::values::ListValue::new(
            elements,
            PrimitiveType::any(),
        ))
    }
}

// ============================================================================
// OPTION TYPE
// ============================================================================

impl<T: ToRaccoon> ToRaccoon for Option<T> {
    fn to_runtime(self) -> RuntimeValue {
        match self {
            Some(v) => v.to_runtime(),
            None => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
        }
    }
}

impl<T: FromRaccoon> FromRaccoon for Option<T> {
    fn from_runtime(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Null(_) => Ok(None),
            other => T::from_runtime(other).map(Some),
        }
    }
}

// ============================================================================
// RESULT TYPE
// ============================================================================

impl<T: ToRaccoon, E: ToString> ToRaccoon for Result<T, E> {
    fn to_runtime(self) -> RuntimeValue {
        match self {
            Ok(v) => v.to_runtime(),
            Err(e) => RuntimeValue::Str(crate::runtime::values::StrValue::new(e.to_string())),
        }
    }
}

// ============================================================================
// UNIT TYPE
// ============================================================================

impl ToRaccoon for () {
    fn to_runtime(self) -> RuntimeValue {
        RuntimeValue::Null(crate::runtime::values::NullValue::new())
    }
}

// ============================================================================
// NATIVE FUNCTION REGISTRY - Core Abstraction
// ============================================================================

/// Central registry for all native Rust functions
pub struct NativeRegistry {
    functions: Arc<RwLock<HashMap<String, NativeFunctionValue>>>,
}

impl NativeRegistry {
    /// Create a new native registry
    pub fn new() -> Self {
        NativeRegistry {
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a synchronous Rust function with automatic type handling
    pub fn register(
        &self,
        name: &str,
        function: crate::runtime::values::NativeFn,
        param_types: Vec<Type>,
        return_type: Type,
    ) {
        let fn_type = Type::Function(Box::new(FunctionType {
            params: param_types,
            return_type: return_type,
            is_variadic: false,
        }));

        let native_fn = NativeFunctionValue::new(function, fn_type);

        self.functions
            .write()
            .unwrap()
            .insert(name.to_string(), native_fn);
    }

    /// Get a registered function by name
    pub fn get(&self, name: &str) -> Option<NativeFunctionValue> {
        self.functions.read().unwrap().get(name).cloned()
    }

    /// List all registered function names
    pub fn list(&self) -> Vec<String> {
        self.functions
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }

    /// Export all registered functions as a HashMap
    pub fn export_all(&self) -> HashMap<String, NativeFunctionValue> {
        self.functions.read().unwrap().clone()
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// NATIVE DECORATOR PROCESSOR - Handles @native decorated functions
// ============================================================================

/// Processes @native decorated functions from the AST
pub struct NativeDecoratorProcessor;

impl NativeDecoratorProcessor {
    /// Check if a function declaration has the @native decorator
    ///
    /// This will integrate with the AST parser to identify @native decorators.
    pub fn has_native_decorator(_decorators: &[crate::ast::nodes::DecoratorDecl]) -> bool {
        // TODO: Implement when AST integration is complete
        false
    }
}

// ============================================================================
// MACRO FOR CONVENIENT REGISTRATION
// ============================================================================

/// Macro for registering native Rust functions with minimal boilerplate
///
/// # Examples
///
/// ```rust
/// let registry = NativeRegistry::new();
///
/// // Simple closure
/// register_native!(registry, "add",
///     |a: i64, b: i64| -> i64 { a + b },
///     vec![Type::Int, Type::Int] => Type::Int
/// );
///
/// // Function reference
/// register_native!(registry, "greet",
///     greet_fn,
///     vec![Type::String] => Type::String
/// );
/// ```
#[macro_export]
macro_rules! register_native {
    ($registry:expr, $name:expr, $function:expr, $params:expr => $return_type:expr) => {
        $registry.register($name, $function, $params, $return_type)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_conversion() {
        let val = RuntimeValue::Int(crate::runtime::values::IntValue::new(42));
        let result = i64::from_runtime(&val);
        assert_eq!(result.unwrap(), 42);

        let converted_back = 42i64.to_runtime();
        assert_eq!(converted_back.to_string(), "42");
    }

    #[test]
    fn test_string_conversion() {
        let val = RuntimeValue::Str(crate::runtime::values::StrValue::new("hello".to_string()));
        let result = String::from_runtime(&val);
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_registry_operations() {
        let registry = NativeRegistry::new();

        // Register a simple function
        fn test_fn_impl(_args: Vec<RuntimeValue>) -> RuntimeValue {
            RuntimeValue::Int(crate::runtime::values::IntValue::new(42))
        }

        registry.register(
            "test_fn",
            test_fn_impl,
            vec![],
            PrimitiveType::int(),
        );

        // Check it's registered
        let names = registry.list();
        assert!(names.contains(&"test_fn".to_string()));

        // Get it back
        let func = registry.get("test_fn");
        assert!(func.is_some());
    }
}
