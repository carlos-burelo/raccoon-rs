use crate::runtime::RuntimeValue;
use std::collections::HashMap;
use std::sync::Arc;

pub type SyncHandler = Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>;

#[derive(Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub namespace: Option<String>,
    pub handler: SyncHandler,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

pub struct Registrar {
    pub functions: HashMap<String, FunctionSignature>,
    pub constants: HashMap<String, RuntimeValue>,
}

impl Registrar {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn register_fn<F>(
        &mut self,
        name: impl Into<String>,
        namespace: Option<&str>,
        handler: F,
        min_args: usize,
        max_args: Option<usize>,
    ) where
        F: Fn(Vec<RuntimeValue>) -> RuntimeValue + 'static + Send + Sync,
    {
        let name = name.into();
        let full_name = match namespace {
            Some(ns) => format!("{}.{}", ns, name),
            None => name.clone(),
        };

        self.functions.insert(
            full_name,
            FunctionSignature {
                name,
                namespace: namespace.map(|s| s.to_string()),
                handler: Arc::new(handler),
                min_args,
                max_args,
            },
        );
    }

    pub fn register_const(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.constants.insert(name.into(), value);
    }

    pub fn get_function(&self, full_name: &str) -> Option<SyncHandler> {
        self.functions.get(full_name).map(|sig| sig.handler.clone())
    }

    pub fn get_function_signature(&self, full_name: &str) -> Option<FunctionSignature> {
        self.functions.get(full_name).cloned()
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Self::new()
    }
}
