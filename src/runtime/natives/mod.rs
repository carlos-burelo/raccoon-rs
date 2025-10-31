pub mod array;
pub mod ffi;
pub mod http;
/// Native function modules (LEGACY)
///
/// NOTE: This module is deprecated in favor of the plugin-based system.
/// See src/runtime/plugin_system.rs and src/runtime/native_bridge_v2.rs
///
/// This module organizes native functions by category. Each submodule provides
/// a register() function for the legacy registration system.
pub mod io;
pub mod json;
pub mod math;
pub mod output;
pub mod random;
pub mod string;
pub mod time;

use crate::runtime::values::NativeFunctionValue;
use std::collections::HashMap;

/// Helper struct to accumulate all native functions
pub struct NativeRegistry {
    pub functions: HashMap<String, NativeFunctionValue>,
    pub async_functions: HashMap<String, crate::runtime::values::NativeAsyncFunctionValue>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            async_functions: HashMap::new(),
        }
    }

    /// Register all native functions from all modules
    ///
    /// NOTE: This is the legacy system. New code should use the plugin system instead.
    /// See src/runtime/plugin_system.rs and src/runtime/builtin_plugins.rs
    pub fn register_all(&mut self) {
        output::register(&mut self.functions);
        time::register(&mut self.functions);
        random::register(&mut self.functions);
        json::register(&mut self.functions);
        string::register(&mut self.functions);
        array::register(&mut self.functions);
        math::register(&mut self.functions);
        ffi::register(&mut self.functions);
        http::register_async(&mut self.async_functions);
    }
}
