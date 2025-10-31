/// NativeBridge V2 - Unified plugin-based native function management
///
/// This is the new unified interface that will replace the old NativeBridge.
/// It uses the PluginManager internally for cleaner, more maintainable code.
use crate::runtime::plugin_system::PluginManager;
use crate::runtime::values::RuntimeValue;

pub struct NativeBridgeV2 {
    plugin_manager: PluginManager,
}

impl NativeBridgeV2 {
    pub fn new() -> Self {
        let manager = PluginManager::new();

        // Load all built-in plugins
        crate::runtime::builtin_plugins::load_builtin_plugins(
            &mut manager.registry().write().unwrap(),
        );

        Self {
            plugin_manager: manager,
        }
    }

    /// Get a synchronous function by name
    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        self.plugin_manager.get_function(name)
    }

    /// Get an asynchronous function by name
    pub fn get_async(&self, name: &str) -> Option<RuntimeValue> {
        self.plugin_manager.get_async_function(name)
    }

    /// Register all functions in the interpreter
    pub fn register_all_in_env(&self, interp: &mut crate::interpreter::Interpreter) {
        self.plugin_manager.register_in_env(interp);
    }

    /// Get all registered function names (sync)
    pub fn function_names(&self) -> Vec<String> {
        let registry = self.plugin_manager.registry();
        let reg = registry.read().unwrap();
        let mut names: Vec<_> = reg.sync_functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get all registered async function names
    pub fn async_function_names(&self) -> Vec<String> {
        let registry = self.plugin_manager.registry();
        let reg = registry.read().unwrap();
        let mut names: Vec<_> = reg.async_functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// List all functions (sync + async)
    pub fn all_function_names(&self) -> Vec<String> {
        self.plugin_manager.list_functions()
    }

    /// Get the plugin manager reference
    pub fn plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
    }
}

impl Default for NativeBridgeV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_bridge_creation() {
        let bridge = NativeBridgeV2::new();
        assert!(!bridge.all_function_names().is_empty());
    }

    #[test]
    fn test_get_print_function() {
        let bridge = NativeBridgeV2::new();
        assert!(bridge.get("print").is_some());
    }
}
