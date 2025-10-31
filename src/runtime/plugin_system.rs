/// Plugin System - Unified architecture for native functions
///
/// This system eliminates boilerplate and provides automatic registration,
/// type erasure, and seamless integration with the runtime.

use crate::ast::types::Type;
use crate::runtime::values::{NativeAsyncFunctionValue, NativeFunctionValue, RuntimeValue};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Core trait for plugins that provide native functions
pub trait NativePlugin: Send + Sync {
    /// Plugin namespace (e.g., "math", "io", "string")
    fn namespace(&self) -> &str;

    /// Register all functions from this plugin
    fn register(&self, registry: &mut PluginRegistry);
}

/// Unified registry for all native functions (sync and async)
pub struct PluginRegistry {
    pub(crate) sync_functions: HashMap<String, NativeFunctionValue>,
    pub(crate) async_functions: HashMap<String, NativeAsyncFunctionValue>,
    pub(crate) namespaces: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            sync_functions: HashMap::new(),
            async_functions: HashMap::new(),
            namespaces: HashMap::new(),
        }
    }

    /// Register a synchronous native function
    pub fn register_sync(
        &mut self,
        name: impl Into<String>,
        namespace: Option<impl Into<String>>,
        _fn_type: Type,
        implementation: NativeFunctionValue,
    ) {
        let name = name.into();
        let namespace = namespace.map(|n| n.into());

        let full_name = if let Some(ref ns) = namespace {
            format!("{}.{}", ns, name)
        } else {
            name.clone()
        };

        self.sync_functions.insert(full_name, implementation);

        if let Some(ns) = namespace {
            self.namespaces
                .entry(ns)
                .or_insert_with(Vec::new)
                .push(name);
        }
    }

    /// Register an asynchronous native function
    pub fn register_async(
        &mut self,
        name: impl Into<String>,
        namespace: Option<impl Into<String>>,
        _fn_type: Type,
        implementation: NativeAsyncFunctionValue,
    ) {
        let name = name.into();
        let namespace = namespace.map(|n| n.into());

        let full_name = if let Some(ref ns) = namespace {
            format!("{}.{}", ns, name)
        } else {
            name.clone()
        };

        self.async_functions.insert(full_name, implementation);

        if let Some(ns) = namespace {
            self.namespaces
                .entry(ns)
                .or_insert_with(Vec::new)
                .push(name);
        }
    }

    /// Get a synchronous function
    pub fn get_sync(&self, name: &str) -> Option<NativeFunctionValue> {
        self.sync_functions.get(name).cloned()
    }

    /// Get an asynchronous function
    pub fn get_async(&self, name: &str) -> Option<NativeAsyncFunctionValue> {
        self.async_functions.get(name).cloned()
    }

    /// Check if a function exists (sync or async)
    pub fn exists(&self, name: &str) -> bool {
        self.sync_functions.contains_key(name) || self.async_functions.contains_key(name)
    }

    /// List all registered function names
    pub fn list_all(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.sync_functions.keys().cloned());
        names.extend(self.async_functions.keys().cloned());
        names.sort();
        names
    }

    /// List functions in a namespace
    pub fn list_namespace(&self, namespace: &str) -> Vec<String> {
        self.namespaces
            .get(namespace)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all namespaces
    pub fn list_namespaces(&self) -> Vec<String> {
        let mut namespaces: Vec<_> = self.namespaces.keys().cloned().collect();
        namespaces.sort();
        namespaces
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Centralized Plugin Manager
pub struct PluginManager {
    registry: Arc<RwLock<PluginRegistry>>,
    plugins: Vec<Arc<dyn NativePlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(PluginRegistry::new())),
            plugins: Vec::new(),
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Arc<dyn NativePlugin>) {
        self.plugins.push(plugin.clone());

        // Load plugin into registry
        let mut reg = self.registry.write().unwrap();
        plugin.register(&mut reg);
    }

    /// Load all built-in plugins
    pub fn load_builtins(&mut self) {
        // These will be created from the existing native modules
        // This is a placeholder - will be populated with actual plugins
    }

    /// Get the registry
    pub fn registry(&self) -> Arc<RwLock<PluginRegistry>> {
        self.registry.clone()
    }

    /// Get a function as RuntimeValue (sync)
    pub fn get_function(&self, name: &str) -> Option<RuntimeValue> {
        let reg = self.registry.read().unwrap();
        reg.get_sync(name)
            .map(|f| RuntimeValue::NativeFunction(f))
    }

    /// Get a function as RuntimeValue (async)
    pub fn get_async_function(&self, name: &str) -> Option<RuntimeValue> {
        let reg = self.registry.read().unwrap();
        reg.get_async(name)
            .map(|f| RuntimeValue::NativeAsyncFunction(f))
    }

    /// List all functions
    pub fn list_functions(&self) -> Vec<String> {
        self.registry.read().unwrap().list_all()
    }

    /// Register all plugins in the interpreter
    pub fn register_in_env(&self, interp: &mut crate::interpreter::Interpreter) {
        let reg = self.registry.read().unwrap();

        // Register sync functions
        for (name, func) in &reg.sync_functions {
            let value = RuntimeValue::NativeFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }

        // Register async functions
        for (name, func) in &reg.async_functions {
            let value = RuntimeValue::NativeAsyncFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct TestPlugin;

    impl NativePlugin for TestPlugin {
        fn namespace(&self) -> &str {
            "test"
        }

        fn register(&self, _registry: &mut PluginRegistry) {
            // Empty for test
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = PluginRegistry::new();
        assert!(registry.sync_functions.is_empty());
        assert!(registry.async_functions.is_empty());
    }

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.list_functions().is_empty());
    }
}
