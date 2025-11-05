use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    type_system::TypeResolver,
};

use super::SemanticAnalyzer;

pub fn check_stmt(analyzer: &mut SemanticAnalyzer, stmt: &Stmt) -> Result<Type, RaccoonError> {
    match stmt {
        Stmt::Program(program) => {
            for s in &program.stmts {
                analyzer.check_stmt(s)?;
            }
            Ok(PrimitiveType::void())
        }

        Stmt::VarDecl(decl) => analyzer.check_var_decl(decl),
        Stmt::FnDecl(decl) => analyzer.check_fn_decl(decl),
        Stmt::ClassDecl(decl) => analyzer.check_class_decl(decl),
        Stmt::InterfaceDecl(_) => Ok(PrimitiveType::void()),
        Stmt::EnumDecl(_) => Ok(PrimitiveType::void()),
        Stmt::TypeAliasDecl(_) => Ok(PrimitiveType::void()),
        Stmt::ImportDecl(_) => Ok(PrimitiveType::void()),
        Stmt::ExportDecl(decl) => check_export_decl(analyzer, decl),
        Stmt::Block(block) => check_block(analyzer, block),
        Stmt::IfStmt(stmt) => analyzer.check_if_stmt(stmt),
        Stmt::WhileStmt(stmt) => analyzer.check_while_stmt(stmt),
        Stmt::DoWhileStmt(stmt) => analyzer.check_do_while_stmt(stmt),
        Stmt::ForStmt(stmt) => analyzer.check_for_stmt(stmt),
        Stmt::ForInStmt(stmt) => analyzer.check_for_in_stmt(stmt),
        Stmt::ForOfStmt(stmt) => analyzer.check_for_of_stmt(stmt),
        Stmt::SwitchStmt(stmt) => analyzer.check_switch_stmt(stmt),
        Stmt::ReturnStmt(stmt) => check_return_stmt(analyzer, stmt),
        Stmt::BreakStmt(_) => check_break_stmt(analyzer),
        Stmt::ContinueStmt(_) => check_continue_stmt(analyzer),
        Stmt::ExprStmt(stmt) => analyzer.check_expr(&stmt.expression),
        Stmt::TryStmt(stmt) => check_try_stmt(analyzer, stmt),
        Stmt::ThrowStmt(stmt) => check_throw_stmt(analyzer, stmt),
    }
}

pub fn check_export_decl(analyzer: &mut SemanticAnalyzer, decl: &ExportDecl) -> Result<Type, RaccoonError> {
    if let Some(ref declaration) = decl.declaration {
        return analyzer.check_stmt(declaration);
    }

    for spec in &decl.specifiers {
        let symbol = analyzer.symbol_table.lookup(&spec.local);
        if symbol.is_none() {
            return Err(RaccoonError::new(
                format!("Cannot export '{}': not found", spec.local),
                decl.position,
                analyzer.file.clone(),
            ));
        }
    }

    Ok(PrimitiveType::void())
}

pub fn check_block(analyzer: &mut SemanticAnalyzer, block: &Block) -> Result<Type, RaccoonError> {
    analyzer.symbol_table.enter_scope();

    for stmt in &block.statements {
        analyzer.check_stmt(stmt)?;
    }

    analyzer.symbol_table.exit_scope();
    Ok(PrimitiveType::void())
}

pub fn check_return_stmt(analyzer: &mut SemanticAnalyzer, stmt: &ReturnStmt) -> Result<Type, RaccoonError> {
    if analyzer.current_function.is_none() {
        return Err(RaccoonError::new(
            "Return statement outside function",
            stmt.position,
            analyzer.file.clone(),
        ));
    }

    if let Some(ref value) = stmt.value {
        let value_type = analyzer.check_expr(value)?;
        return Ok(value_type);
    }

    Ok(PrimitiveType::void())
}

pub fn check_break_stmt(analyzer: &SemanticAnalyzer) -> Result<Type, RaccoonError> {
    if !analyzer.in_loop {
        return Err(RaccoonError::new(
            "Break statement outside loop",
            (0, 0),
            analyzer.file.clone(),
        ));
    }
    Ok(PrimitiveType::void())
}

pub fn check_continue_stmt(analyzer: &SemanticAnalyzer) -> Result<Type, RaccoonError> {
    if !analyzer.in_loop {
        return Err(RaccoonError::new(
            "Continue statement outside loop",
            (0, 0),
            analyzer.file.clone(),
        ));
    }
    Ok(PrimitiveType::void())
}

pub fn check_try_stmt(analyzer: &mut SemanticAnalyzer, stmt: &TryStmt) -> Result<Type, RaccoonError> {
    check_block(analyzer, &stmt.try_block)?;

    for catch_clause in &stmt.catch_clauses {
        analyzer.symbol_table.enter_scope();

        let error_type = if let Some(ref error_type) = catch_clause.error_type {
            let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
            resolver.resolve(error_type)?
        } else {
            PrimitiveType::any()
        };

        analyzer.symbol_table.define(
            catch_clause.error_var.clone(),
            crate::symbol_table::SymbolKind::Variable,
            error_type,
            false,
            None,
        );

        check_block(analyzer, &catch_clause.body)?;
        analyzer.symbol_table.exit_scope();
    }

    if let Some(ref finally_block) = stmt.finally_block {
        check_block(analyzer, finally_block)?;
    }

    Ok(PrimitiveType::void())
}

pub fn check_throw_stmt(analyzer: &mut SemanticAnalyzer, stmt: &ThrowStmt) -> Result<Type, RaccoonError> {
    analyzer.check_expr(&stmt.value)?;
    Ok(PrimitiveType::void())
}
