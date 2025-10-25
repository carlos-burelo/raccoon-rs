use crate::ast::nodes::{Program, Stmt};
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Module {
    pub path: PathBuf,
    pub exports: HashMap<String, RuntimeValue>,
    pub default_export: Option<RuntimeValue>,
    pub ast: Program,
}

impl Module {
    pub fn new(path: PathBuf, ast: Program) -> Self {
        Self {
            path,
            exports: HashMap::new(),
            default_export: None,
            ast,
        }
    }

    pub fn add_named_export(&mut self, name: String, value: RuntimeValue) {
        self.exports.insert(name, value);
    }

    pub fn set_default_export(&mut self, value: RuntimeValue) {
        self.default_export = Some(value);
    }

    pub fn get_export(&self, name: &str) -> Option<&RuntimeValue> {
        self.exports.get(name)
    }

    pub fn get_default(&self) -> Option<&RuntimeValue> {
        self.default_export.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleCache {
    modules: Arc<RwLock<HashMap<PathBuf, Module>>>,
}

impl ModuleCache {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, path: &Path) -> Option<Module> {
        self.modules.read().unwrap().get(path).cloned()
    }

    pub fn insert(&self, path: PathBuf, module: Module) {
        self.modules.write().unwrap().insert(path, module);
    }

    pub fn contains(&self, path: &Path) -> bool {
        self.modules.read().unwrap().contains_key(path)
    }
}

pub fn resolve_module_path(
    current_file: Option<&String>,
    module_specifier: &str,
) -> Result<PathBuf, RaccoonError> {
    if module_specifier.starts_with("./") || module_specifier.starts_with("../") {
        let current_dir = if let Some(file) = current_file {
            PathBuf::from(file)
                .parent()
                .unwrap_or(Path::new("."))
                .to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };

        let mut path = current_dir.join(module_specifier);

        if path.extension().is_none() {
            path.set_extension("rcc");
        }

        Ok(path)
    } else if !module_specifier.starts_with('/') && !module_specifier.contains('\\') {
        let stdlib_path = PathBuf::from("stdlib").join(format!("{}.rcc", module_specifier));
        Ok(stdlib_path)
    } else {
        let mut path = PathBuf::from(module_specifier);
        if path.extension().is_none() {
            path.set_extension("rcc");
        }
        Ok(path)
    }
}

pub fn analyze_exports(program: &Program) -> (Vec<String>, bool) {
    let mut named_exports = Vec::new();
    let mut has_default = false;

    for stmt in &program.stmts {
        if let Stmt::ExportDecl(export) = stmt {
            if export.is_default {
                has_default = true;
            } else {
                for spec in &export.specifiers {
                    let export_name = spec.exported.as_ref().unwrap_or(&spec.local);
                    named_exports.push(export_name.clone());
                }

                if let Some(decl) = &export.declaration {
                    match decl.as_ref() {
                        Stmt::VarDecl(var_decl) => {
                            if let crate::ast::nodes::VarPattern::Identifier(name) =
                                &var_decl.pattern
                            {
                                named_exports.push(name.clone());
                            }
                        }
                        Stmt::FnDecl(fn_decl) => {
                            named_exports.push(fn_decl.name.clone());
                        }
                        Stmt::ClassDecl(class_decl) => {
                            named_exports.push(class_decl.name.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    (named_exports, has_default)
}

#[derive(Debug, Clone)]
pub struct ModuleSystem {
    pub cache: ModuleCache,
}

impl ModuleSystem {
    pub fn new() -> Self {
        Self {
            cache: ModuleCache::new(),
        }
    }

    pub fn load_module(&mut self, path: PathBuf, module: Module) {
        self.cache.insert(path, module);
    }

    pub fn get_module(&self, path: &Path) -> Option<Module> {
        self.cache.get(path)
    }
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self::new()
    }
}
