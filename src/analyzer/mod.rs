use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::{SymbolItem, SymbolTable},
    type_system::{TypeChecker, TypeInferenceEngine},
};

pub mod control_flow;
pub mod declarations;
pub mod expressions;
pub mod statements;
pub mod types;

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
                Stmt::ClassDecl(decl) => declarations::register_class(self, decl)?,
                Stmt::InterfaceDecl(decl) => declarations::register_interface(self, decl)?,
                Stmt::EnumDecl(decl) => declarations::register_enum(self, decl)?,
                Stmt::TypeAliasDecl(decl) => declarations::register_type_alias(self, decl)?,
                Stmt::FnDecl(decl) => declarations::register_function(self, decl)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn second_pass(&mut self, program: &Program) -> Result<(), RaccoonError> {
        for stmt in &program.stmts {
            statements::check_stmt(self, stmt)?;
        }
        Ok(())
    }

    // Private methods for inter-module communication within the analyzer
    pub(in crate::analyzer) fn check_stmt(&mut self, stmt: &Stmt) -> Result<Type, RaccoonError> {
        statements::check_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_expr(&mut self, expr: &Expr) -> Result<Type, RaccoonError> {
        expressions::check_expr(self, expr)
    }

    pub(in crate::analyzer) fn check_var_decl(
        &mut self,
        decl: &VarDecl,
    ) -> Result<Type, RaccoonError> {
        declarations::check_var_decl(self, decl)
    }

    pub(in crate::analyzer) fn check_fn_decl(
        &mut self,
        decl: &FnDecl,
    ) -> Result<Type, RaccoonError> {
        declarations::check_fn_decl(self, decl)
    }

    pub(in crate::analyzer) fn check_class_decl(
        &mut self,
        decl: &ClassDecl,
    ) -> Result<Type, RaccoonError> {
        declarations::check_class_decl(self, decl)
    }

    pub(in crate::analyzer) fn check_if_stmt(
        &mut self,
        stmt: &IfStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_if_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_while_stmt(
        &mut self,
        stmt: &WhileStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_while_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_do_while_stmt(
        &mut self,
        stmt: &DoWhileStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_do_while_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_for_stmt(
        &mut self,
        stmt: &ForStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_for_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_for_in_stmt(
        &mut self,
        stmt: &ForInStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_for_in_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_for_of_stmt(
        &mut self,
        stmt: &ForOfStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_for_of_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn check_switch_stmt(
        &mut self,
        stmt: &SwitchStmt,
    ) -> Result<Type, RaccoonError> {
        control_flow::check_switch_stmt(self, stmt)
    }

    pub(in crate::analyzer) fn infer_function_return_type(
        &mut self,
        body: &[Stmt],
    ) -> Result<Type, RaccoonError> {
        types::infer_function_return_type(self, body)
    }

    pub(in crate::analyzer) fn collect_return_types(
        &mut self,
        stmt: &Stmt,
        return_types: &mut Vec<Type>,
    ) -> Result<(), RaccoonError> {
        types::collect_return_types(self, stmt, return_types)
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new(Option::None)
    }
}
