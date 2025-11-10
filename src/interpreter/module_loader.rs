use crate::ast::nodes::*;
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::{NullValue, ObjectValue, RuntimeValue};
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::{Interpreter, InterpreterResult};

pub struct ModuleLoader;

impl ModuleLoader {
    #[async_recursion(?Send)]
    pub async fn execute_import_decl(
        interpreter: &mut Interpreter,
        import_decl: &ImportDecl,
    ) -> Result<InterpreterResult, RaccoonError> {
        let module_spec = &import_decl.module_specifier;

        if let Some(namespace_name) = &import_decl.namespace_import {
            let namespace_obj = Self::get_module_namespace(interpreter, module_spec).await?;
            interpreter
                .environment
                .declare(namespace_name.clone(), namespace_obj)?;
            return Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )));
        }

        for spec in &import_decl.named_imports {
            let imported_name = &spec.imported;
            let local_name = spec.local.as_ref().unwrap_or(imported_name);

            let value = Self::get_module_export(interpreter, module_spec, imported_name).await?;
            interpreter.environment.declare(local_name.clone(), value)?;
        }

        if let Some(default_name) = &import_decl.default_import {
            let value = Self::get_module_export(interpreter, module_spec, "default").await?;
            interpreter
                .environment
                .declare(default_name.clone(), value)?;
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    pub async fn get_module_namespace(
        interpreter: &Interpreter,
        module_spec: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            if interpreter.stdlib_loader.module_exists(module_spec) {
                return interpreter.stdlib_loader.load_module(module_spec).await;
            } else {
                return Err(RaccoonError::new(
                    format!("Unknown module: {}", module_spec),
                    (0, 0),
                    interpreter.file.clone(),
                ));
            }
        } else if module_spec.starts_with("./") || module_spec.starts_with("../") {
            let module_path = Self::resolve_relative_path(interpreter, module_spec)?;
            return Self::load_file_module(interpreter, &module_path).await;
        } else {
            Err(RaccoonError::new(
                format!(
                    "Invalid module specifier: {}. Use 'std:' for stdlib or './', '../' for relative paths",
                    module_spec
                ),
                (0, 0),
                interpreter.file.clone(),
            ))
        }
    }

    pub async fn get_module_export(
        interpreter: &Interpreter,
        module_spec: &str,
        export_name: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            if interpreter.stdlib_loader.module_exists(module_spec) {
                return interpreter
                    .stdlib_loader
                    .get_module_export(module_spec, export_name)
                    .await;
            } else {
                return Err(RaccoonError::new(
                    format!("Unknown module: {}", module_spec),
                    (0, 0),
                    interpreter.file.clone(),
                ));
            }
        } else if module_spec.starts_with("./") || module_spec.starts_with("../") {
            let module_path = Self::resolve_relative_path(interpreter, module_spec)?;
            let module = Self::load_file_module(interpreter, &module_path).await?;

            if let RuntimeValue::Object(obj) = module {
                obj.properties.get(export_name).cloned().ok_or_else(|| {
                    RaccoonError::new(
                        format!("{} does not export '{}'", module_spec, export_name),
                        (0, 0),
                        interpreter.file.clone(),
                    )
                })
            } else {
                Err(RaccoonError::new(
                    format!("Module {} is not an object", module_spec),
                    (0, 0),
                    interpreter.file.clone(),
                ))
            }
        } else {
            Err(RaccoonError::new(
                format!(
                    "Invalid module specifier: {}. Use 'std:' for stdlib or './', '../' for relative paths",
                    module_spec
                ),
                (0, 0),
                interpreter.file.clone(),
            ))
        }
    }

    pub fn resolve_relative_path(
        interpreter: &Interpreter,
        module_spec: &str,
    ) -> Result<String, RaccoonError> {
        use std::path::PathBuf;

        let current_dir = if let Some(file) = &interpreter.file {
            PathBuf::from(file)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };

        let mut path = current_dir.join(module_spec);

        if path.extension().is_none() {
            path.set_extension("rcc");
        }

        path.to_str().map(|s| s.to_string()).ok_or_else(|| {
            RaccoonError::new(
                format!("Invalid path: {}", module_spec),
                (0, 0),
                interpreter.file.clone(),
            )
        })
    }

    pub async fn load_file_module(
        interpreter: &Interpreter,
        path: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        use crate::lexer::Lexer;
        use crate::parser::Parser;
        use std::fs;

        let source = fs::read_to_string(path).map_err(|e| {
            RaccoonError::new(
                format!("Failed to read module {}: {}", path, e),
                (0, 0),
                interpreter.file.clone(),
            )
        })?;

        let file_path = Some(path.to_string());
        let mut lexer = Lexer::new(source, file_path.clone());
        let tokens = lexer.tokenize().map_err(|e| {
            RaccoonError::new(
                format!("Lexer error in {}: {:?}", path, e),
                (0, 0),
                file_path.clone(),
            )
        })?;

        let mut parser = Parser::new(tokens, file_path.clone());
        let program = parser.parse().map_err(|e| {
            RaccoonError::new(
                format!("Parser error in {}: {:?}", path, e),
                (0, 0),
                file_path.clone(),
            )
        })?;

        let mut module_interp = Interpreter::new(file_path.clone());

        let mut exports = HashMap::new();
        let mut default_export: Option<RuntimeValue> = None;

        for stmt in &program.stmts {
            match stmt {
                Stmt::ExportDecl(export_decl) => {
                    if export_decl.is_default {
                        if let Some(decl) = &export_decl.declaration {
                            match decl.as_ref() {
                                Stmt::ExprStmt(e) => {
                                    default_export =
                                        Some(module_interp.eval_expr_public(&e.expression).await?);
                                }
                                _ => {
                                    module_interp.execute_stmt(decl).await?;
                                    let name = match decl.as_ref() {
                                        Stmt::FnDecl(f) => &f.name,
                                        Stmt::ClassDecl(c) => &c.name,
                                        Stmt::EnumDecl(e) => &e.name,
                                        _ => {
                                            return Err(RaccoonError::new(
                                                "Invalid default export",
                                                (0, 0),
                                                file_path.clone(),
                                            ));
                                        }
                                    };
                                    default_export = Some(module_interp.get_from_env(name)?);
                                }
                            }
                        }
                    } else if let Some(decl) = &export_decl.declaration {
                        let name = match decl.as_ref() {
                            Stmt::FnDecl(f) => f.name.clone(),
                            Stmt::ClassDecl(c) => c.name.clone(),
                            Stmt::EnumDecl(e) => e.name.clone(),
                            Stmt::VarDecl(v) => match &v.pattern {
                                VarPattern::Identifier(id) => id.clone(),
                                _ => {
                                    return Err(RaccoonError::new(
                                        "Cannot export destructured variable",
                                        (0, 0),
                                        file_path.clone(),
                                    ));
                                }
                            },
                            _ => {
                                return Err(RaccoonError::new(
                                    "Invalid export declaration",
                                    (0, 0),
                                    file_path.clone(),
                                ));
                            }
                        };
                        module_interp.execute_stmt(decl).await?;
                        if let Ok(val) = module_interp.get_from_env(&name) {
                            exports.insert(name, val);
                        }
                    } else {
                        if let Some(module_spec) = &export_decl.module_specifier {
                            let source_module_path =
                                Self::resolve_relative_path(&module_interp, module_spec)?;
                            let source_module = Box::pin(Self::load_file_module(
                                &module_interp,
                                &source_module_path,
                            ))
                            .await?;

                            if let RuntimeValue::Object(obj) = source_module {
                                for spec in &export_decl.specifiers {
                                    let import_name = &spec.local;
                                    let export_name = spec.exported.as_ref().unwrap_or(import_name);

                                    if let Some(val) = obj.properties.get(import_name) {
                                        exports.insert(export_name.clone(), val.clone());
                                    } else {
                                        return Err(RaccoonError::new(
                                            format!(
                                                "{} does not export '{}'",
                                                module_spec, import_name
                                            ),
                                            (0, 0),
                                            file_path.clone(),
                                        ));
                                    }
                                }
                            } else {
                                return Err(RaccoonError::new(
                                    format!("Module {} is not an object", module_spec),
                                    (0, 0),
                                    file_path.clone(),
                                ));
                            }
                        } else {
                            for spec in &export_decl.specifiers {
                                let exported_name =
                                    spec.exported.clone().unwrap_or(spec.local.clone());
                                if let Ok(val) = module_interp.get_from_env(&spec.local) {
                                    exports.insert(exported_name, val);
                                }
                            }
                        }
                    }
                }
                _ => {
                    module_interp.execute_stmt(stmt).await?;
                }
            }
        }

        if let Some(default_val) = default_export {
            exports.insert("default".into(), default_val);
        }

        Ok(RuntimeValue::Object(ObjectValue::new(
            exports,
            PrimitiveType::any(),
        )))
    }
}
