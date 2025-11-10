use std::collections::HashMap;

use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::SymbolKind,
    type_system::TypeResolver,
};

use super::SemanticAnalyzer;

pub fn register_class(
    analyzer: &mut SemanticAnalyzer,
    decl: &ClassDecl,
) -> Result<(), RaccoonError> {
    let mut superclass = None;
    if let Some(ref superclass_name) = decl.superclass {
        let super_symbol = analyzer
            .symbol_table
            .lookup(superclass_name)
            .ok_or_else(|| {
                RaccoonError::new(
                    format!("Superclass '{}' not found", superclass_name),
                    decl.position,
                    analyzer.file.clone(),
                )
            })?;

        if super_symbol.kind != SymbolKind::Class {
            return Err(RaccoonError::new(
                format!("'{}' is not a class", superclass_name),
                decl.position,
                analyzer.file.clone(),
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

    analyzer.symbol_table.define(
        decl.name.clone(),
        SymbolKind::Class,
        Type::Class(Box::new(class_type)),
        false,
        Some(Box::new(Stmt::ClassDecl(decl.clone()))),
    );

    Ok(())
}

pub fn register_interface(
    analyzer: &mut SemanticAnalyzer,
    decl: &InterfaceDecl,
) -> Result<(), RaccoonError> {
    let mut properties = HashMap::new();

    for prop in &decl.properties {
        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
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

    analyzer.symbol_table.define(
        decl.name.clone(),
        SymbolKind::Interface,
        Type::Interface(Box::new(interface_type)),
        false,
        Some(Box::new(Stmt::InterfaceDecl(decl.clone()))),
    );

    Ok(())
}

pub fn register_enum(analyzer: &mut SemanticAnalyzer, decl: &EnumDecl) -> Result<(), RaccoonError> {
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
                        analyzer.file.clone(),
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

    analyzer.symbol_table.define(
        decl.name.clone(),
        SymbolKind::Enum,
        Type::Enum(Box::new(enum_type)),
        false,
        Some(Box::new(Stmt::EnumDecl(decl.clone()))),
    );

    Ok(())
}

pub fn register_type_alias(
    analyzer: &mut SemanticAnalyzer,
    decl: &TypeAliasDecl,
) -> Result<(), RaccoonError> {
    let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
    let resolved_type = resolver.resolve(&decl.alias_type)?;

    analyzer.symbol_table.define(
        decl.name.clone(),
        SymbolKind::TypeAlias,
        resolved_type,
        false,
        Some(Box::new(Stmt::TypeAliasDecl(decl.clone()))),
    );

    Ok(())
}

pub fn register_function(
    analyzer: &mut SemanticAnalyzer,
    decl: &FnDecl,
) -> Result<(), RaccoonError> {
    let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());

    let mut param_types = Vec::new();
    for param in &decl.parameters {
        let resolved = resolver.resolve(&param.param_type)?;
        param_types.push(resolved);
    }

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

    analyzer.symbol_table.define(
        decl.name.clone(),
        SymbolKind::Function,
        Type::Function(Box::new(fn_type)),
        false,
        Some(Box::new(Stmt::FnDecl(decl.clone()))),
    );

    Ok(())
}

pub fn check_var_decl(
    analyzer: &mut SemanticAnalyzer,
    decl: &VarDecl,
) -> Result<Type, RaccoonError> {
    let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());

    let explicit_type = resolver.resolve(&decl.type_annotation)?;

    let var_type: Type;

    if let Some(ref initializer) = decl.initializer {
        let init_type = analyzer.check_expr(initializer)?;

        if matches!(explicit_type.kind(), TypeKind::Unknown | TypeKind::Any) {
            var_type = init_type;
        } else {
            if !init_type.is_assignable_to(&explicit_type) {
                return Err(RaccoonError::new(
                    format!(
                        "Cannot assign type '{:?}' to variable of type '{:?}'",
                        init_type, explicit_type
                    ),
                    decl.position,
                    analyzer.file.clone(),
                ));
            }
            var_type = explicit_type;
        }
    } else {
        if matches!(explicit_type.kind(), TypeKind::Unknown) {
            return Err(RaccoonError::new(
                "Variable must have a type annotation or an initializer",
                decl.position,
                analyzer.file.clone(),
            ));
        }
        var_type = explicit_type;
    }

    if let VarPattern::Identifier(ref name) = decl.pattern {
        analyzer.symbol_table.define(
            name.clone(),
            SymbolKind::Variable,
            var_type.clone(),
            decl.is_constant,
            Some(Box::new(Stmt::VarDecl(decl.clone()))),
        );
    }

    Ok(var_type)
}

pub fn check_fn_decl(analyzer: &mut SemanticAnalyzer, decl: &FnDecl) -> Result<Type, RaccoonError> {
    let fn_symbol = analyzer
        .symbol_table
        .lookup(&decl.name)
        .ok_or_else(|| {
            RaccoonError::new(
                format!("Function '{}' not found", decl.name),
                decl.position,
                analyzer.file.clone(),
            )
        })?
        .clone();

    let prev_function = analyzer.current_function.clone();
    let prev_async = analyzer.in_async_function;

    analyzer.current_function = Some(fn_symbol.clone());
    analyzer.in_async_function = decl.is_async;

    analyzer.symbol_table.enter_scope();

    let param_types: Result<Vec<_>, _> = {
        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
        decl.parameters
            .iter()
            .map(|param| resolver.resolve(&param.param_type))
            .collect()
    };
    let param_types = param_types?;

    for (param, param_type) in decl.parameters.iter().zip(param_types.iter()) {
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

    let explicit_return_type = if let Some(ref ret_type) = decl.return_type {
        let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
        Some(resolver.resolve(ret_type)?)
    } else {
        None
    };

    let mut final_return_type = if let Some(explicit) = explicit_return_type {
        explicit
    } else {
        analyzer.infer_function_return_type(&decl.body)?
    };

    if decl.is_async && !matches!(final_return_type, Type::Future(_)) {
        final_return_type = Type::Future(Box::new(FutureType {
            inner_type: final_return_type,
        }));
    }

    let updated_fn_type = FunctionType {
        params: param_types,
        return_type: final_return_type.clone(),
        is_variadic: false,
    };

    analyzer
        .symbol_table
        .update_symbol_type(&decl.name, Type::Function(Box::new(updated_fn_type)))?;

    analyzer.symbol_table.exit_scope();
    analyzer.current_function = prev_function;
    analyzer.in_async_function = prev_async;

    Ok(Type::Function(Box::new(FunctionType {
        params: vec![],
        return_type: final_return_type,
        is_variadic: false,
    })))
}

pub fn check_class_decl(
    analyzer: &mut SemanticAnalyzer,
    decl: &ClassDecl,
) -> Result<Type, RaccoonError> {
    let class_symbol = analyzer
        .symbol_table
        .lookup(&decl.name)
        .ok_or_else(|| {
            RaccoonError::new(
                format!("Class '{}' not found", decl.name),
                decl.position,
                analyzer.file.clone(),
            )
        })?
        .clone();

    let prev_class = analyzer.current_class.clone();
    analyzer.current_class = Some(class_symbol.clone());

    analyzer.symbol_table.enter_scope();

    if let Type::Class(ref class_type) = class_symbol.symbol_type {
        analyzer.symbol_table.define(
            "this".to_string(),
            SymbolKind::Variable,
            Type::Class(class_type.clone()),
            false,
            None,
        );
    }

    for prop in &decl.properties {
        if let Some(ref initializer) = prop.initializer {
            let init_type = analyzer.check_expr(initializer)?;
            let prop_type = {
                let resolver = TypeResolver::new(&analyzer.symbol_table, analyzer.file.clone());
                resolver.resolve(&prop.property_type)?
            };

            if !init_type.is_assignable_to(&prop_type) {
                return Err(RaccoonError::new(
                    format!(
                        "Property '{}' initializer type '{:?}' not assignable to '{:?}'",
                        prop.name, init_type, prop_type
                    ),
                    decl.position,
                    analyzer.file.clone(),
                ));
            }
        }
    }

    analyzer.symbol_table.exit_scope();
    analyzer.current_class = prev_class;

    Ok(PrimitiveType::void())
}
