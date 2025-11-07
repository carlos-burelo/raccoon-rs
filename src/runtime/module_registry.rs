use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, Arc};
use crate::runtime::Registrar;

/// Type for module loader functions
pub type ModuleLoader = Arc<dyn Fn(&mut Registrar) + Send + Sync>;

/// Registry for all available modules
/// Handles lazy-loading of native modules on demand
pub struct ModuleRegistry {
    /// Maps module name to its loader function
    registrations: HashMap<String, ModuleLoader>,
    /// Tracks which modules have been loaded
    loaded: Arc<Mutex<HashSet<String>>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            registrations: HashMap::new(),
            loaded: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Register a module (metadata only, no execution)
    pub fn register<F>(&mut self, name: &str, loader: F)
    where
        F: Fn(&mut Registrar) + 'static + Send + Sync,
    {
        self.registrations
            .insert(name.to_string(), Arc::new(loader));
    }

    /// Load a module (lazy) - only execute if not already loaded
    pub fn load_module(&self, name: &str, registrar: &mut Registrar) -> Result<(), String> {
        let mut loaded = self
            .loaded
            .lock()
            .map_err(|_| "Failed to acquire lock".to_string())?;

        // Already loaded?
        if loaded.contains(name) {
            return Ok(());
        }

        // Get the loader function
        let loader = self
            .registrations
            .get(name)
            .ok_or(format!("Module '{}' not found", name))?;

        // Execute loader (first time only)
        (loader)(registrar);

        // Mark as loaded
        loaded.insert(name.to_string());

        Ok(())
    }

    /// Check if module is available (without loading)
    pub fn has_module(&self, name: &str) -> bool {
        self.registrations.contains_key(name)
    }

    /// Get list of available modules
    pub fn list_modules(&self) -> Vec<String> {
        self.registrations.keys().cloned().collect()
    }

    /// Check if module is already loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded
            .lock()
            .map(|loaded| loaded.contains(name))
            .unwrap_or(false)
    }

    /// Get the loader function for a module
    pub fn get_loader(&self, name: &str) -> Option<ModuleLoader> {
        self.registrations.get(name).cloned()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
