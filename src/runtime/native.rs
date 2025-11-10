use crate::ast::types::{FunctionType, Type};
use crate::runtime::values::NativeFunctionValue;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Registry for native functions
/// This provides a centralized way to register and lookup native functions
/// with proper type information

pub struct NativeRegistry {
    functions: Arc<RwLock<HashMap<String, NativeFunctionValue>>>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        NativeRegistry {
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

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

    pub fn get(&self, name: &str) -> Option<NativeFunctionValue> {
        self.functions.read().unwrap().get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.functions.read().unwrap().keys().cloned().collect()
    }

    pub fn export_all(&self) -> HashMap<String, NativeFunctionValue> {
        self.functions.read().unwrap().clone()
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Processor for native decorators
/// Used to check if functions have native decorators that require special handling
pub struct NativeDecoratorProcessor;

impl NativeDecoratorProcessor {
    pub fn has_native_decorator(_decorators: &[crate::ast::nodes::DecoratorDecl]) -> bool {
        false
    }
}
