use super::contexts::PrimitiveContext;
use crate::runtime::Registrar;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct LazyPrimitiveRegistry {
    loaded_contexts: Arc<RwLock<HashMap<PrimitiveContext, bool>>>,

    registrar: Arc<Mutex<Registrar>>,
}

impl LazyPrimitiveRegistry {
    pub fn new(registrar: Arc<Mutex<Registrar>>) -> Self {
        Self {
            loaded_contexts: Arc::new(RwLock::new(HashMap::new())),
            registrar,
        }
    }

    pub fn is_loaded(&self, context: PrimitiveContext) -> bool {
        self.loaded_contexts
            .read()
            .unwrap()
            .get(&context)
            .copied()
            .unwrap_or(false)
    }

    pub fn load_context(&self, context: PrimitiveContext) {
        if self.is_loaded(context) {
            return;
        }

        let mut reg = self.registrar.lock().unwrap();
        match context {
            PrimitiveContext::Math => {
                super::math::register_math_primitives(&mut reg);
            }
            PrimitiveContext::String => {
                super::string::register_string_primitives(&mut reg);
            }
            PrimitiveContext::Array => {
                super::array::register_array_primitives(&mut reg);
            }
            PrimitiveContext::IO => {
                super::io::register_io_primitives(&mut reg);
            }
            PrimitiveContext::HTTP => {
                super::http::register_http_primitives(&mut reg);
            }
            PrimitiveContext::Time => {
                super::time::register_time_primitives(&mut reg);
            }
            PrimitiveContext::JSON => {
                super::json::register_json_primitives(&mut reg);
            }
            PrimitiveContext::System => {
                super::system::register_system_primitives(&mut reg);
            }
            PrimitiveContext::Builtins => {}
        }

        self.loaded_contexts.write().unwrap().insert(context, true);
    }

    pub fn load_all(&self) {
        for context in PrimitiveContext::all() {
            self.load_context(*context);
        }
    }

    pub fn loaded_contexts_list(&self) -> Vec<PrimitiveContext> {
        self.loaded_contexts
            .read()
            .unwrap()
            .iter()
            .filter_map(|(ctx, &loaded)| if loaded { Some(*ctx) } else { None })
            .collect()
    }
}

impl Default for LazyPrimitiveRegistry {
    fn default() -> Self {
        Self::new(Arc::new(Mutex::new(Registrar::new())))
    }
}
