use crate::ast::types::Type;
use crate::runtime::values::{NativeAsyncFunctionValue, NativeFunctionValue, RuntimeValue};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait NativePlugin: Send + Sync {
    fn namespace(&self) -> &str;

    fn register(&self, registry: &mut PluginRegistry);
}

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

    pub fn get_sync(&self, name: &str) -> Option<NativeFunctionValue> {
        self.sync_functions.get(name).cloned()
    }

    pub fn get_async(&self, name: &str) -> Option<NativeAsyncFunctionValue> {
        self.async_functions.get(name).cloned()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.sync_functions.contains_key(name) || self.async_functions.contains_key(name)
    }

    pub fn list_all(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.sync_functions.keys().cloned());
        names.extend(self.async_functions.keys().cloned());
        names.sort();
        names
    }

    pub fn list_namespace(&self, namespace: &str) -> Vec<String> {
        self.namespaces.get(namespace).cloned().unwrap_or_default()
    }

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

    pub fn register_plugin(&mut self, plugin: Arc<dyn NativePlugin>) {
        self.plugins.push(plugin.clone());

        let mut reg = self.registry.write().unwrap();
        plugin.register(&mut reg);
    }

    pub fn load_builtins(&mut self) {}

    pub fn registry(&self) -> Arc<RwLock<PluginRegistry>> {
        self.registry.clone()
    }

    pub fn get_function(&self, name: &str) -> Option<RuntimeValue> {
        let reg = self.registry.read().unwrap();
        reg.get_sync(name).map(|f| RuntimeValue::NativeFunction(f))
    }

    pub fn get_async_function(&self, name: &str) -> Option<RuntimeValue> {
        let reg = self.registry.read().unwrap();
        reg.get_async(name)
            .map(|f| RuntimeValue::NativeAsyncFunction(f))
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.registry.read().unwrap().list_all()
    }

    pub fn register_in_env(&self, interp: &mut crate::interpreter::Interpreter) {
        let reg = self.registry.read().unwrap();

        for (name, func) in &reg.sync_functions {
            let value = RuntimeValue::NativeFunction(func.clone());
            let _ = interp.declare_in_env(name.clone(), value);
        }

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
