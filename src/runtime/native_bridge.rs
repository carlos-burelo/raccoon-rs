/// NativeBridge - Orchestrator for native functions
///
/// This module is now modular: each category of native functions lives in
/// src/runtime/natives/*.rs files. This keeps native_bridge.rs small and
/// maintains a clean architecture.

use crate::runtime::natives::NativeRegistry;
use crate::runtime::values::{NativeAsyncFunctionValue, NativeFunctionValue, RuntimeValue};
use std::collections::HashMap;

pub struct NativeBridge {
    functions: HashMap<String, NativeFunctionValue>,
    async_functions: HashMap<String, NativeAsyncFunctionValue>,
}

impl NativeBridge {
    pub fn new() -> Self {
        let mut registry = NativeRegistry::new();
        registry.register_all();

        Self {
            functions: registry.functions,
            async_functions: registry.async_functions,
        }
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        self.functions
            .get(name)
            .map(|f| RuntimeValue::NativeFunction(f.clone()))
    }

    pub fn get_async(&self, name: &str) -> Option<RuntimeValue> {
        self.async_functions
            .get(name)
            .map(|f| RuntimeValue::NativeAsyncFunction(f.clone()))
    }

    pub fn register_all_in_env(&self, interp: &mut crate::interpreter::Interpreter) {
        // Register all synchronous native functions
        for (name, func) in &self.functions {
            let value = RuntimeValue::NativeFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }

        // Register all asynchronous native functions
        for (name, func) in &self.async_functions {
            let value = RuntimeValue::NativeAsyncFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }
    }

    /// Get all registered function names (sync)
    pub fn function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// Get all registered async function names
    pub fn async_function_names(&self) -> Vec<String> {
        self.async_functions.keys().cloned().collect()
    }
}

impl Default for NativeBridge {
    fn default() -> Self {
        Self::new()
    }
}
