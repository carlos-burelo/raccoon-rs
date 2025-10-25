use std::time::Instant;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::runtime::RuntimeValue;

/// Metadatos de decoradores aplicados a una función
#[derive(Debug, Clone)]
pub struct DecoratorMetadata {
    pub is_ffi: bool,
    pub namespace: Option<String>,
    pub validate: bool,
    pub cache_ttl_ms: Option<i64>,
    pub is_deprecated: Option<String>,
    pub is_pure: bool,
    pub should_inline: bool,
}

impl Default for DecoratorMetadata {
    fn default() -> Self {
        Self {
            is_ffi: false,
            namespace: None,
            validate: false,
            cache_ttl_ms: None,
            is_deprecated: None,
            is_pure: false,
            should_inline: false,
        }
    }
}

/// Sistema de cache para funciones con decorador @cache
pub struct FunctionCache {
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

struct CacheEntry {
    value: RuntimeValue,
    created_at: Instant,
    ttl_ms: i64,
}

impl FunctionCache {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<RuntimeValue> {
        let mut entries = self.entries.write().unwrap();

        if let Some(entry) = entries.get(key) {
            let elapsed = entry.created_at.elapsed().as_millis() as i64;
            if elapsed < entry.ttl_ms {
                return Some(entry.value.clone());
            } else {
                entries.remove(key);
            }
        }

        None
    }

    pub fn set(&self, key: String, value: RuntimeValue, ttl_ms: i64) {
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            ttl_ms,
        };
        self.entries.write().unwrap().insert(key, entry);
    }

    pub fn clear(&self) {
        self.entries.write().unwrap().clear();
    }
}

impl Default for FunctionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Aplicador de decoradores - aplica efectos de decoradores
pub struct DecoratorApplier {
    cache: FunctionCache,
}

impl DecoratorApplier {
    pub fn new() -> Self {
        Self {
            cache: FunctionCache::new(),
        }
    }

    /// Aplica decorador @cache
    pub fn apply_cache(
        &self,
        cache_key: &str,
        ttl_ms: i64,
        execute: impl Fn() -> RuntimeValue,
    ) -> RuntimeValue {
        // Intentar obtener del cache
        if let Some(cached) = self.cache.get(cache_key) {
            return cached;
        }

        // Ejecutar función
        let result = execute();

        // Guardar en cache
        self.cache.set(cache_key.to_string(), result.clone(), ttl_ms);

        result
    }

    /// Aplica decorador @deprecated (solo imprime warning)
    pub fn apply_deprecated(name: &str, message: Option<&str>) {
        let msg = message.unwrap_or("This function is deprecated");
        eprintln!("⚠️  Warning: '{}' is deprecated. {}", name, msg);
    }

    /// Valida que función sea pura (no hay side effects)
    pub fn validate_pure(_name: &str) {
        // En runtime, es difícil validar que NO hay side effects
        // Esto es más un hint para el compilador
    }

    /// Sugerencia para inline
    pub fn hint_inline(_name: &str) {
        // Esto es un hint para optimización del compilador/interpreter
        // En runtime interpretado, no tiene efecto real
    }
}

impl Default for DecoratorApplier {
    fn default() -> Self {
        Self::new()
    }
}
