/// FFI Registry - DEPRECATED
///
/// This module is deprecated and will be refactored with the new plugin system.
/// The code duplication in register_function() and register_async_function() (lines 45-125)
/// is eliminated by the PluginRegistry design.
///
/// See: src/runtime/plugin_system.rs and src/runtime/builtin_plugins.rs

use crate::ast::types::Type;
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct FFIFunctionInfo {
    pub name: String,
    pub namespace: Option<String>,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub is_async: bool,
}

pub type FFIFunction = Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>;

pub type FFIAsyncFunction = Arc<
    dyn Fn(
            Vec<RuntimeValue>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = RuntimeValue> + Send>>
        + Send
        + Sync,
>;

pub struct FFIRegistry {
    functions: Arc<RwLock<HashMap<String, FFIFunctionInfo>>>,
    async_functions: Arc<RwLock<HashMap<String, FFIFunctionInfo>>>,
    implementations: Arc<RwLock<HashMap<String, FFIFunction>>>,
    async_implementations: Arc<RwLock<HashMap<String, FFIAsyncFunction>>>,
    namespaces: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl FFIRegistry {
    pub fn new() -> Self {
        Self {
            functions: Arc::new(RwLock::new(HashMap::new())),
            async_functions: Arc::new(RwLock::new(HashMap::new())),
            implementations: Arc::new(RwLock::new(HashMap::new())),
            async_implementations: Arc::new(RwLock::new(HashMap::new())),
            namespaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_function(
        &self,
        name: String,
        namespace: Option<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
        implementation: FFIFunction,
    ) -> Result<(), RaccoonError> {
        let full_name = if let Some(ref ns) = namespace {
            format!("{}.{}", ns, name)
        } else {
            name.clone()
        };

        {
            let mut funcs = self.functions.write().unwrap();
            funcs.insert(
                full_name.clone(),
                FFIFunctionInfo {
                    name: name.clone(),
                    namespace: namespace.clone(),
                    params,
                    return_type,
                    is_async: false,
                },
            );
        }

        {
            let mut impls = self.implementations.write().unwrap();
            impls.insert(full_name.clone(), implementation);
        }

        if let Some(ns) = namespace {
            let mut namespaces = self.namespaces.write().unwrap();
            namespaces.entry(ns).or_insert_with(Vec::new).push(name);
        }

        Ok(())
    }

    pub fn register_async_function(
        &self,
        name: String,
        namespace: Option<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
        implementation: FFIAsyncFunction,
    ) -> Result<(), RaccoonError> {
        let full_name = if let Some(ref ns) = namespace {
            format!("{}.{}", ns, name)
        } else {
            name.clone()
        };

        {
            let mut funcs = self.async_functions.write().unwrap();
            funcs.insert(
                full_name.clone(),
                FFIFunctionInfo {
                    name: name.clone(),
                    namespace: namespace.clone(),
                    params,
                    return_type,
                    is_async: true,
                },
            );
        }

        {
            let mut impls = self.async_implementations.write().unwrap();
            impls.insert(full_name.clone(), implementation);
        }

        if let Some(ns) = namespace {
            let mut namespaces = self.namespaces.write().unwrap();
            namespaces.entry(ns).or_insert_with(Vec::new).push(name);
        }

        Ok(())
    }

    pub fn call_function(
        &self,
        name: &str,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let implementations = self.implementations.read().unwrap();
        let func = implementations.get(name).ok_or_else(|| {
            RaccoonError::new(
                format!("FFI function '{}' not found", name),
                (0, 0),
                None::<String>,
            )
        })?;

        Ok(func(args))
    }

    pub async fn call_async_function(
        &self,
        name: &str,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let implementations = self.async_implementations.read().unwrap();
        let func = implementations.get(name).ok_or_else(|| {
            RaccoonError::new(
                format!("FFI async fn '{}' not found", name),
                (0, 0),
                None::<String>,
            )
        })?;

        Ok(func(args).await)
    }

    pub fn get_function_info(&self, name: &str) -> Option<FFIFunctionInfo> {
        self.functions
            .read()
            .unwrap()
            .get(name)
            .cloned()
            .or_else(|| self.async_functions.read().unwrap().get(name).cloned())
    }

    pub fn list_functions(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.functions.read().unwrap().keys().cloned());
        names.extend(self.async_functions.read().unwrap().keys().cloned());
        names.sort();
        names
    }

    pub fn list_namespace(&self, namespace: &str) -> Vec<String> {
        self.namespaces
            .read()
            .unwrap()
            .get(namespace)
            .cloned()
            .unwrap_or_default()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.functions.read().unwrap().contains_key(name)
            || self.async_functions.read().unwrap().contains_key(name)
    }

    pub fn clear(&self) {
        self.functions.write().unwrap().clear();
        self.async_functions.write().unwrap().clear();
        self.implementations.write().unwrap().clear();
        self.async_implementations.write().unwrap().clear();
        self.namespaces.write().unwrap().clear();
    }

    pub fn register_raccoon_function(
        &self,
        name: String,
        _function: crate::runtime::FunctionValue,
        params: Vec<(String, Type)>,
        return_type: Type,
    ) {
        {
            let mut funcs = self.functions.write().unwrap();
            funcs.insert(
                name.clone(),
                FFIFunctionInfo {
                    name: name.clone(),
                    namespace: None,
                    params,
                    return_type,
                    is_async: false,
                },
            );
        }
    }
}

impl Default for FFIRegistry {
    fn default() -> Self {
        Self::new()
    }
}
