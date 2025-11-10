use crate::runtime::Registrar;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub type ModuleLoader = Arc<dyn Fn(&mut Registrar) + Send + Sync>;

pub struct ModuleRegistry {
    registrations: HashMap<String, ModuleLoader>,

    loaded: Arc<Mutex<HashSet<String>>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            registrations: HashMap::new(),
            loaded: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn register<F>(&mut self, name: &str, loader: F)
    where
        F: Fn(&mut Registrar) + 'static + Send + Sync,
    {
        self.registrations
            .insert(name.to_string(), Arc::new(loader));
    }

    pub fn load_module(&self, name: &str, registrar: &mut Registrar) -> Result<(), String> {
        let mut loaded = self
            .loaded
            .lock()
            .map_err(|_| "Failed to acquire lock".to_string())?;

        if loaded.contains(name) {
            return Ok(());
        }

        let loader = self
            .registrations
            .get(name)
            .ok_or(format!("Module '{}' not found", name))?;

        (loader)(registrar);

        loaded.insert(name.to_string());

        Ok(())
    }

    pub fn has_module(&self, name: &str) -> bool {
        self.registrations.contains_key(name)
    }

    pub fn list_modules(&self) -> Vec<String> {
        self.registrations.keys().cloned().collect()
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded
            .lock()
            .map(|loaded| loaded.contains(name))
            .unwrap_or(false)
    }

    pub fn get_loader(&self, name: &str) -> Option<ModuleLoader> {
        self.registrations.get(name).cloned()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
