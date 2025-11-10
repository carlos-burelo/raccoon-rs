use crate::ast::nodes::{Program, Stmt, VarPattern};
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::values::{NullValue, ObjectValue, RuntimeValue};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct StdLibLoader {
    stdlib_path: PathBuf,
    module_cache: Arc<RwLock<HashMap<String, RuntimeValue>>>,
}

impl StdLibLoader {
    pub fn new(stdlib_path: PathBuf) -> Self {
        Self {
            stdlib_path,
            module_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_default_path() -> Self {
        Self::new(PathBuf::from("./stdlib"))
    }

    pub fn module_exists(&self, module_name: &str) -> bool {
        // Virtual module always exists
        if module_name == "internal:core" {
            return true;
        }
        if !module_name.starts_with("std:") {
            return false;
        }
        self.get_module_path(module_name).exists()
    }

    fn get_module_path(&self, module_name: &str) -> PathBuf {
        let basename = module_name.strip_prefix("std:").unwrap_or(module_name);
        self.stdlib_path.join(format!("{}.rcc", basename))
    }

    /// Load the virtual "internal:core" module
    /// This module exposes all core primitives registered in the system
    fn load_core_module(&self) -> Result<RuntimeValue, RaccoonError> {
        let mut exports = HashMap::new();

        // Create a temporary interpreter to get the registrar
        let interp = Interpreter::new(Some("internal:core".to_string()));
        let registrar = interp.registrar.lock().unwrap();

        // Export all functions that start with "core_" without namespace
        for (full_name, sig) in &registrar.functions {
            if full_name.starts_with("core_") && sig.namespace.is_none() {
                // Create export name without "core_" prefix
                let export_name = full_name.strip_prefix("core_").unwrap_or(full_name);

                // Export as NativeFunction (not async) since these are synchronous operations
                let handler = sig.handler.clone();
                let native_fn = crate::runtime::NativeFunctionValue::new(
                    move |args| handler(args),
                    crate::ast::types::Type::Primitive(PrimitiveType::any()),
                );
                let function_value = RuntimeValue::NativeFunction(native_fn);

                // Export both with and without prefix for compatibility
                exports.insert(export_name.to_string(), function_value.clone());
                exports.insert(full_name.to_string(), function_value);
            }
        }

        Ok(RuntimeValue::Object(ObjectValue::new(
            exports,
            PrimitiveType::any(),
        )))
    }

    pub async fn load_module(&self, module_name: &str) -> Result<RuntimeValue, RaccoonError> {
        // Handle virtual "internal:core" module
        if module_name == "internal:core" {
            return self.load_core_module();
        }

        {
            let cache = self.module_cache.read().unwrap();
            if let Some(value) = cache.get(module_name) {
                return Ok(value.clone());
            }
        }

        let module_path = self.get_module_path(module_name);
        let source = fs::read_to_string(&module_path).map_err(|e| {
            RaccoonError::new(
                format!("Failed to read module {}: {}", module_name, e),
                (0, 0),
                Some(module_path.display().to_string()),
            )
        })?;

        let file_path = Some(module_path.display().to_string());
        let mut lexer = Lexer::new(source, file_path.clone());
        let tokens = lexer.tokenize().map_err(|e| {
            RaccoonError::new(
                format!("Lexer error in {}: {:?}", module_name, e),
                (0, 0),
                file_path.clone(),
            )
        })?;

        let mut parser = Parser::new(tokens, file_path.clone());
        let program = parser.parse().map_err(|e| {
            RaccoonError::new(
                format!("Parser error in {}: {:?}", module_name, e),
                (0, 0),
                file_path.clone(),
            )
        })?;

        let exports = self
            .execute_module_and_extract_exports(&program, file_path.clone())
            .await?;

        let module_obj = RuntimeValue::Object(ObjectValue::new(exports, PrimitiveType::any()));

        {
            let mut cache = self.module_cache.write().unwrap();
            cache.insert(module_name.to_string(), module_obj.clone());
        }

        Ok(module_obj)
    }

    pub async fn get_module_export(
        &self,
        module_name: &str,
        export_name: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        let module = self.load_module(module_name).await?;
        if let RuntimeValue::Object(obj) = module {
            obj.properties.get(export_name).cloned().ok_or_else(|| {
                RaccoonError::new(
                    format!("{} does not export '{}'", module_name, export_name),
                    (0, 0),
                    Option::<String>::None,
                )
            })
        } else {
            Err(RaccoonError::new(
                format!("Module {} is not an object", module_name),
                (0, 0),
                Option::<String>::None,
            ))
        }
    }

    async fn execute_module_and_extract_exports(
        &self,
        ast: &Program,
        file_path: Option<String>,
    ) -> Result<HashMap<String, RuntimeValue>, RaccoonError> {
        let mut interp = Interpreter::new(file_path.clone());
        // Share the same stdlib_loader so nested imports work
        interp.stdlib_loader = std::sync::Arc::new(Self {
            stdlib_path: self.stdlib_path.clone(),
            module_cache: self.module_cache.clone(),
        });
        self.setup_native_functions_in_interpreter(&mut interp, &file_path);

        let mut exports = HashMap::new();
        let mut default_export: Option<RuntimeValue> = None;

        for stmt in &ast.stmts {
            match stmt {
                Stmt::ExportDecl(export_decl) => {
                    if export_decl.is_default {
                        if let Some(decl) = &export_decl.declaration {
                            default_export =
                                Some(self.extract_declaration_value(decl, &mut interp).await?);
                        }
                    } else if let Some(decl) = &export_decl.declaration {
                        let name = self.get_declaration_name(decl)?;

                        match decl.as_ref() {
                            Stmt::InterfaceDecl(_) | Stmt::TypeAliasDecl(_) => {
                                exports.insert(name, RuntimeValue::Null(NullValue));
                            }
                            _ => {
                                interp.execute_stmt(decl).await?;
                                if let Ok(val) = interp.get_from_env(&name) {
                                    exports.insert(name, val);
                                }
                            }
                        }
                    } else {
                        for spec in &export_decl.specifiers {
                            let exported_name = spec.exported.clone().unwrap_or(spec.local.clone());
                            if let Ok(val) = interp.get_from_env(&spec.local) {
                                exports.insert(exported_name, val);
                            }
                        }
                    }
                }
                _ => {
                    let _ = interp.execute_stmt(stmt).await?;
                }
            }
        }

        if let Some(default_val) = default_export {
            exports.insert("default".into(), default_val);
        }

        // Inject native module functions as exports if they're not already present
        if let Some(path_str) = &file_path {
            let module_name = if path_str.contains("stdlib/math.rcc")
                || path_str.contains("stdlib\\math.rcc")
            {
                Some("math")
            } else if path_str.contains("stdlib/json.rcc") || path_str.contains("stdlib\\json.rcc")
            {
                Some("json")
            } else if path_str.contains("stdlib/http.rcc") || path_str.contains("stdlib\\http.rcc")
            {
                Some("http")
            } else {
                None
            };

            if let Some(module) = module_name {
                if let Some(loader) = interp.module_registry.get_loader(module) {
                    let mut registrar = interp.registrar.lock().unwrap();
                    loader(&mut registrar);

                    // Add native functions to exports if not already present
                    for (_full_name, sig) in &registrar.functions {
                        if sig.namespace.as_ref() == Some(&module.to_string()) {
                            let func_name = &sig.name;
                            if !exports.contains_key(func_name) {
                                // Create a simple wrapper that will be called at runtime
                                let _handler = sig.handler.clone();

                                // We need to store this handler in a way that can be called
                                // Since we can't convert it to a function pointer, we'll use
                                // a workaround: create a NativeAsyncFunction that wraps it

                                // For now, let's just log that we're skipping this
                                // and rely on the .rcc files to export them
                            }
                        }
                    }
                }
            }
        }

        Ok(exports)
    }

    fn get_declaration_name(&self, stmt: &Stmt) -> Result<String, RaccoonError> {
        match stmt {
            Stmt::FnDecl(f) => Ok(f.name.clone()),
            Stmt::ClassDecl(c) => Ok(c.name.clone()),
            Stmt::VarDecl(v) => match &v.pattern {
                VarPattern::Identifier(id) => Ok(id.clone()),
                _ => Err(RaccoonError::new(
                    "Cannot export destructured variable",
                    (0, 0),
                    Option::<String>::None,
                )),
            },
            Stmt::EnumDecl(e) => Ok(e.name.clone()),
            Stmt::InterfaceDecl(i) => Ok(i.name.clone()),
            Stmt::TypeAliasDecl(t) => Ok(t.name.clone()),
            _ => Err(RaccoonError::new(
                "Invalid export declaration",
                (0, 0),
                Option::<String>::None,
            )),
        }
    }

    async fn extract_declaration_value(
        &self,
        stmt: &Stmt,
        interp: &mut Interpreter,
    ) -> Result<RuntimeValue, RaccoonError> {
        match stmt {
            Stmt::FnDecl(f) => {
                interp.execute_stmt(stmt).await?;
                interp.get_from_env(&f.name)
            }
            Stmt::ClassDecl(c) => {
                interp.execute_stmt(stmt).await?;
                interp.get_from_env(&c.name)
            }
            Stmt::EnumDecl(e) => {
                interp.execute_stmt(stmt).await?;
                interp.get_from_env(&e.name)
            }
            Stmt::InterfaceDecl(_i) => Ok(RuntimeValue::Null(NullValue)),
            Stmt::TypeAliasDecl(_t) => Ok(RuntimeValue::Null(NullValue)),
            Stmt::ExprStmt(e) => Ok(interp.eval_expr_public(&e.expression).await?),
            _ => Err(RaccoonError::new(
                "Invalid default export",
                (0, 0),
                Option::<String>::None,
            )),
        }
    }

    fn setup_native_functions_in_interpreter(
        &self,
        interp: &mut Interpreter,
        _file_path: &Option<String>,
    ) {
        // Register wrapper functions that stdlib modules can use
        super::register_stdlib_wrappers(
            &mut interp.environment,
            interp.registrar.clone(),
        );

        // Also register all core_* functions directly in the environment
        let registrar = interp.registrar.lock().unwrap();
        for (full_name, _sig) in &registrar.functions {
            if full_name.starts_with("core_") && _sig.namespace.is_none() {
                // Get the export name without prefix
                let export_name = full_name.strip_prefix("core_").unwrap_or(full_name);

                // Create a wrapper RuntimeValue::NativeRegisteredFunction that references the registered function
                if let Some(func) = registrar.get_function(full_name, None) {
                    // Store the function name so it can be called via the registrar
                    let func_name = full_name.clone();
                    let reg_clone = interp.registrar.clone();
                    let wrapper = crate::runtime::NativeFunctionValue::new(
                        move |args: Vec<RuntimeValue>| {
                            let reg = reg_clone.lock().unwrap();
                            if let Some(f) = reg.get_function(&func_name, None) {
                                (f.handler)(args)
                            } else {
                                RuntimeValue::Null(crate::runtime::NullValue::new())
                            }
                        },
                        crate::ast::types::PrimitiveType::any(),
                    );

                    // Register with both names (with and without core_ prefix)
                    let _ = interp.environment.declare(
                        export_name.to_string(),
                        RuntimeValue::NativeFunction(wrapper.clone()),
                    );
                    let _ = interp.environment.declare(
                        full_name.clone(),
                        RuntimeValue::NativeFunction(wrapper),
                    );
                }
            }
        }
    }

    pub fn available_modules(&self) -> Vec<String> {
        let mut modules = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.stdlib_path) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|e| e == "rcc") {
                    if let Some(name) = entry.path().file_stem() {
                        modules.push(format!("std:{}", name.to_string_lossy()));
                    }
                }
            }
        }
        modules
    }

    pub fn clear_cache(&self) {
        self.module_cache.write().unwrap().clear();
    }
}

impl Default for StdLibLoader {
    fn default() -> Self {
        Self::with_default_path()
    }
}
