use crate::ast::types::Type;
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Información sobre una función registrada en FFI
#[derive(Debug, Clone)]
pub struct FFIFunctionInfo {
    pub name: String,
    pub namespace: Option<String>,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub is_async: bool,
}

/// Función nativa registrada dinámicamente
pub type FFIFunction = Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>;

/// Función nativa async registrada dinámicamente
pub type FFIAsyncFunction =
    Arc<dyn Fn(Vec<RuntimeValue>) -> std::pin::Pin<Box<dyn std::future::Future<Output = RuntimeValue> + Send>> + Send + Sync>;

/// Registro central de funciones FFI (invocables dinámicamente)
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

    /// Registra una función síncrona
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

        // Registrar información
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

        // Registrar implementación
        {
            let mut impls = self.implementations.write().unwrap();
            impls.insert(full_name.clone(), implementation);
        }

        // Registrar en namespace si aplica
        if let Some(ns) = namespace {
            let mut namespaces = self.namespaces.write().unwrap();
            namespaces.entry(ns).or_insert_with(Vec::new).push(name);
        }

        Ok(())
    }

    /// Registra una función asincrónica
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

        // Registrar información
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

        // Registrar implementación
        {
            let mut impls = self.async_implementations.write().unwrap();
            impls.insert(full_name.clone(), implementation);
        }

        // Registrar en namespace si aplica
        if let Some(ns) = namespace {
            let mut namespaces = self.namespaces.write().unwrap();
            namespaces.entry(ns).or_insert_with(Vec::new).push(name);
        }

        Ok(())
    }

    /// Llama una función síncrona registrada
    pub fn call_function(&self, name: &str, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RaccoonError> {
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

    /// Llama una función asincrónica registrada
    pub async fn call_async_function(
        &self,
        name: &str,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let implementations = self.async_implementations.read().unwrap();
        let func = implementations.get(name).ok_or_else(|| {
            RaccoonError::new(
                format!("FFI async function '{}' not found", name),
                (0, 0),
                None::<String>,
            )
        })?;

        Ok(func(args).await)
    }

    /// Obtiene información de una función registrada
    pub fn get_function_info(&self, name: &str) -> Option<FFIFunctionInfo> {
        self.functions
            .read()
            .unwrap()
            .get(name)
            .cloned()
            .or_else(|| self.async_functions.read().unwrap().get(name).cloned())
    }

    /// Lista todas las funciones registradas
    pub fn list_functions(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.functions.read().unwrap().keys().cloned());
        names.extend(self.async_functions.read().unwrap().keys().cloned());
        names.sort();
        names
    }

    /// Lista funciones en un namespace específico
    pub fn list_namespace(&self, namespace: &str) -> Vec<String> {
        self.namespaces
            .read()
            .unwrap()
            .get(namespace)
            .cloned()
            .unwrap_or_default()
    }

    /// Verifica si una función está registrada
    pub fn exists(&self, name: &str) -> bool {
        self.functions.read().unwrap().contains_key(name)
            || self.async_functions.read().unwrap().contains_key(name)
    }

    /// Limpia todas las funciones registradas (para testing)
    pub fn clear(&self) {
        self.functions.write().unwrap().clear();
        self.async_functions.write().unwrap().clear();
        self.implementations.write().unwrap().clear();
        self.async_implementations.write().unwrap().clear();
        self.namespaces.write().unwrap().clear();
    }

    /// Registra una función de Raccoon (no nativa) para ser invocable dinámicamente via FFI
    /// Las funciones de Raccoon se almacenan solo con metadata, sin implementación
    pub fn register_raccoon_function(
        &self,
        name: String,
        _function: crate::runtime::FunctionValue,
        params: Vec<(String, Type)>,
        return_type: Type,
    ) {
        // Registrar metadata de la función de Raccoon
        {
            let mut funcs = self.functions.write().unwrap();
            funcs.insert(
                name.clone(),
                FFIFunctionInfo {
                    name: name.clone(),
                    namespace: None,
                    params,
                    return_type,
                    is_async: false, // Se determinará por el tipo de retorno si es Future
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_registry_registration() {
        let registry = FFIRegistry::new();

        registry
            .register_function(
                "add".to_string(),
                None,
                vec![
                    ("a".to_string(), Type::Primitive(crate::ast::types::PrimitiveType::int())),
                    ("b".to_string(), Type::Primitive(crate::ast::types::PrimitiveType::int())),
                ],
                Type::Primitive(crate::ast::types::PrimitiveType::int()),
                Arc::new(|args| {
                    if args.len() == 2 {
                        if let (RuntimeValue::Int(a), RuntimeValue::Int(b)) = (&args[0], &args[1]) {
                            return RuntimeValue::Int(crate::runtime::IntValue::new(a.value + b.value));
                        }
                    }
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                }),
            )
            .unwrap();

        assert!(registry.exists("add"));
    }

    #[test]
    fn test_ffi_registry_call() {
        let registry = FFIRegistry::new();

        registry
            .register_function(
                "multiply".to_string(),
                None,
                vec![
                    ("a".to_string(), Type::Primitive(crate::ast::types::PrimitiveType::int())),
                    ("b".to_string(), Type::Primitive(crate::ast::types::PrimitiveType::int())),
                ],
                Type::Primitive(crate::ast::types::PrimitiveType::int()),
                Arc::new(|args| {
                    if args.len() == 2 {
                        if let (RuntimeValue::Int(a), RuntimeValue::Int(b)) = (&args[0], &args[1]) {
                            return RuntimeValue::Int(crate::runtime::IntValue::new(a.value * b.value));
                        }
                    }
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                }),
            )
            .unwrap();

        let result = registry
            .call_function(
                "multiply",
                vec![
                    RuntimeValue::Int(crate::runtime::IntValue::new(5)),
                    RuntimeValue::Int(crate::runtime::IntValue::new(3)),
                ],
            )
            .unwrap();

        if let RuntimeValue::Int(i) = result {
            assert_eq!(i.value, 15);
        } else {
            panic!("Expected Int value");
        }
    }
}
