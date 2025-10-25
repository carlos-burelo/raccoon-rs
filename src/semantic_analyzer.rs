use std::collections::HashMap;

use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::{SymbolItem, SymbolKind, SymbolTable},
    type_checker::TypeChecker,
    type_inference::TypeInferenceEngine,
    type_resolver::TypeResolver,
};

pub struct SemanticAnalyzer {
    pub file: Option<String>,
    pub symbol_table: SymbolTable,
    pub type_checker: TypeChecker,
    pub type_inference: TypeInferenceEngine,
    pub current_function: Option<SymbolItem>,
    pub current_class: Option<SymbolItem>,
    pub in_loop: bool,
    pub in_async_function: bool,
}

impl SemanticAnalyzer {
    pub fn new(file: Option<String>) -> Self {
        Self {
            file: file.clone(),
            symbol_table: SymbolTable::new(file.clone()),
            type_checker: TypeChecker::new(file.clone()),
            type_inference: TypeInferenceEngine::new(file.clone()),
            current_function: None,
            current_class: None,
            in_loop: false,
            in_async_function: false,
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self {
            file: None,
            symbol_table,
            type_checker: TypeChecker::new(None),
            type_inference: TypeInferenceEngine::new(None),
            current_function: None,
            current_class: None,
            in_loop: false,
            in_async_function: false,
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), RaccoonError> {
        self.first_pass(program)?;
        self.second_pass(program)?;
        Ok(())
    }

    fn first_pass(&mut self, program: &Program) -> Result<(), RaccoonError> {
        for stmt in &program.stmts {
            match stmt {
                Stmt::ClassDecl(decl) => self.register_class(decl)?,
                Stmt::InterfaceDecl(decl) => self.register_interface(decl)?,
                Stmt::EnumDecl(decl) => self.register_enum(decl)?,
                Stmt::TypeAliasDecl(decl) => self.register_type_alias(decl)?,
                Stmt::FnDecl(decl) => self.register_function(decl)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn second_pass(&mut self, program: &Program) -> Result<(), RaccoonError> {
        for stmt in &program.stmts {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    fn register_class(&mut self, decl: &ClassDecl) -> Result<(), RaccoonError> {
        let mut superclass = None;
        if let Some(ref superclass_name) = decl.superclass {
            let super_symbol = self.symbol_table.lookup(superclass_name).ok_or_else(|| {
                RaccoonError::new(
                    format!("Superclass '{}' not found", superclass_name),
                    decl.position,
                    self.file.clone(),
                )
            })?;

            if super_symbol.kind != SymbolKind::Class {
                return Err(RaccoonError::new(
                    format!("'{}' is not a class", superclass_name),
                    decl.position,
                    self.file.clone(),
                ));
            }

            if let Type::Class(ref class_type) = super_symbol.symbol_type {
                superclass = Some(class_type.clone());
            }
        }

        let class_type = ClassType {
            name: decl.name.clone(),
            superclass,
            properties: HashMap::new(),
            methods: HashMap::new(),
            constructor: None,
            type_parameters: decl.type_parameters.clone(),
        };

        self.symbol_table.define(
            decl.name.clone(),
            SymbolKind::Class,
            Type::Class(Box::new(class_type)),
            false,
            Some(Box::new(Stmt::ClassDecl(decl.clone()))),
        );

        Ok(())
    }

    fn register_interface(&mut self, decl: &InterfaceDecl) -> Result<(), RaccoonError> {
        let mut properties = HashMap::new();

        for prop in &decl.properties {
            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            let resolved_type = resolver.resolve(&prop.property_type)?;

            properties.insert(
                prop.name.clone(),
                InterfaceProperty {
                    property_type: resolved_type,
                    optional: prop.optional,
                },
            );
        }

        let interface_type = InterfaceType {
            name: decl.name.clone(),
            properties,
            type_parameters: decl.type_parameters.clone(),
        };

        self.symbol_table.define(
            decl.name.clone(),
            SymbolKind::Interface,
            Type::Interface(Box::new(interface_type)),
            false,
            Some(Box::new(Stmt::InterfaceDecl(decl.clone()))),
        );

        Ok(())
    }

    fn register_enum(&mut self, decl: &EnumDecl) -> Result<(), RaccoonError> {
        let mut members = HashMap::new();
        let mut current_value = 0i64;

        for member in &decl.members {
            if let Some(ref value_expr) = member.value {
                match value_expr {
                    Expr::IntLiteral(lit) => {
                        current_value = lit.value;
                        members.insert(member.name.clone(), EnumValue::Int(current_value));
                    }
                    Expr::StrLiteral(lit) => {
                        members.insert(member.name.clone(), EnumValue::Str(lit.value.clone()));
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Enum member value must be int or string literal",
                            decl.position,
                            self.file.clone(),
                        ));
                    }
                }
            } else {
                members.insert(member.name.clone(), EnumValue::Int(current_value));
            }
            current_value += 1;
        }

        let enum_type = EnumType {
            name: decl.name.clone(),
            members,
        };

        self.symbol_table.define(
            decl.name.clone(),
            SymbolKind::Enum,
            Type::Enum(Box::new(enum_type)),
            false,
            Some(Box::new(Stmt::EnumDecl(decl.clone()))),
        );

        Ok(())
    }

    fn register_type_alias(&mut self, decl: &TypeAliasDecl) -> Result<(), RaccoonError> {
        let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
        let resolved_type = resolver.resolve(&decl.alias_type)?;

        self.symbol_table.define(
            decl.name.clone(),
            SymbolKind::TypeAlias,
            resolved_type,
            false,
            Some(Box::new(Stmt::TypeAliasDecl(decl.clone()))),
        );

        Ok(())
    }

    fn register_function(&mut self, decl: &FnDecl) -> Result<(), RaccoonError> {
        let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());

        let mut param_types = Vec::new();
        for param in &decl.parameters {
            let resolved = resolver.resolve(&param.param_type)?;
            param_types.push(resolved);
        }

        // If return type is not explicitly specified, use 'unknown' for now
        // It will be inferred during the second pass in check_fn_decl
        let mut return_type = if let Some(ref ret_type) = decl.return_type {
            resolver.resolve(ret_type)?
        } else {
            PrimitiveType::unknown()
        };

        if decl.is_async {
            if !matches!(return_type, Type::Future(_)) {
                return_type = Type::Future(Box::new(FutureType {
                    inner_type: return_type,
                }));
            }
        }

        let fn_type = FunctionType {
            params: param_types,
            return_type,
            is_variadic: false,
        };

        self.symbol_table.define(
            decl.name.clone(),
            SymbolKind::Function,
            Type::Function(Box::new(fn_type)),
            false,
            Some(Box::new(Stmt::FnDecl(decl.clone()))),
        );

        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<Type, RaccoonError> {
        match stmt {
            Stmt::Program(program) => {
                for s in &program.stmts {
                    self.check_stmt(s)?;
                }
                Ok(PrimitiveType::void())
            }

            Stmt::VarDecl(decl) => self.check_var_decl(decl),
            Stmt::FnDecl(decl) => self.check_fn_decl(decl),
            Stmt::ClassDecl(decl) => self.check_class_decl(decl),
            Stmt::InterfaceDecl(_) => Ok(PrimitiveType::void()),
            Stmt::EnumDecl(_) => Ok(PrimitiveType::void()),
            Stmt::TypeAliasDecl(_) => Ok(PrimitiveType::void()),
            Stmt::ImportDecl(_) => Ok(PrimitiveType::void()),
            Stmt::ExportDecl(decl) => self.check_export_decl(decl),
            Stmt::Block(block) => self.check_block(block),
            Stmt::IfStmt(stmt) => self.check_if_stmt(stmt),
            Stmt::WhileStmt(stmt) => self.check_while_stmt(stmt),
            Stmt::ForStmt(stmt) => self.check_for_stmt(stmt),
            Stmt::ForInStmt(stmt) => self.check_for_in_stmt(stmt),
            Stmt::ReturnStmt(stmt) => self.check_return_stmt(stmt),
            Stmt::BreakStmt(_) => self.check_break_stmt(),
            Stmt::ContinueStmt(_) => self.check_continue_stmt(),
            Stmt::ExprStmt(stmt) => self.check_expr(&stmt.expression),
            Stmt::TryStmt(stmt) => self.check_try_stmt(stmt),
            Stmt::ThrowStmt(stmt) => self.check_throw_stmt(stmt),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, RaccoonError> {
        match expr {
            Expr::Binary(e) => self.check_binary_expr(e),
            Expr::Unary(e) => self.check_unary_expr(e),
            Expr::Call(e) => self.check_call_expr(e),
            Expr::New(e) => self.check_new_expr(e),
            Expr::Member(e) => self.check_member_expr(e),
            Expr::MethodCall(e) => self.check_method_call_expr(e),
            Expr::Index(e) => self.check_index_expr(e),
            Expr::Await(e) => self.check_await_expr(e),
            Expr::This(_) => self.check_this_expr(),
            Expr::Super(_) => self.check_super_expr(),
            Expr::TypeOf(_) => Ok(PrimitiveType::str()),
            Expr::InstanceOf(e) => self.check_instanceof_expr(e),
            Expr::ArrowFn(e) => self.check_arrow_fn_expr(e),
            Expr::Identifier(e) => self.check_identifier(e),
            Expr::Assignment(e) => self.check_assignment(e),
            Expr::Range(e) => self.check_range_expr(e),
            Expr::Conditional(e) => self.check_conditional_expr(e),
            Expr::NullCoalescing(e) => self.check_null_coalescing_expr(e),
            Expr::OptionalChaining(e) => self.check_optional_chaining_expr(e),
            Expr::NullAssertion(e) => self.check_null_assertion_expr(e),
            Expr::UnaryUpdate(e) => self.check_unary_update_expr(e),
            Expr::TemplateStr(_) => Ok(PrimitiveType::str()),
            Expr::TaggedTemplate(_) => Ok(PrimitiveType::str()),
            Expr::IntLiteral(_) => Ok(PrimitiveType::int()),
            Expr::FloatLiteral(_) => Ok(PrimitiveType::float()),
            Expr::StrLiteral(_) => Ok(PrimitiveType::str()),
            Expr::BoolLiteral(_) => Ok(PrimitiveType::bool()),
            Expr::NullLiteral(_) => Ok(PrimitiveType::null()),
            Expr::ListLiteral(e) => self.check_list_literal(e),
            Expr::ObjectLiteral(e) => self.check_object_literal(e),
            Expr::Spread(_) => {
                // Spread should only appear in call arguments context, not as standalone expression
                Err(RaccoonError::new(
                    "Spread operator cannot be used outside of function calls",
                    (1, 1),
                    self.file.clone(),
                ))
            }
        }
    }

    // fn check_var_decl(&mut self, decl: &VarDecl) -> Result<Type, RaccoonError> {
    //     let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
    //     let mut var_type = resolver.resolve(&decl.type_annotation)?;

    //     if let Some(ref initializer) = decl.initializer {
    //         let init_type = self.check_expr(initializer)?;

    //         if matches!(var_type.kind(), TypeKind::Any) {
    //             var_type = init_type.clone();
    //         }

    //         if !init_type.is_assignable_to(&var_type) {
    //             return Err(RaccoonError::new(
    //                 format!(
    //                     "Cannot assign type '{:?}' to variable of type '{:?}'",
    //                     init_type, var_type
    //                 ),
    //                 decl.position,
    //                 self.file.clone(),
    //             ));
    //         }

    //         if let VarPattern::Identifier(ref name) = decl.pattern {
    //             self.symbol_table.define(
    //                 name.clone(),
    //                 SymbolKind::Variable,
    //                 var_type.clone(),
    //                 decl.is_constant,
    //                 Some(Box::new(Stmt::VarDecl(decl.clone()))),
    //             );
    //         }
    //     } else if let VarPattern::Identifier(ref name) = decl.pattern {
    //         self.symbol_table.define(
    //             name.clone(),
    //             SymbolKind::Variable,
    //             var_type.clone(),
    //             decl.is_constant,
    //             Some(Box::new(Stmt::VarDecl(decl.clone()))),
    //         );
    //     }

    //     Ok(var_type)
    // }

    fn check_var_decl(&mut self, decl: &VarDecl) -> Result<Type, RaccoonError> {
        let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());

        // 1. Resuelve el tipo explícito que dio el usuario (si lo hay)
        let explicit_type = resolver.resolve(&decl.type_annotation)?;

        let var_type: Type; // Este será el tipo final de la variable

        if let Some(ref initializer) = decl.initializer {
            // 2. Hay un inicializador, así que revisa su tipo
            let init_type = self.check_expr(initializer)?;

            // 3. Comprueba si el tipo explícito es 'unknown' o 'any'
            //    (asumiendo que 'unknown' es el tipo por defecto si no se anota nada)
            if matches!(explicit_type.kind(), TypeKind::Unknown | TypeKind::Any) {
                // 4. INFERENCIA: El tipo de la variable ES el tipo del inicializador
                var_type = init_type;
            } else {
                // 5. VERIFICACIÓN: Hay un tipo explícito, así que verifica la asignación
                if !init_type.is_assignable_to(&explicit_type) {
                    return Err(RaccoonError::new(
                        format!(
                            "Cannot assign type '{:?}' to variable of type '{:?}'",
                            init_type, explicit_type
                        ),
                        decl.position,
                        self.file.clone(),
                    ));
                }
                var_type = explicit_type;
            }
        } else {
            // No hay inicializador, se debe usar el tipo explícito.
            // Si es 'unknown' o 'any', podría ser un error si el lenguaje no lo permite.
            if matches!(explicit_type.kind(), TypeKind::Unknown) {
                return Err(RaccoonError::new(
                    "Variable must have a type annotation or an initializer",
                    decl.position,
                    self.file.clone(),
                ));
            }
            var_type = explicit_type;
        }

        // 6. Define la variable en la tabla de símbolos con el tipo determinado
        if let VarPattern::Identifier(ref name) = decl.pattern {
            self.symbol_table.define(
                name.clone(),
                SymbolKind::Variable,
                var_type.clone(),
                decl.is_constant,
                Some(Box::new(Stmt::VarDecl(decl.clone()))),
            );
        }

        Ok(var_type)
    }

    fn check_fn_decl(&mut self, decl: &FnDecl) -> Result<Type, RaccoonError> {
        let fn_symbol = self
            .symbol_table
            .lookup(&decl.name)
            .ok_or_else(|| {
                RaccoonError::new(
                    format!("Function '{}' not found", decl.name),
                    decl.position,
                    self.file.clone(),
                )
            })?
            .clone();

        let prev_function = self.current_function.clone();
        let prev_async = self.in_async_function;

        self.current_function = Some(fn_symbol.clone());
        self.in_async_function = decl.is_async;

        self.symbol_table.enter_scope();

        // Resolve all parameter types first, then drop the resolver
        let param_types: Result<Vec<_>, _> = {
            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            decl.parameters
                .iter()
                .map(|param| resolver.resolve(&param.param_type))
                .collect()
        };
        let param_types = param_types?;

        // Now we can mutably borrow symbol_table
        for (param, param_type) in decl.parameters.iter().zip(param_types.iter()) {
            if let VarPattern::Identifier(ref name) = param.pattern {
                self.symbol_table.define(
                    name.clone(),
                    SymbolKind::Parameter,
                    param_type.clone(),
                    false,
                    None,
                );
            }
        }

        // Get the explicit return type if provided
        let explicit_return_type = if let Some(ref ret_type) = decl.return_type {
            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            Some(resolver.resolve(ret_type)?)
        } else {
            None
        };

        // Determine final return type
        let mut final_return_type = if let Some(explicit) = explicit_return_type {
            // If there's an explicit type, use it directly without inferring from body
            // This prevents stack overflow in recursive functions
            explicit
        } else {
            // Only infer return type from body if no explicit type was provided
            self.infer_function_return_type(&decl.body)?
        };

        // Wrap in Future if async and not already a Future
        if decl.is_async && !matches!(final_return_type, Type::Future(_)) {
            final_return_type = Type::Future(Box::new(FutureType {
                inner_type: final_return_type,
            }));
        }

        // Update the function symbol with the inferred type
        let updated_fn_type = FunctionType {
            params: param_types,
            return_type: final_return_type.clone(),
            is_variadic: false,
        };

        self.symbol_table.update_symbol_type(
            &decl.name,
            Type::Function(Box::new(updated_fn_type)),
        )?;

        self.symbol_table.exit_scope();
        self.current_function = prev_function;
        self.in_async_function = prev_async;

        Ok(Type::Function(Box::new(FunctionType {
            params: vec![],
            return_type: final_return_type,
            is_variadic: false,
        })))
    }

    /// Infer the return type of a function by analyzing its body
    fn infer_function_return_type(&mut self, body: &[Stmt]) -> Result<Type, RaccoonError> {
        let mut return_types = Vec::new();

        for stmt in body {
            self.collect_return_types(stmt, &mut return_types)?;
        }

        if return_types.is_empty() {
            // No return statements found
            return Ok(PrimitiveType::void());
        }

        // Find the common type among all return types
        self.type_inference
            .infer_common_type(&return_types, (0, 0))
    }

    /// Recursively collect all return statement types
    fn collect_return_types(
        &mut self,
        stmt: &Stmt,
        return_types: &mut Vec<Type>,
    ) -> Result<(), RaccoonError> {
        match stmt {
            Stmt::ReturnStmt(ret) => {
                if let Some(ref value) = ret.value {
                    let value_type = self.check_expr(value)?;
                    return_types.push(value_type);
                } else {
                    return_types.push(PrimitiveType::void());
                }
            }
            Stmt::Block(block) => {
                for s in &block.statements {
                    self.collect_return_types(s, return_types)?;
                }
            }
            Stmt::IfStmt(if_stmt) => {
                self.collect_return_types(&if_stmt.then_branch, return_types)?;
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.collect_return_types(else_branch, return_types)?;
                }
            }
            Stmt::WhileStmt(while_stmt) => {
                self.collect_return_types(&while_stmt.body, return_types)?;
            }
            Stmt::ForStmt(for_stmt) => {
                self.collect_return_types(&for_stmt.body, return_types)?;
            }
            Stmt::ForInStmt(for_in) => {
                self.collect_return_types(&for_in.body, return_types)?;
            }
            Stmt::TryStmt(try_stmt) => {
                for s in &try_stmt.try_block.statements {
                    self.collect_return_types(s, return_types)?;
                }
                for catch in &try_stmt.catch_clauses {
                    for s in &catch.body.statements {
                        self.collect_return_types(s, return_types)?;
                    }
                }
                if let Some(ref finally) = try_stmt.finally_block {
                    for s in &finally.statements {
                        self.collect_return_types(s, return_types)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn check_class_decl(&mut self, decl: &ClassDecl) -> Result<Type, RaccoonError> {
        let class_symbol = self
            .symbol_table
            .lookup(&decl.name)
            .ok_or_else(|| {
                RaccoonError::new(
                    format!("Class '{}' not found", decl.name),
                    decl.position,
                    self.file.clone(),
                )
            })?
            .clone();

        let prev_class = self.current_class.clone();
        self.current_class = Some(class_symbol.clone());

        self.symbol_table.enter_scope();

        if let Type::Class(ref class_type) = class_symbol.symbol_type {
            self.symbol_table.define(
                "this".to_string(),
                SymbolKind::Variable,
                Type::Class(class_type.clone()),
                false,
                None,
            );
        }

        for prop in &decl.properties {
            if let Some(ref initializer) = prop.initializer {
                let init_type = self.check_expr(initializer)?;
                let prop_type = {
                    let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
                    resolver.resolve(&prop.property_type)?
                };

                if !init_type.is_assignable_to(&prop_type) {
                    return Err(RaccoonError::new(
                        format!(
                            "Property '{}' initializer type '{:?}' not assignable to '{:?}'",
                            prop.name, init_type, prop_type
                        ),
                        decl.position,
                        self.file.clone(),
                    ));
                }
            }
        }

        self.symbol_table.exit_scope();
        self.current_class = prev_class;

        Ok(PrimitiveType::void())
    }

    fn check_export_decl(&mut self, decl: &ExportDecl) -> Result<Type, RaccoonError> {
        if let Some(ref declaration) = decl.declaration {
            return self.check_stmt(declaration);
        }

        for spec in &decl.specifiers {
            let symbol = self.symbol_table.lookup(&spec.local);
            if symbol.is_none() {
                return Err(RaccoonError::new(
                    format!("Cannot export '{}': not found", spec.local),
                    decl.position,
                    self.file.clone(),
                ));
            }
        }

        Ok(PrimitiveType::void())
    }
    fn check_block(&mut self, block: &Block) -> Result<Type, RaccoonError> {
        self.symbol_table.enter_scope();

        for stmt in &block.statements {
            self.check_stmt(stmt)?;
        }

        self.symbol_table.exit_scope();
        Ok(PrimitiveType::void())
    }

    fn check_if_stmt(&mut self, stmt: &IfStmt) -> Result<Type, RaccoonError> {
        let cond_type = self.check_expr(&stmt.condition)?;

        if !matches!(cond_type.kind(), TypeKind::Bool) {
            return Err(RaccoonError::new(
                format!("If condition must be boolean, got '{:?}'", cond_type),
                stmt.position,
                self.file.clone(),
            ));
        }

        // Analyze type narrowing from condition
        let narrowing_info = self
            .type_inference
            .analyze_type_narrowing(&stmt.condition, &self.symbol_table)?;

        // Apply narrowing to then branch
        self.type_inference.push_narrowing_scope();
        for (name, ty) in narrowing_info.then_narrows {
            self.type_inference.set_narrowed_type(name, ty);
        }
        self.check_stmt(&stmt.then_branch)?;
        self.type_inference.pop_narrowing_scope();

        // Apply narrowing to else branch
        if let Some(ref else_branch) = stmt.else_branch {
            self.type_inference.push_narrowing_scope();
            for (name, ty) in narrowing_info.else_narrows {
                self.type_inference.set_narrowed_type(name, ty);
            }
            self.check_stmt(else_branch)?;
            self.type_inference.pop_narrowing_scope();
        }

        Ok(PrimitiveType::void())
    }

    fn check_while_stmt(&mut self, stmt: &WhileStmt) -> Result<Type, RaccoonError> {
        let cond_type = self.check_expr(&stmt.condition)?;

        if !matches!(cond_type.kind(), TypeKind::Bool) {
            return Err(RaccoonError::new(
                format!("While condition must be boolean, got '{:?}'", cond_type),
                stmt.position,
                self.file.clone(),
            ));
        }

        let prev_in_loop = self.in_loop;
        self.in_loop = true;

        self.check_stmt(&stmt.body)?;

        self.in_loop = prev_in_loop;

        Ok(PrimitiveType::void())
    }

    fn check_for_stmt(&mut self, stmt: &ForStmt) -> Result<Type, RaccoonError> {
        self.symbol_table.enter_scope();

        if let Some(ref initializer) = stmt.initializer {
            self.check_stmt(initializer)?;
        }

        if let Some(ref condition) = stmt.condition {
            let cond_type = self.check_expr(condition)?;
            if !matches!(cond_type.kind(), TypeKind::Bool) {
                return Err(RaccoonError::new(
                    format!("For condition must be boolean, got '{:?}'", cond_type),
                    stmt.position,
                    self.file.clone(),
                ));
            }
        }

        if let Some(ref increment) = stmt.increment {
            self.check_expr(increment)?;
        }

        let prev_in_loop = self.in_loop;
        self.in_loop = true;

        self.check_stmt(&stmt.body)?;

        self.in_loop = prev_in_loop;
        self.symbol_table.exit_scope();

        Ok(PrimitiveType::void())
    }

    fn check_for_in_stmt(&mut self, stmt: &ForInStmt) -> Result<Type, RaccoonError> {
        let iterable_type = self.check_expr(&stmt.iterable)?;

        let element_type = if let Type::List(ref list_type) = iterable_type {
            list_type.element_type.clone()
        } else if matches!(iterable_type.kind(), TypeKind::Str) {
            PrimitiveType::str()
        } else {
            return Err(RaccoonError::new(
                format!("Cannot iterate over type '{:?}'", iterable_type),
                stmt.position,
                self.file.clone(),
            ));
        };

        self.symbol_table.enter_scope();

        self.symbol_table.define(
            stmt.variable.clone(),
            SymbolKind::Variable,
            element_type,
            false,
            None,
        );

        let prev_in_loop = self.in_loop;
        self.in_loop = true;

        self.check_stmt(&stmt.body)?;

        self.in_loop = prev_in_loop;
        self.symbol_table.exit_scope();

        Ok(PrimitiveType::void())
    }

    fn check_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<Type, RaccoonError> {
        if self.current_function.is_none() {
            return Err(RaccoonError::new(
                "Return statement outside function",
                stmt.position,
                self.file.clone(),
            ));
        }

        if let Some(ref value) = stmt.value {
            let value_type = self.check_expr(value)?;
            return Ok(value_type);
        }

        Ok(PrimitiveType::void())
    }

    fn check_break_stmt(&self) -> Result<Type, RaccoonError> {
        if !self.in_loop {
            return Err(RaccoonError::new(
                "Break statement outside loop",
                (0, 0),
                self.file.clone(),
            ));
        }
        Ok(PrimitiveType::void())
    }

    fn check_continue_stmt(&self) -> Result<Type, RaccoonError> {
        if !self.in_loop {
            return Err(RaccoonError::new(
                "Continue statement outside loop",
                (0, 0),
                self.file.clone(),
            ));
        }
        Ok(PrimitiveType::void())
    }

    fn check_try_stmt(&mut self, stmt: &TryStmt) -> Result<Type, RaccoonError> {
        self.check_block(&stmt.try_block)?;

        for catch_clause in &stmt.catch_clauses {
            self.symbol_table.enter_scope();

            let error_type = if let Some(ref error_type) = catch_clause.error_type {
                let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
                resolver.resolve(error_type)?
            } else {
                PrimitiveType::any()
            };

            self.symbol_table.define(
                catch_clause.error_var.clone(),
                SymbolKind::Variable,
                error_type,
                false,
                None,
            );

            self.check_block(&catch_clause.body)?;
            self.symbol_table.exit_scope();
        }

        if let Some(ref finally_block) = stmt.finally_block {
            self.check_block(finally_block)?;
        }

        Ok(PrimitiveType::void())
    }

    fn check_throw_stmt(&mut self, stmt: &ThrowStmt) -> Result<Type, RaccoonError> {
        self.check_expr(&stmt.value)?;
        Ok(PrimitiveType::void())
    }

    fn check_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Type, RaccoonError> {
        let left_type = self.check_expr(&expr.left)?;
        let right_type = self.check_expr(&expr.right)?;

        if !self
            .type_checker
            .are_types_compatible(&left_type, &right_type, expr.operator)
        {
            return Err(RaccoonError::new(
                format!(
                    "Cannot apply operator {:?} to types '{:?}' and '{:?}'",
                    expr.operator, left_type, right_type
                ),
                expr.position,
                self.file.clone(),
            ));
        }

        self.type_checker
            .infer_binary_type(expr.operator, &left_type, &right_type)
    }

    fn check_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Type, RaccoonError> {
        let operand_type = self.check_expr(&expr.operand)?;
        self.type_checker
            .infer_unary_type(expr.operator, &operand_type, expr.position)
    }

    fn check_call_expr(&mut self, expr: &CallExpr) -> Result<Type, RaccoonError> {
        let callee_type = self.check_expr(&expr.callee)?;

        if let Type::Function(fn_type) = callee_type {
            if expr.args.len() != fn_type.params.len() {
                return Err(RaccoonError::new(
                    format!(
                        "Function expects {} arguments, got {}",
                        fn_type.params.len(),
                        expr.args.len()
                    ),
                    expr.position,
                    self.file.clone(),
                ));
            }

            for (i, arg) in expr.args.iter().enumerate() {
                let arg_type = self.check_expr(arg)?;
                if !arg_type.is_assignable_to(&fn_type.params[i]) {
                    return Err(RaccoonError::new(
                        format!(
                            "Argument {}: type '{:?}' not assignable to '{:?}'",
                            i + 1,
                            arg_type,
                            fn_type.params[i]
                        ),
                        expr.position,
                        self.file.clone(),
                    ));
                }
            }

            return Ok(fn_type.return_type);
        }

        Err(RaccoonError::new(
            format!("Cannot call non-function type '{:?}'", callee_type),
            expr.position,
            self.file.clone(),
        ))
    }

    fn check_new_expr(&mut self, expr: &NewExpr) -> Result<Type, RaccoonError> {
        if expr.class_name == "Map" {
            if expr.type_args.len() != 2 {
                return Err(RaccoonError::new(
                    "Map requires exactly two type arguments",
                    expr.position,
                    self.file.clone(),
                ));
            }

            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            let key_type = resolver.resolve(&expr.type_args[0])?;
            let value_type = resolver.resolve(&expr.type_args[1])?;

            return Ok(Type::Map(Box::new(MapType {
                key_type,
                value_type,
            })));
        }

        let class_symbol = self.symbol_table.lookup(&expr.class_name).ok_or_else(|| {
            RaccoonError::new(
                format!("Class '{}' not found", expr.class_name),
                expr.position,
                self.file.clone(),
            )
        })?;

        if class_symbol.kind != SymbolKind::Class {
            return Err(RaccoonError::new(
                format!("'{}' is not a class", expr.class_name),
                expr.position,
                self.file.clone(),
            ));
        }

        Ok(class_symbol.symbol_type.clone())
    }

    fn check_member_expr(&mut self, expr: &MemberExpr) -> Result<Type, RaccoonError> {
        let object_type = self.check_expr(&expr.object)?;

        if let Type::Class(ref class_type) = object_type {
            if let Some(prop_info) = class_type.properties.get(&expr.property) {
                return Ok(prop_info.property_type.clone());
            }

            return Err(RaccoonError::new(
                format!(
                    "Property '{}' does not exist on class '{}'",
                    expr.property, class_type.name
                ),
                expr.position,
                self.file.clone(),
            ));
        }

        if let Type::Interface(ref interface_type) = object_type {
            if let Some(prop) = interface_type.properties.get(&expr.property) {
                return Ok(prop.property_type.clone());
            }
        }

        if matches!(object_type.kind(), TypeKind::Str) {
            if expr.property == "length" {
                return Ok(PrimitiveType::int());
            }
        }

        if let Type::List(_) = object_type {
            if expr.property == "length" {
                return Ok(PrimitiveType::int());
            }
        }

        Err(RaccoonError::new(
            format!(
                "Property '{}' does not exist on type '{:?}'",
                expr.property, object_type
            ),
            expr.position,
            self.file.clone(),
        ))
    }

    fn check_method_call_expr(&mut self, expr: &MethodCallExpr) -> Result<Type, RaccoonError> {
        let object_type = self.check_expr(&expr.object)?;

        if let Type::Class(ref class_type) = object_type {
            if let Some(method_info) = class_type.methods.get(&expr.method) {
                if expr.args.len() != method_info.method_type.params.len() {
                    return Err(RaccoonError::new(
                        format!(
                            "Method '{}' expects {} arguments, got {}",
                            expr.method,
                            method_info.method_type.params.len(),
                            expr.args.len()
                        ),
                        expr.position,
                        self.file.clone(),
                    ));
                }

                for (i, arg) in expr.args.iter().enumerate() {
                    let arg_type = self.check_expr(arg)?;
                    if !arg_type.is_assignable_to(&method_info.method_type.params[i]) {
                        return Err(RaccoonError::new(
                            format!(
                                "Argument {}: type '{:?}' not assignable to '{:?}'",
                                i + 1,
                                arg_type,
                                method_info.method_type.params[i]
                            ),
                            expr.position,
                            self.file.clone(),
                        ));
                    }
                }

                return Ok(method_info.method_type.return_type.clone());
            }
        }

        if matches!(object_type.kind(), TypeKind::Str) {
            if expr.method == "toUpper" || expr.method == "toLower" {
                return Ok(PrimitiveType::str());
            }
        }

        Err(RaccoonError::new(
            format!(
                "Method '{}' does not exist on type '{:?}'",
                expr.method, object_type
            ),
            expr.position,
            self.file.clone(),
        ))
    }

    fn check_index_expr(&mut self, expr: &IndexExpr) -> Result<Type, RaccoonError> {
        let object_type = self.check_expr(&expr.object)?;
        let index_type = self.check_expr(&expr.index)?;

        self.type_checker
            .validate_index_expr(&object_type, &index_type, expr.position)
    }

    fn check_await_expr(&mut self, expr: &AwaitExpr) -> Result<Type, RaccoonError> {
        let expr_type = self.check_expr(&expr.expression)?;

        if let Type::Future(future_type) = expr_type {
            return Ok(future_type.inner_type);
        }

        Err(RaccoonError::new(
            format!(
                "Cannot await non-Future type '{:?}'. Expected Future<T>",
                expr_type
            ),
            expr.position,
            self.file.clone(),
        ))
    }

    fn check_this_expr(&self) -> Result<Type, RaccoonError> {
        if let Some(ref current_class) = self.current_class {
            return Ok(current_class.symbol_type.clone());
        }

        Err(RaccoonError::new(
            "Cannot use 'this' outside of class",
            (0, 0),
            self.file.clone(),
        ))
    }

    fn check_super_expr(&self) -> Result<Type, RaccoonError> {
        if let Some(ref current_class) = self.current_class {
            if let Type::Class(ref class_type) = current_class.symbol_type {
                if let Some(ref superclass) = class_type.superclass {
                    return Ok(Type::Class(superclass.clone()));
                }
                return Err(RaccoonError::new(
                    "Cannot use 'super' in class without superclass",
                    (0, 0),
                    self.file.clone(),
                ));
            }
        }

        Err(RaccoonError::new(
            "Cannot use 'super' outside of class",
            (0, 0),
            self.file.clone(),
        ))
    }

    fn check_instanceof_expr(&mut self, expr: &InstanceOfExpr) -> Result<Type, RaccoonError> {
        self.check_expr(&expr.operand)?;

        let type_symbol = self.symbol_table.lookup(&expr.type_name);
        if type_symbol.is_none() || type_symbol.as_ref().unwrap().kind != SymbolKind::Class {
            return Err(RaccoonError::new(
                format!("'{}' is not a class", expr.type_name),
                expr.position,
                self.file.clone(),
            ));
        }

        Ok(PrimitiveType::bool())
    }

    fn check_arrow_fn_expr(&mut self, expr: &ArrowFnExpr) -> Result<Type, RaccoonError> {
        self.symbol_table.enter_scope();

        // Resolve all parameter types first, then drop the resolver
        let param_types: Result<Vec<_>, _> = {
            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            expr.parameters
                .iter()
                .map(|param| resolver.resolve(&param.param_type))
                .collect()
        };
        let param_types = param_types?;

        // Now we can mutably borrow symbol_table
        for (param, param_type) in expr.parameters.iter().zip(param_types.iter()) {
            if let VarPattern::Identifier(ref name) = param.pattern {
                self.symbol_table.define(
                    name.clone(),
                    SymbolKind::Parameter,
                    param_type.clone(),
                    false,
                    None,
                );
            }
        }

        // Infer return type from body
        let inferred_return_type = match &expr.body {
            ArrowFnBody::Expr(body_expr) => self.check_expr(body_expr)?,
            ArrowFnBody::Block(stmts) => {
                let mut last_return_type = PrimitiveType::void();
                for stmt in stmts {
                    if let Stmt::ReturnStmt(ret) = stmt {
                        if let Some(ref value) = ret.value {
                            last_return_type = self.check_expr(value)?;
                        }
                    } else {
                        self.check_stmt(stmt)?;
                    }
                }
                last_return_type
            }
        };

        // Use explicit return type if provided, otherwise use inferred
        let return_type = if let Some(ref explicit_type) = expr.return_type {
            let resolver = TypeResolver::new(&self.symbol_table, self.file.clone());
            let resolved_type = resolver.resolve(explicit_type)?;

            // Verify that inferred type is assignable to explicit type
            if !inferred_return_type.is_assignable_to(&resolved_type) {
                self.symbol_table.exit_scope();
                return Err(RaccoonError::new(
                    format!(
                        "Function body returns '{:?}' but declared return type is '{:?}'",
                        inferred_return_type, resolved_type
                    ),
                    expr.position,
                    self.file.clone(),
                ));
            }
            resolved_type
        } else {
            inferred_return_type
        };

        self.symbol_table.exit_scope();

        Ok(Type::Function(Box::new(FunctionType {
            params: param_types,
            return_type,
            is_variadic: false,
        })))
    }
    fn check_identifier(&mut self, identifier: &Identifier) -> Result<Type, RaccoonError> {
        // Check if there's a narrowed type first
        if let Some(narrowed_type) = self.type_inference.get_narrowed_type(&identifier.name) {
            return Ok(narrowed_type);
        }

        let symbol = self.symbol_table.lookup(&identifier.name).ok_or_else(|| {
            RaccoonError::new(
                format!("Undefined variable '{}'", identifier.name),
                identifier.position,
                self.file.clone(),
            )
        })?;

        Ok(symbol.symbol_type.clone())
    }

    fn check_assignment(&mut self, assignment: &Assignment) -> Result<Type, RaccoonError> {
        let target_type = self.check_expr(&assignment.target)?;
        let value_type = self.check_expr(&assignment.value)?;

        self.type_checker.validate_assignment(
            &target_type,
            &value_type,
            assignment.operator,
            assignment.position,
        )
    }

    fn check_range_expr(&mut self, expr: &RangeExpr) -> Result<Type, RaccoonError> {
        let start_type = self.check_expr(&expr.start)?;
        let end_type = self.check_expr(&expr.end)?;

        self.type_checker
            .validate_range_expr(&start_type, &end_type, expr.position)
    }

    fn check_conditional_expr(&mut self, expr: &ConditionalExpr) -> Result<Type, RaccoonError> {
        let cond_type = self.check_expr(&expr.condition)?;

        if !matches!(cond_type.kind(), TypeKind::Bool) {
            return Err(RaccoonError::new(
                "Conditional expression condition must be boolean",
                expr.position,
                self.file.clone(),
            ));
        }

        let then_type = self.check_expr(&expr.then_expr)?;
        let else_type = self.check_expr(&expr.else_expr)?;

        if then_type.equals(&else_type) {
            return Ok(then_type);
        }

        Ok(Type::Union(Box::new(UnionType::new(vec![
            then_type, else_type,
        ]))))
    }

    fn check_null_coalescing_expr(
        &mut self,
        expr: &NullCoalescingExpr,
    ) -> Result<Type, RaccoonError> {
        let left_type = self.check_expr(&expr.left)?;
        let right_type = self.check_expr(&expr.right)?;

        if let Type::Nullable(_nullable) = left_type {
            return Ok(right_type);
        }

        Ok(left_type)
    }

    fn check_optional_chaining_expr(
        &mut self,
        expr: &OptionalChainingExpr,
    ) -> Result<Type, RaccoonError> {
        let object_type = self.check_expr(&expr.object)?;

        if let Type::Nullable(nullable) = object_type {
            let unwrapped = &nullable.inner_type;

            if let Type::Interface(interface_type) = unwrapped {
                if let Some(prop) = interface_type.properties.get(&expr.property) {
                    return Ok(Type::Nullable(Box::new(NullableType {
                        inner_type: prop.property_type.clone(),
                    })));
                }
            }

            if let Type::Class(class_type) = unwrapped {
                if let Some(prop) = class_type.properties.get(&expr.property) {
                    return Ok(Type::Nullable(Box::new(NullableType {
                        inner_type: prop.property_type.clone(),
                    })));
                }
            }
        }

        Ok(Type::Nullable(Box::new(NullableType {
            inner_type: PrimitiveType::any(),
        })))
    }

    fn check_null_assertion_expr(
        &mut self,
        expr: &NullAssertionExpr,
    ) -> Result<Type, RaccoonError> {
        let operand_type = self.check_expr(&expr.operand)?;

        if let Type::Nullable(nullable) = operand_type {
            return Ok(nullable.inner_type);
        }

        Ok(operand_type)
    }

    fn check_unary_update_expr(&mut self, expr: &UnaryUpdateExpr) -> Result<Type, RaccoonError> {
        let operand_type = self.check_expr(&expr.operand)?;

        if !self.type_checker.is_numeric_type(&operand_type) {
            return Err(RaccoonError::new(
                "Increment/decrement requires numeric type",
                expr.position,
                self.file.clone(),
            ));
        }

        Ok(operand_type)
    }

    fn check_list_literal(&mut self, list: &ListLiteral) -> Result<Type, RaccoonError> {
        if list.elements.is_empty() {
            return Ok(Type::List(Box::new(ListType {
                element_type: PrimitiveType::unknown(),
            })));
        }

        // Collect all element types
        let mut element_types = Vec::new();
        for element in &list.elements {
            element_types.push(self.check_expr(element)?);
        }

        // Use the improved type inference engine
        let common_type = self
            .type_inference
            .infer_common_type(&element_types, list.position)?;

        Ok(Type::List(Box::new(ListType {
            element_type: common_type,
        })))
    }

    fn check_object_literal(&mut self, obj: &ObjectLiteral) -> Result<Type, RaccoonError> {
        let mut properties = HashMap::new();

        for (key, value) in &obj.properties {
            let value_type = self.check_expr(value)?;
            properties.insert(
                key.clone(),
                InterfaceProperty {
                    property_type: value_type,
                    optional: false,
                },
            );
        }

        Ok(Type::Interface(Box::new(InterfaceType {
            name: "anonymous".to_string(),
            properties,
            type_parameters: Vec::new(),
        })))
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new(Option::None)
    }
}
