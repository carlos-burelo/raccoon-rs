use crate::ast::nodes::DecoratorDecl;
use crate::error::RaccoonError;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoratorVisibility {
    Internal,

    Public,
}

#[derive(Debug, Clone)]
pub struct DecoratorSpec {
    pub name: String,
    pub visibility: DecoratorVisibility,
    pub description: String,
    pub allowed_on: Vec<DecoratorTarget>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoratorTarget {
    Function,
    AsyncFunction,
    Class,
    ClassMethod,
    ClassProperty,
}

#[derive(Clone)]
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
        self.register_decorator(DecoratorSpec {
            name: "cache".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Caches function results for specified time in milliseconds".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

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

        self.register_decorator(DecoratorSpec {
            name: "pure".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks function as pure (no side effects)".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "inline".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Suggests to inline this function at call sites".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "readonly".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks property as read-only".to_string(),
            allowed_on: vec![DecoratorTarget::ClassProperty],
        });

        self.register_decorator(DecoratorSpec {
            name: "override".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks method as override of base class method".to_string(),
            allowed_on: vec![DecoratorTarget::ClassMethod],
        });

        self.register_decorator(DecoratorSpec {
            name: "measureTime".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Measures execution time of function".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "memoize".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Memoizes function results (alias for cache)".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "throttle".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Throttles function calls to max N times per interval".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "debounce".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Debounces function calls by N milliseconds".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "retry".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Retries function on error".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "log".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Logs function calls with arguments and return value".to_string(),
            allowed_on: vec![DecoratorTarget::Function, DecoratorTarget::AsyncFunction],
        });

        self.register_decorator(DecoratorSpec {
            name: "sealed".to_string(),
            visibility: DecoratorVisibility::Public,
            description: "Marks class as sealed (cannot be extended)".to_string(),
            allowed_on: vec![DecoratorTarget::Class],
        });

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

    pub fn get(&self, name: &str) -> Option<&DecoratorSpec> {
        self.decorators.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.decorators.contains_key(name)
    }

    pub fn validate(
        &self,
        decorators: &[DecoratorDecl],
        target: DecoratorTarget,
        is_in_stdlib: bool,
        file_path: Option<&str>,
    ) -> Result<Vec<DecoratorInfo>, RaccoonError> {
        let mut result = Vec::new();

        for decorator in decorators {
            let spec = if let Some(spec) = self.get(&decorator.name) {
                spec.clone()
            } else {
                DecoratorSpec {
                    name: decorator.name.clone(),
                    description: format!("User-defined decorator: {}", decorator.name),
                    visibility: DecoratorVisibility::Public,
                    allowed_on: vec![
                        DecoratorTarget::Function,
                        DecoratorTarget::Class,
                        DecoratorTarget::ClassMethod,
                    ],
                }
            };

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
                spec,
                decl: decorator.clone(),
            });
        }

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct DecoratorInfo {
    pub spec: DecoratorSpec,
    pub decl: DecoratorDecl,
}

impl DecoratorInfo {
    pub fn arg_as_string(&self, index: usize) -> Option<String> {
        use crate::ast::nodes::Expr;
        match self.decl.args.get(index)? {
            Expr::StrLiteral(s) => Some(s.value.clone()),
            _ => None,
        }
    }

    pub fn arg_as_int(&self, index: usize) -> Option<i64> {
        use crate::ast::nodes::Expr;
        match self.decl.args.get(index)? {
            Expr::IntLiteral(i) => Some(i.value),
            _ => None,
        }
    }

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
