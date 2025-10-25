use crate::ast::nodes::{Program, Stmt, VarPattern};
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::NativeBridge;
use crate::runtime::values::{ObjectValue, RuntimeValue};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct StdLibLoader {
    stdlib_path: PathBuf,
    module_cache: Arc<RwLock<HashMap<String, RuntimeValue>>>,
    native_bridge: Arc<NativeBridge>,
}

impl StdLibLoader {
    pub fn new(stdlib_path: PathBuf) -> Self {
        Self {
            stdlib_path,
            module_cache: Arc::new(RwLock::new(HashMap::new())),
            native_bridge: Arc::new(NativeBridge::new()),
        }
    }

    pub fn with_default_path() -> Self {
        Self::new(PathBuf::from("./stdlib"))
    }

    pub fn module_exists(&self, module_name: &str) -> bool {
        if !module_name.starts_with("std:") {
            return false;
        }
        self.get_module_path(module_name).exists()
    }

    fn get_module_path(&self, module_name: &str) -> PathBuf {
        let basename = module_name.strip_prefix("std:").unwrap_or(module_name);
        self.stdlib_path.join(format!("{}.rcc", basename))
    }

    pub async fn load_module(&self, module_name: &str) -> Result<RuntimeValue, RaccoonError> {
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
        self.setup_native_functions_in_interpreter(&mut interp);

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
                        interp.execute_stmt(decl).await?;
                        if let Ok(val) = interp.get_from_env(&name) {
                            exports.insert(name, val);
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
            Stmt::ExprStmt(e) => Ok(interp.eval_expr_public(&e.expression).await?),
            _ => Err(RaccoonError::new(
                "Invalid default export",
                (0, 0),
                Option::<String>::None,
            )),
        }
    }

    fn setup_native_functions_in_interpreter(&self, interp: &mut Interpreter) {
        let all_native_names = vec![
            "_print_native",
            "_eprint_native",
            "native_http_request",
            "native_time_now",
            "native_time_now_secs",
            "native_random",
            "native_json_parse",
            "native_json_stringify",
            "native_json_stringify_pretty",
            "native_str_length",
            "native_str_upper",
            "native_str_lower",
            "native_str_trim",
            "native_str_substring",
            "native_str_char_at",
            "native_str_index_of",
            "native_str_replace",
            "native_str_split",
            "native_str_starts_with",
            "native_str_ends_with",
            "native_array_length",
            "native_array_push",
            "native_array_slice",
            "native_array_reverse",
            "native_array_pop",
            "native_array_shift",
            "native_array_unshift",
            "native_array_sort",
            "native_ffi_load_library",
            "native_ffi_register_function",
            "native_ffi_call",
            "getOS",
            "native_io_read_file",
            "native_io_write_file",
            "native_io_append_file",
            "native_io_file_exists",
            "native_io_delete_file",
            "native_io_read_dir",
            "native_io_create_dir",
            "native_io_input",
            "_sqrt_native",
            "_pow_native",
            "_sin_native",
            "_cos_native",
            "_tan_native",
            "_random_native",
        ];

        for name in all_native_names {
            if let Some(f) = self.native_bridge.get(name) {
                let _ = interp.declare_in_env(name.to_string(), f);
            }
        }

        if let Some(f) = self.native_bridge.get_async("native_http_request") {
            let _ = interp.declare_in_env("native_http_request".to_string(), f);
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
