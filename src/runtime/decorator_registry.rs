use crate::ast::nodes::DecoratorDecl;
use crate::error::RaccoonError;
use std::collections::HashMap;

/// Define la visibilidad de un decorador
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoratorVisibility {
    /// Solo puede usarse en stdlib
    Internal,
    /// Usuarios pueden usarlo
    Public,
}

/// Especificación de qué es un decorador
#[derive(Debug, Clone)]
pub struct DecoratorSpec {
    pub name: String,
    pub visibility: DecoratorVisibility,
    pub description: String,
    pub allowed_on: Vec<DecoratorTarget>,
}

/// Dónde puede aplicarse un decorador
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoratorTarget {
    Function,
    AsyncFunction,
    Class,
    ClassMethod,
    ClassProperty,
}

/// Registro central de decoradores
pub struct DecoratorRegistry {
    decorators: HashMap<String, DecoratorSpec>,
}

impl DecoratorRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            decorators: HashMap::new(),
        };
        registry.register_all_decorators();
        registry
    }

    fn register_all_decorators(&mut self) {
        // DECORADORES INTERNOS (Prefijo _)

        // @_ffi() - Registra función en FFI Registry
        self.register_decorator(DecoratorSpec {
            name: "_ffi".to_string(),
            visibility: DecoratorVisibility::Internal,
            description: "Registers function in FFI Registry for dynamic invocation".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @_register(namespace) - Registra en namespace
        self.register_decorator(DecoratorSpec {
            name: "_register".to_string(),
            visibility: DecoratorVisibility::Internal,
            description: "Registers function in a specific namespace".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @_validate() - Validación automática de tipos
        self.register_decorator(DecoratorSpec {
            name: "_validate".to_string(),
            visibility: DecoratorVisibility::Internal,
            description: "Enables automatic type validation for parameters and return value"
                .to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // DECORADORES PÚBLICOS (Users pueden usarlos)

        // @cache(ttl_ms) - Cachea resultados
        self.register_decorator(DecoratorSpec {
            name: "cache".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Caches function results for specified time in milliseconds".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @deprecated(message) - Marca como deprecated
        self.register_decorator(DecoratorSpec {
            name: "deprecated".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks function or class as deprecated".to_string(),
            allowed_on: vec![
                DecoratorTarget::Function,
                DecoratorTarget::AsyncFunction,
                DecoratorTarget::Class,
            ],
        });

        // @pure() - Función pura
        self.register_decorator(DecoratorSpec {
            name: "pure".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks function as pure (no side effects)".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @inline() - Sugerir inline
        self.register_decorator(DecoratorSpec {
            name: "inline".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Suggests to inline this function at call sites".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @readonly() - Propiedad solo lectura
        self.register_decorator(DecoratorSpec {
            name: "readonly".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks property as read-only".to_string(),
            allowed_on: vec![DecoratorTarget::ClassProperty],
        });

        // @override() - Override de clase base
        self.register_decorator(DecoratorSpec {
            name: "override".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks method as override of base class method".to_string(),
            allowed_on: vec![DecoratorTarget::ClassMethod],
        });

        // @measureTime(label) - Medir tiempo de ejecución
        self.register_decorator(DecoratorSpec {
            name: "measureTime".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Measures execution time of function".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @memoize() - Cachear resultados (alias de cache)
        self.register_decorator(DecoratorSpec {
            name: "memoize".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Memoizes function results (alias for cache)".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @throttle(ms) - Limitar llamadas
        self.register_decorator(DecoratorSpec {
            name: "throttle".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Throttles function calls to max N times per interval".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @debounce(ms) - Retardar ejecución
        self.register_decorator(DecoratorSpec {
            name: "debounce".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Debounces function calls by N milliseconds".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @retry(times) - Reintentar en error
        self.register_decorator(DecoratorSpec {
            name: "retry".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Retries function on error".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @log() - Loguear llamadas
        self.register_decorator(DecoratorSpec {
            name: "log".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Logs function calls with arguments and return value".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        // @sealed() - Prevenir extensión
        self.register_decorator(DecoratorSpec {
            name: "sealed".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks class as sealed (cannot be extended)".to_string(),
            allowed_on: vec![DecoratorTarget::Class],
        });

        // @abstract() - Clase/método abstracto
        self.register_decorator(DecoratorSpec {
            name: "abstract".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks class or method as abstract".to_string(),
            allowed_on: vec![DecoratorTarget::Class, DecoratorTarget::ClassMethod],
        });
    }

    pub fn register_decorator(&mut self, spec: DecoratorSpec) {
        self.decorators.insert(spec.name.clone(), spec);
    }

    /// Obtiene especificación de un decorador
    pub fn get(&self, name: &str) -> Option<&DecoratorSpec> {
        self.decorators.get(name)
    }

    /// Valida si un decorador existe
    pub fn exists(&self, name: &str) -> bool {
        self.decorators.contains_key(name)
    }

    /// Valida decoradores para un contexto específico
    pub fn validate(
        &self,
        decorators: &[DecoratorDecl],
        target: DecoratorTarget,
        is_in_stdlib: bool,
        file_path: Option<&str>,
    ) -> Result<Vec<DecoratorInfo>, RaccoonError> {
        let mut result = Vec::new();

        for decorator in decorators {
            let spec = self.get(&decorator.name).ok_or_else(|| {
                RaccoonError::new(
                    format!("Unknown decorator '@{}'", decorator.name),
                    decorator.position,
                    file_path.map(|s| s.to_string()),
                )
            })?;

            // Validar visibilidad: decoradores internos solo en stdlib
            if spec.visibility == DecoratorVisibility::Internal && !is_in_stdlib {
                return Err(RaccoonError::new(
                    format!(
                        "Decorator '@{}' is internal and can only be used in standard library",
                        decorator.name
                    ),
                    decorator.position,
                    file_path.map(|s| s.to_string()),
                ));
            }

            // Validar que el decorador pueda aplicarse a este target
            if !spec.allowed_on.contains(&target) {
                return Err(RaccoonError::new(
                    format!(
                        "Decorator '@{}' cannot be applied to {:?}",
                        decorator.name, target
                    ),
                    decorator.position,
                    file_path.map(|s| s.to_string()),
                ));
            }

            result.push(DecoratorInfo {
                spec: spec.clone(),
                decl: decorator.clone(),
            });
        }

        Ok(result)
    }
}

/// Información sobre un decorador aplicado
#[derive(Debug, Clone)]
pub struct DecoratorInfo {
    pub spec: DecoratorSpec,
    pub decl: DecoratorDecl,
}

impl DecoratorInfo {
    /// Obtiene el argumento de un decorador como string
    pub fn arg_as_string(&self, index: usize) -> Option<String> {
        use crate::ast::nodes::Expr;
        match self.decl.args.get(index)? {
            Expr::StrLiteral(s) => Some(s.value.clone()),
            _ => None,
        }
    }

    /// Obtiene el argumento de un decorador como int
    pub fn arg_as_int(&self, index: usize) -> Option<i64> {
        use crate::ast::nodes::Expr;
        match self.decl.args.get(index)? {
            Expr::IntLiteral(i) => Some(i.value),
            _ => None,
        }
    }

    /// Obtiene el argumento de un decorador como bool
    pub fn arg_as_bool(&self, index: usize) -> Option<bool> {
        use crate::ast::nodes::Expr;
        match self.decl.args.get(index)? {
            Expr::BoolLiteral(b) => Some(b.value),
            _ => None,
        }
    }
}

impl Default for DecoratorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator_registry_creation() {
        let registry = DecoratorRegistry::new();
        assert!(registry.exists("_ffi"));
        assert!(registry.exists("cache"));
        assert!(!registry.exists("nonexistent"));
    }

    #[test]
    fn test_decorator_visibility() {
        let registry = DecoratorRegistry::new();
        let ffi_spec = registry.get("_ffi").unwrap();
        assert_eq!(ffi_spec.visibility, DecoratorVisibility::Internal);

        let cache_spec = registry.get("cache").unwrap();
        assert_eq!(cache_spec.visibility, DecoratorVisibility::Public);
    }
}
