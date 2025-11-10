use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::SymbolKind,
};

use super::SemanticAnalyzer;

pub fn check_if_stmt(analyzer: &mut SemanticAnalyzer, stmt: &IfStmt) -> Result<Type, RaccoonError> {
    let cond_type = analyzer.check_expr(&stmt.condition)?;

    if !matches!(cond_type.kind(), TypeKind::Bool) {
        return Err(RaccoonError::new(
            format!("If condition must be boolean, got '{:?}'", cond_type),
            stmt.position,
            analyzer.file.clone(),
        ));
    }

    let narrowing_info = analyzer
        .type_inference
        .analyze_type_narrowing(&stmt.condition, &analyzer.symbol_table)?;

    analyzer.type_inference.push_narrowing_scope();
    for (name, ty) in narrowing_info.then_narrows {
        analyzer.type_inference.set_narrowed_type(name, ty);
    }
    analyzer.check_stmt(&stmt.then_branch)?;
    analyzer.type_inference.pop_narrowing_scope();

    if let Some(ref else_branch) = stmt.else_branch {
        analyzer.type_inference.push_narrowing_scope();
        for (name, ty) in narrowing_info.else_narrows {
            analyzer.type_inference.set_narrowed_type(name, ty);
        }
        analyzer.check_stmt(else_branch)?;
        analyzer.type_inference.pop_narrowing_scope();
    }

    Ok(PrimitiveType::void())
}

pub fn check_while_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &WhileStmt,
) -> Result<Type, RaccoonError> {
    let cond_type = analyzer.check_expr(&stmt.condition)?;

    if !matches!(cond_type.kind(), TypeKind::Bool) {
        return Err(RaccoonError::new(
            format!("While condition must be boolean, got '{:?}'", cond_type),
            stmt.position,
            analyzer.file.clone(),
        ));
    }

    let prev_in_loop = analyzer.in_loop;
    analyzer.in_loop = true;

    analyzer.check_stmt(&stmt.body)?;

    analyzer.in_loop = prev_in_loop;

    Ok(PrimitiveType::void())
}

pub fn check_do_while_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &DoWhileStmt,
) -> Result<Type, RaccoonError> {
    let prev_in_loop = analyzer.in_loop;
    analyzer.in_loop = true;

    analyzer.check_stmt(&stmt.body)?;

    analyzer.in_loop = prev_in_loop;

    let cond_type = analyzer.check_expr(&stmt.condition)?;

    if !matches!(cond_type.kind(), TypeKind::Bool) {
        return Err(RaccoonError::new(
            format!("Do-while condition must be boolean, got '{:?}'", cond_type),
            stmt.position,
            analyzer.file.clone(),
        ));
    }

    Ok(PrimitiveType::void())
}

pub fn check_for_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &ForStmt,
) -> Result<Type, RaccoonError> {
    analyzer.symbol_table.enter_scope();

    if let Some(ref initializer) = stmt.initializer {
        analyzer.check_stmt(initializer)?;
    }

    if let Some(ref condition) = stmt.condition {
        let cond_type = analyzer.check_expr(condition)?;
        if !matches!(cond_type.kind(), TypeKind::Bool) {
            return Err(RaccoonError::new(
                format!("For condition must be boolean, got '{:?}'", cond_type),
                stmt.position,
                analyzer.file.clone(),
            ));
        }
    }

    if let Some(ref increment) = stmt.increment {
        analyzer.check_expr(increment)?;
    }

    let prev_in_loop = analyzer.in_loop;
    analyzer.in_loop = true;

    analyzer.check_stmt(&stmt.body)?;

    analyzer.in_loop = prev_in_loop;
    analyzer.symbol_table.exit_scope();

    Ok(PrimitiveType::void())
}

pub fn check_for_in_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &ForInStmt,
) -> Result<Type, RaccoonError> {
    let iterable_type = analyzer.check_expr(&stmt.iterable)?;

    let element_type = if let Type::Array(ref list_type) = iterable_type {
        list_type.element_type.clone()
    } else if matches!(iterable_type.kind(), TypeKind::Str) {
        PrimitiveType::str()
    } else {
        return Err(RaccoonError::new(
            format!("Cannot iterate over type '{:?}'", iterable_type),
            stmt.position,
            analyzer.file.clone(),
        ));
    };

    analyzer.symbol_table.enter_scope();

    analyzer.symbol_table.define(
        stmt.variable.clone(),
        SymbolKind::Variable,
        element_type,
        false,
        None,
    );

    let prev_in_loop = analyzer.in_loop;
    analyzer.in_loop = true;

    analyzer.check_stmt(&stmt.body)?;

    analyzer.in_loop = prev_in_loop;
    analyzer.symbol_table.exit_scope();

    Ok(PrimitiveType::void())
}

pub fn check_for_of_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &ForOfStmt,
) -> Result<Type, RaccoonError> {
    let iterable_type = analyzer.check_expr(&stmt.iterable)?;

    let element_type = if let Type::Array(ref list_type) = iterable_type {
        list_type.element_type.clone()
    } else if matches!(iterable_type.kind(), TypeKind::Str) {
        PrimitiveType::str()
    } else {
        return Err(RaccoonError::new(
            format!("Cannot iterate over type '{:?}'", iterable_type),
            stmt.position,
            analyzer.file.clone(),
        ));
    };

    analyzer.symbol_table.enter_scope();

    analyzer.symbol_table.define(
        stmt.variable.clone(),
        SymbolKind::Variable,
        element_type,
        false,
        None,
    );

    let prev_in_loop = analyzer.in_loop;
    analyzer.in_loop = true;

    analyzer.check_stmt(&stmt.body)?;

    analyzer.in_loop = prev_in_loop;
    analyzer.symbol_table.exit_scope();

    Ok(PrimitiveType::void())
}

pub fn check_switch_stmt(
    analyzer: &mut SemanticAnalyzer,
    stmt: &SwitchStmt,
) -> Result<Type, RaccoonError> {
    let discriminant_type = analyzer.check_expr(&stmt.discriminant)?;

    for case in &stmt.cases {
        if let Some(ref test) = case.test {
            let test_type = analyzer.check_expr(test)?;
            // Check that test type is comparable with discriminant type
            // For now, we'll allow any comparison
            let _ = (discriminant_type.clone(), test_type);
        }

        for consequent_stmt in &case.consequent {
            analyzer.check_stmt(consequent_stmt)?;
        }
    }

    Ok(PrimitiveType::void())
}
