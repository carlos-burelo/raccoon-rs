//! Lazy-loading primitive registry
//!
//! This module provides a registry system for primitives with lazy loading.
//! Primitives are only loaded when their context is requested.

use super::contexts::PrimitiveContext;
use crate::runtime::Registrar;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

/// A lazy-loading registry for primitives organized by context
pub struct LazyPrimitiveRegistry {
    /// Track which contexts have been loaded
    loaded_contexts: Arc<RwLock<HashMap<PrimitiveContext, bool>>>,
    /// The registrar for storing primitives
    registrar: Arc<Mutex<Registrar>>,
}

impl LazyPrimitiveRegistry {
    /// Create a new lazy primitive registry
    pub fn new(registrar: Arc<Mutex<Registrar>>) -> Self {
        Self {
            loaded_contexts: Arc::new(RwLock::new(HashMap::new())),
            registrar,
        }
    }

    /// Check if a context has been loaded
    pub fn is_loaded(&self, context: PrimitiveContext) -> bool {
        self.loaded_contexts
            .read()
            .unwrap()
            .get(&context)
            .copied()
            .unwrap_or(false)
    }

    /// Load a specific context
    pub fn load_context(&self, context: PrimitiveContext) {
        // Check if already loaded
        if self.is_loaded(context) {
            return;
        }

        // Load the context
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
            PrimitiveContext::Builtins => {
                // Builtins are always loaded
            }
        }

        // Mark as loaded
        self.loaded_contexts
            .write()
            .unwrap()
            .insert(context, true);
    }

    /// Load all contexts
    pub fn load_all(&self) {
        for context in PrimitiveContext::all() {
            self.load_context(*context);
        }
    }

    /// Get the list of loaded contexts
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
