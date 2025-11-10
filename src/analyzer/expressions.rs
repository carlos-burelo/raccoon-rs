use std::collections::HashMap;

use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::SymbolKind,
    type_system::TypeResolver,
};

use super::SemanticAnalyzer;

pub fn check_expr(analyzer: &mut SemanticAnalyzer, expr: &Expr) -> Result<Type, RaccoonError> {
    match expr {
        Expr::Binary(e) => check_binary_expr(analyzer, e),
        Expr::Unary(e) => check_unary_expr(analyzer, e),
        Expr::Call(e) => check_call_expr(analyzer, e),
        Expr::New(e) => check_new_expr(analyzer, e),
        Expr::Member(e) => check_member_expr(analyzer, e),
        Expr::MethodCall(e) => check_method_call_expr(analyzer, e),
        Expr::Index(e) => check_index_expr(analyzer, e),
        Expr::Await(e) => check_await_expr(analyzer, e),
        Expr::This(_) => check_this_expr(analyzer),
        Expr::Super(_) => check_super_expr(analyzer),
        Expr::TypeOf(_) => Ok(PrimitiveType::str()),
        Expr::InstanceOf(e) => check_instanceof_expr(analyzer, e),
        Expr::ArrowFn(e) => check_arrow_fn_expr(analyzer, e),
        Expr::Identifier(e) => check_identifier(analyzer, e),
        Expr::Assignment(e) => check_assignment(analyzer, e),
        Expr::Range(e) => check_range_expr(analyzer, e),
        Expr::Conditional(e) => check_conditional_expr(analyzer, e),
        Expr::NullCoalescing(e) => check_null_coalescing_expr(analyzer, e),
        Expr::OptionalChaining(e) => check_optional_chaining_expr(analyzer, e),
        Expr::NullAssertion(e) => check_null_assertion_expr(analyzer, e),
        Expr::UnaryUpdate(e) => check_unary_update_expr(analyzer, e),
        Expr::TemplateStr(_) => Ok(PrimitiveType::str()),
        Expr::TaggedTemplate(_) => Ok(PrimitiveType::str()),
        Expr::IntLiteral(_) => Ok(PrimitiveType::int()),
        Expr::BigIntLiteral(_) => Ok(PrimitiveType::bigint()),
        Expr::FloatLiteral(_) => Ok(PrimitiveType::float()),
        Expr::StrLiteral(_) => Ok(PrimitiveType::str()),
        Expr::BoolLiteral(_) => Ok(PrimitiveType::bool()),
        Expr::NullLiteral(_) => Ok(PrimitiveType::null()),
        Expr::ListLiteral(e) => check_list_literal(analyzer, e),
        Expr::ObjectLiteral(e) => check_object_literal(analyzer, e),
        Expr::Spread(_) => Err(RaccoonError::new(
            "Spread operator cannot be used outside of function calls",
            (1, 1),
            analyzer.file.clone(),
        )),
        Expr::Match(e) => check_match_expr(analyzer, e),
        Expr::Class(e) => check_class_expr(analyzer, e),
    }
}

pub fn check_binary_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &BinaryExpr,
) -> Result<Type, RaccoonError> {
    let left_type = analyzer.check_expr(&expr.left)?;
    let right_type = analyzer.check_expr(&expr.right)?;

    if !analyzer
        .type_checker
        .are_types_compatible(&left_type, &right_type, expr.operator)
    {
        return Err(RaccoonError::new(
            format!(
                "Cannot apply operator {:?} to types '{:?}' and '{:?}'",
                expr.operator, left_type, right_type
            ),
            expr.position,
            analyzer.file.clone(),
        ));
    }

    analyzer
        .type_checker
        .infer_binary_type(expr.operator, &left_type, &right_type)
}

pub fn check_unary_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &UnaryExpr,
) -> Result<Type, RaccoonError> {
    let operand_type = analyzer.check_expr(&expr.operand)?;
    analyzer
        .type_checker
        .infer_unary_type(expr.operator, &operand_type, expr.position)
}

pub fn check_call_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &CallExpr,
) -> Result<Type, RaccoonError> {
    let callee_type = analyzer.check_expr(&expr.callee)?;

    if let Type::Function(fn_type) = callee_type {
        if expr.args.len() != fn_type.params.len() {
            return Err(RaccoonError::new(
                format!(
                    "Function expects {} arguments, got {}",
                    fn_type.params.len(),
                    expr.args.len()
                ),
                expr.position,
                analyzer.file.clone(),
            ));
        }

        for (i, arg) in expr.args.iter().enumerate() {
            let arg_type = analyzer.check_expr(arg)?;
            if !arg_type.is_assignable_to(&fn_type.params[i]) {
                return Err(RaccoonError::new(
                    format!(
                        "Argument {}: type '{:?}' not assignable to '{:?}'",
                        i + 1,
                        arg_type,
                        fn_type.params[i]
                    ),
                    expr.position,
                    analyzer.file.clone(),
                ));
            }
        }

        return Ok(fn_type.return_type);
    }

    Err(RaccoonError::new(
        format!("Cannot call non-function type '{:?}'", callee_type),
        expr.position,
        analyzer.file.clone(),
    ))
}

pub fn check_new_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &NewExpr,
) -> Result<Type, RaccoonError> {
    if expr.class_name == "Map" {
        if expr.type_args.len() != 2 {
            return Err(RaccoonError::new(
                "Map requires exactly two type arguments",
                expr.position,
                analyzer.file.clone(),
            ));
        }

        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
        let key_type = resolver.resolve(&expr.type_args[0])?;
        let value_type = resolver.resolve(&expr.type_args[1])?;

        return Ok(Type::Map(Box::new(MapType {
            key_type,
            value_type,
        })));
    }

    let class_symbol = analyzer
        .symbol_table
        .lookup(&expr.class_name)
        .ok_or_else(|| {
            RaccoonError::new(
                format!("Class '{}' not found", expr.class_name),
                expr.position,
                analyzer.file.clone(),
            )
        })?;

    if class_symbol.kind != SymbolKind::Class {
        return Err(RaccoonError::new(
            format!("'{}' is not a class", expr.class_name),
            expr.position,
            analyzer.file.clone(),
        ));
    }

    Ok(class_symbol.symbol_type.clone())
}

pub fn check_member_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &MemberExpr,
) -> Result<Type, RaccoonError> {
    let object_type = analyzer.check_expr(&expr.object)?;

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
            analyzer.file.clone(),
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
        analyzer.file.clone(),
    ))
}

pub fn check_method_call_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &MethodCallExpr,
) -> Result<Type, RaccoonError> {
    let object_type = analyzer.check_expr(&expr.object)?;

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
                    analyzer.file.clone(),
                ));
            }

            for (i, arg) in expr.args.iter().enumerate() {
                let arg_type = analyzer.check_expr(arg)?;
                if !arg_type.is_assignable_to(&method_info.method_type.params[i]) {
                    return Err(RaccoonError::new(
                        format!(
                            "Argument {}: type '{:?}' not assignable to '{:?}'",
                            i + 1,
                            arg_type,
                            method_info.method_type.params[i]
                        ),
                        expr.position,
                        analyzer.file.clone(),
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
        analyzer.file.clone(),
    ))
}

pub fn check_index_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &IndexExpr,
) -> Result<Type, RaccoonError> {
    let object_type = analyzer.check_expr(&expr.object)?;
    let index_type = analyzer.check_expr(&expr.index)?;

    analyzer
        .type_checker
        .validate_index_expr(&object_type, &index_type, expr.position)
}

pub fn check_await_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &AwaitExpr,
) -> Result<Type, RaccoonError> {
    let expr_type = analyzer.check_expr(&expr.expression)?;

    if let Type::Future(future_type) = expr_type {
        return Ok(future_type.inner_type);
    }

    Err(RaccoonError::new(
        format!(
            "Cannot await non-Future type '{:?}'. Expected Future<T>",
            expr_type
        ),
        expr.position,
        analyzer.file.clone(),
    ))
}

pub fn check_this_expr(analyzer: &SemanticAnalyzer) -> Result<Type, RaccoonError> {
    if let Some(ref current_class) = analyzer.current_class {
        return Ok(current_class.symbol_type.clone());
    }

    Err(RaccoonError::new(
        "Cannot use 'this' outside of class",
        (0, 0),
        analyzer.file.clone(),
    ))
}

pub fn check_super_expr(analyzer: &SemanticAnalyzer) -> Result<Type, RaccoonError> {
    if let Some(ref current_class) = analyzer.current_class {
        if let Type::Class(ref class_type) = current_class.symbol_type {
            if let Some(ref superclass) = class_type.superclass {
                return Ok(Type::Class(superclass.clone()));
            }
            return Err(RaccoonError::new(
                "Cannot use 'super' in class without superclass",
                (0, 0),
                analyzer.file.clone(),
            ));
        }
    }

    Err(RaccoonError::new(
        "Cannot use 'super' outside of class",
        (0, 0),
        analyzer.file.clone(),
    ))
}

pub fn check_instanceof_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &InstanceOfExpr,
) -> Result<Type, RaccoonError> {
    analyzer.check_expr(&expr.operand)?;

    let type_symbol = analyzer.symbol_table.lookup(&expr.type_name);
    if type_symbol.is_none() || type_symbol.as_ref().unwrap().kind != SymbolKind::Class {
        return Err(RaccoonError::new(
            format!("'{}' is not a class", expr.type_name),
            expr.position,
            analyzer.file.clone(),
        ));
    }

    Ok(PrimitiveType::bool())
}

pub fn check_arrow_fn_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &ArrowFnExpr,
) -> Result<Type, RaccoonError> {
    analyzer.symbol_table.enter_scope();

    let param_types: Result<Vec<_>, _> = {
        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
        expr.parameters
            .iter()
            .map(|param| resolver.resolve(&param.param_type))
            .collect()
    };
    let param_types = param_types?;

    for (param, param_type) in expr.parameters.iter().zip(param_types.iter()) {
        if let VarPattern::Identifier(ref name) = param.pattern {
            analyzer.symbol_table.define(
                name.clone(),
                SymbolKind::Parameter,
                param_type.clone(),
                false,
                None,
            );
        }
    }

    let inferred_return_type = match &expr.body {
        ArrowFnBody::Expr(body_expr) => analyzer.check_expr(body_expr)?,
        ArrowFnBody::Block(stmts) => {
            let mut last_return_type = PrimitiveType::void();
            for stmt in stmts {
                if let Stmt::ReturnStmt(ret) = stmt {
                    if let Some(ref value) = ret.value {
                        last_return_type = analyzer.check_expr(value)?;
                    }
                } else {
                    analyzer.check_stmt(stmt)?;
                }
            }
            last_return_type
        }
    };

    let return_type = if let Some(ref explicit_type) = expr.return_type {
        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
        let resolved_type = resolver.resolve(explicit_type)?;

        if !inferred_return_type.is_assignable_to(&resolved_type) {
            analyzer.symbol_table.exit_scope();
            return Err(RaccoonError::new(
                format!(
                    "Function body returns '{:?}' but declared return type is '{:?}'",
                    inferred_return_type, resolved_type
                ),
                expr.position,
                analyzer.file.clone(),
            ));
        }
        resolved_type
    } else {
        inferred_return_type
    };

    analyzer.symbol_table.exit_scope();

    Ok(Type::Function(Box::new(FunctionType {
        params: param_types,
        return_type,
        is_variadic: false,
    })))
}

pub fn check_identifier(
    analyzer: &mut SemanticAnalyzer,
    identifier: &Identifier,
) -> Result<Type, RaccoonError> {
    if let Some(narrowed_type) = analyzer.type_inference.get_narrowed_type(&identifier.name) {
        return Ok(narrowed_type);
    }

    let symbol = analyzer
        .symbol_table
        .lookup(&identifier.name)
        .ok_or_else(|| {
            RaccoonError::new(
                format!("Undefined variable '{}'", identifier.name),
                identifier.position,
                analyzer.file.clone(),
            )
        })?;

    Ok(symbol.symbol_type.clone())
}

pub fn check_assignment(
    analyzer: &mut SemanticAnalyzer,
    assignment: &Assignment,
) -> Result<Type, RaccoonError> {
    let target_type = analyzer.check_expr(&assignment.target)?;
    let value_type = analyzer.check_expr(&assignment.value)?;

    analyzer.type_checker.validate_assignment(
        &target_type,
        &value_type,
        assignment.operator,
        assignment.position,
    )
}

pub fn check_range_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &RangeExpr,
) -> Result<Type, RaccoonError> {
    let start_type = analyzer.check_expr(&expr.start)?;
    let end_type = analyzer.check_expr(&expr.end)?;

    analyzer
        .type_checker
        .validate_range_expr(&start_type, &end_type, expr.position)
}

pub fn check_conditional_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &ConditionalExpr,
) -> Result<Type, RaccoonError> {
    let cond_type = analyzer.check_expr(&expr.condition)?;

    if !matches!(cond_type.kind(), TypeKind::Bool) {
        return Err(RaccoonError::new(
            "Conditional expression condition must be boolean",
            expr.position,
            analyzer.file.clone(),
        ));
    }

    let then_type = analyzer.check_expr(&expr.then_expr)?;
    let else_type = analyzer.check_expr(&expr.else_expr)?;

    if then_type.equals(&else_type) {
        return Ok(then_type);
    }

    Ok(Type::Union(Box::new(UnionType::new(vec![
        then_type, else_type,
    ]))))
}

pub fn check_null_coalescing_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &NullCoalescingExpr,
) -> Result<Type, RaccoonError> {
    let left_type = analyzer.check_expr(&expr.left)?;
    let right_type = analyzer.check_expr(&expr.right)?;

    if let Type::Nullable(_nullable) = left_type {
        return Ok(right_type);
    }

    Ok(left_type)
}

pub fn check_optional_chaining_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &OptionalChainingExpr,
) -> Result<Type, RaccoonError> {
    let object_type = analyzer.check_expr(&expr.object)?;

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

pub fn check_null_assertion_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &NullAssertionExpr,
) -> Result<Type, RaccoonError> {
    let operand_type = analyzer.check_expr(&expr.operand)?;

    if let Type::Nullable(nullable) = operand_type {
        return Ok(nullable.inner_type);
    }

    Ok(operand_type)
}

pub fn check_unary_update_expr(
    analyzer: &mut SemanticAnalyzer,
    expr: &UnaryUpdateExpr,
) -> Result<Type, RaccoonError> {
    let operand_type = analyzer.check_expr(&expr.operand)?;

    if !analyzer.type_checker.is_numeric_type(&operand_type) {
        return Err(RaccoonError::new(
            "Increment/decrement requires numeric type",
            expr.position,
            analyzer.file.clone(),
        ));
    }

    Ok(operand_type)
}

pub fn check_list_literal(
    analyzer: &mut SemanticAnalyzer,
    list: &ListLiteral,
) -> Result<Type, RaccoonError> {
    if list.elements.is_empty() {
        return Ok(Type::List(Box::new(ListType {
            element_type: PrimitiveType::unknown(),
        })));
    }

    let mut element_types = Vec::new();
    for element in &list.elements {
        // Handle spread expressions in array literals
        if let Expr::Spread(spread) = element {
            let spread_type = analyzer.check_expr(&spread.argument)?;
            // If it's a list, get its element type
            if let Type::List(list_type) = spread_type {
                element_types.push(list_type.element_type.clone());
            }
        } else {
            element_types.push(analyzer.check_expr(element)?);
        }
    }

    let common_type = analyzer
        .type_inference
        .infer_common_type(&element_types, list.position)?;

    Ok(Type::List(Box::new(ListType {
        element_type: common_type,
    })))
}

pub fn check_object_literal(
    analyzer: &mut SemanticAnalyzer,
    obj: &ObjectLiteral,
) -> Result<Type, RaccoonError> {
    let mut properties = HashMap::new();

    for prop in &obj.properties {
        match prop {
            ObjectLiteralProperty::KeyValue { key, value } => {
                let value_type = analyzer.check_expr(value)?;
                properties.insert(
                    key.clone(),
                    InterfaceProperty {
                        property_type: value_type,
                        optional: false,
                    },
                );
            }
            ObjectLiteralProperty::Spread(expr) => {
                // Type check the spread expression
                let spread_type = analyzer.check_expr(expr)?;
                // If it's an object-like type, merge its properties
                if let Type::Interface(ref iface) = spread_type {
                    for (k, v) in &iface.properties {
                        properties.insert(k.clone(), v.clone());
                    }
                }
            }
        }
    }

    Ok(Type::Interface(Box::new(InterfaceType {
        name: "anonymous".to_string(),
        properties,
        type_parameters: Vec::new(),
    })))
}

pub fn check_match_expr(
    _analyzer: &mut SemanticAnalyzer,
    _expr: &MatchExpr,
) -> Result<Type, RaccoonError> {
    // TODO: Implement pattern matching type checking
    // For now, return any type
    Ok(Type::Primitive(PrimitiveType::new(TypeKind::Any, "any")))
}

pub fn check_class_expr(
    _analyzer: &mut SemanticAnalyzer,
    _expr: &ClassExpr,
) -> Result<Type, RaccoonError> {
    // Class expressions return a class type
    // The type is the class itself (can be instantiated with new)
    Ok(Type::Primitive(PrimitiveType::new(
        TypeKind::Class,
        "class",
    )))
}
