use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
};

use super::SemanticAnalyzer;

pub fn infer_function_return_type(analyzer: &mut SemanticAnalyzer, body: &[Stmt]) -> Result<Type, RaccoonError> {
    let mut return_types = Vec::new();

    for stmt in body {
        analyzer.collect_return_types(stmt, &mut return_types)?;
    }

    if return_types.is_empty() {
        return Ok(PrimitiveType::void());
    }

    analyzer.type_inference.infer_common_type(&return_types, (0, 0))
}

pub fn collect_return_types(
    analyzer: &mut SemanticAnalyzer,
    stmt: &Stmt,
    return_types: &mut Vec<Type>,
) -> Result<(), RaccoonError> {
    match stmt {
        Stmt::ReturnStmt(ret) => {
            if let Some(ref value) = ret.value {
                let value_type = analyzer.check_expr(value)?;
                return_types.push(value_type);
            } else {
                return_types.push(PrimitiveType::void());
            }
        }
        Stmt::Block(block) => {
            for s in &block.statements {
                analyzer.collect_return_types(s, return_types)?;
            }
        }
        Stmt::IfStmt(if_stmt) => {
            analyzer.collect_return_types(&if_stmt.then_branch, return_types)?;
            if let Some(ref else_branch) = if_stmt.else_branch {
                analyzer.collect_return_types(else_branch, return_types)?;
            }
        }
        Stmt::WhileStmt(while_stmt) => {
            analyzer.collect_return_types(&while_stmt.body, return_types)?;
        }
        Stmt::ForStmt(for_stmt) => {
            analyzer.collect_return_types(&for_stmt.body, return_types)?;
        }
        Stmt::ForInStmt(for_in) => {
            analyzer.collect_return_types(&for_in.body, return_types)?;
        }
        Stmt::TryStmt(try_stmt) => {
            for s in &try_stmt.try_block.statements {
                analyzer.collect_return_types(s, return_types)?;
            }
            for catch in &try_stmt.catch_clauses {
                for s in &catch.body.statements {
                    analyzer.collect_return_types(s, return_types)?;
                }
            }
            if let Some(ref finally) = try_stmt.finally_block {
                for s in &finally.statements {
                    analyzer.collect_return_types(s, return_types)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
