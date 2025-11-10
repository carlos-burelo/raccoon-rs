use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    symbol_table::SymbolTable,
    tokens::Position,
    type_system::TypeChecker,
};
use std::collections::HashMap;

pub struct TypeInferenceEngine {
    pub type_checker: TypeChecker,
    pub file: Option<String>,

    narrowed_types: Vec<HashMap<String, Type>>,
}

impl TypeInferenceEngine {
    pub fn new(file: Option<String>) -> Self {
        Self {
            type_checker: TypeChecker::new(file.clone()),
            file,
            narrowed_types: vec![HashMap::new()],
        }
    }

    pub fn push_narrowing_scope(&mut self) {
        self.narrowed_types.push(HashMap::new());
    }

    pub fn pop_narrowing_scope(&mut self) {
        self.narrowed_types.pop();
    }

    pub fn get_narrowed_type(&self, name: &str) -> Option<Type> {
        for scope in self.narrowed_types.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    pub fn set_narrowed_type(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.narrowed_types.last_mut() {
            scope.insert(name, ty);
        }
    }

    pub fn infer_with_hint(
        &mut self,
        expr: &Expr,
        expected_type: Option<&Type>,
        symbol_table: &SymbolTable,
    ) -> Result<Type, RaccoonError> {
        match expr {
            Expr::ArrayLiteral(list) => self.infer_list_with_hint(list, expected_type, symbol_table),
            Expr::ObjectLiteral(obj) => {
                self.infer_object_with_hint(obj, expected_type, symbol_table)
            }
            Expr::ArrowFn(arrow) => {
                self.infer_arrow_fn_with_hint(arrow, expected_type, symbol_table)
            }
            Expr::Conditional(cond) => {
                self.infer_conditional_with_hint(cond, expected_type, symbol_table)
            }
            _ => Ok(PrimitiveType::unknown()),
        }
    }

    fn infer_list_with_hint(
        &mut self,
        _list: &ArrayLiteral,
        expected_type: Option<&Type>,
        _symbol_table: &SymbolTable,
    ) -> Result<Type, RaccoonError> {
        if let Some(Type::Array(list_type)) = expected_type {
            Ok(Type::Array(Box::new(ArrayType {
                element_type: list_type.element_type.clone(),
            })))
        } else {
            Ok(Type::Array(Box::new(ArrayType {
                element_type: PrimitiveType::unknown(),
            })))
        }
    }

    fn infer_object_with_hint(
        &mut self,
        _obj: &ObjectLiteral,
        expected_type: Option<&Type>,
        _symbol_table: &SymbolTable,
    ) -> Result<Type, RaccoonError> {
        if let Some(Type::Interface(interface_type)) = expected_type {
            Ok(Type::Interface(interface_type.clone()))
        } else {
            Ok(Type::Interface(Box::new(InterfaceType {
                name: "anonymous".to_string(),
                properties: HashMap::new(),
                type_parameters: Vec::new(),
            })))
        }
    }

    fn infer_arrow_fn_with_hint(
        &mut self,
        _arrow: &ArrowFnExpr,
        expected_type: Option<&Type>,
        _symbol_table: &SymbolTable,
    ) -> Result<Type, RaccoonError> {
        if let Some(Type::Function(fn_type)) = expected_type {
            Ok(Type::Function(fn_type.clone()))
        } else {
            Ok(Type::Function(Box::new(FunctionType {
                params: Vec::new(),
                return_type: PrimitiveType::unknown(),
                is_variadic: false,
            })))
        }
    }

    fn infer_conditional_with_hint(
        &mut self,
        _cond: &ConditionalExpr,
        expected_type: Option<&Type>,
        _symbol_table: &SymbolTable,
    ) -> Result<Type, RaccoonError> {
        if let Some(ty) = expected_type {
            Ok(ty.clone())
        } else {
            Ok(PrimitiveType::unknown())
        }
    }

    pub fn analyze_type_narrowing(
        &mut self,
        condition: &Expr,
        symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        match condition {
            Expr::Binary(bin) if matches!(bin.operator, crate::tokens::BinaryOperator::Equal) => {
                self.analyze_typeof_narrowing(bin, symbol_table)
            }

            Expr::Binary(bin)
                if matches!(bin.operator, crate::tokens::BinaryOperator::NotEqual) =>
            {
                self.analyze_null_check_narrowing(bin, symbol_table)
            }

            Expr::InstanceOf(inst) => self.analyze_instanceof_narrowing(inst, symbol_table),

            Expr::Unary(un) if matches!(un.operator, crate::tokens::UnaryOperator::Not) => {
                let inner_info = self.analyze_type_narrowing(&un.operand, symbol_table)?;
                Ok(TypeNarrowingInfo {
                    then_narrows: inner_info.else_narrows,
                    else_narrows: inner_info.then_narrows,
                })
            }

            Expr::Binary(bin) if matches!(bin.operator, crate::tokens::BinaryOperator::And) => {
                self.analyze_and_narrowing(bin, symbol_table)
            }

            Expr::Binary(bin) if matches!(bin.operator, crate::tokens::BinaryOperator::Or) => {
                self.analyze_or_narrowing(bin, symbol_table)
            }

            _ => Ok(TypeNarrowingInfo::default()),
        }
    }

    fn analyze_typeof_narrowing(
        &mut self,
        bin: &BinaryExpr,
        _symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        if let Expr::TypeOf(typeof_expr) = &*bin.left {
            if let Expr::Identifier(ident) = &*typeof_expr.operand {
                if let Expr::StrLiteral(type_str) = &*bin.right {
                    let narrowed_type = match type_str.value.as_str() {
                        "int" => PrimitiveType::int(),
                        "float" => PrimitiveType::float(),
                        "str" | "string" => PrimitiveType::str(),
                        "bool" | "boolean" => PrimitiveType::bool(),
                        _ => return Ok(TypeNarrowingInfo::default()),
                    };

                    let mut then_narrows = HashMap::new();
                    then_narrows.insert(ident.name.clone(), narrowed_type);

                    return Ok(TypeNarrowingInfo {
                        then_narrows,
                        else_narrows: HashMap::new(),
                    });
                }
            }
        }

        Ok(TypeNarrowingInfo::default())
    }

    fn analyze_null_check_narrowing(
        &mut self,
        bin: &BinaryExpr,
        symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        let (ident, is_null_check) = if let Expr::Identifier(id) = &*bin.left {
            if matches!(&*bin.right, Expr::NullLiteral(_)) {
                (id, true)
            } else {
                return Ok(TypeNarrowingInfo::default());
            }
        } else if let Expr::Identifier(id) = &*bin.right {
            if matches!(&*bin.left, Expr::NullLiteral(_)) {
                (id, true)
            } else {
                return Ok(TypeNarrowingInfo::default());
            }
        } else {
            return Ok(TypeNarrowingInfo::default());
        };

        if !is_null_check {
            return Ok(TypeNarrowingInfo::default());
        }

        if let Some(symbol) = symbol_table.lookup(&ident.name) {
            if let Type::Nullable(nullable) = &symbol.symbol_type {
                let mut then_narrows = HashMap::new();
                then_narrows.insert(ident.name.clone(), nullable.inner_type.clone());

                return Ok(TypeNarrowingInfo {
                    then_narrows,
                    else_narrows: HashMap::new(),
                });
            }
        }

        Ok(TypeNarrowingInfo::default())
    }

    fn analyze_instanceof_narrowing(
        &mut self,
        inst: &InstanceOfExpr,
        symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        if let Expr::Identifier(ident) = &*inst.operand {
            if let Some(class_symbol) = symbol_table.lookup(&inst.type_name) {
                let mut then_narrows = HashMap::new();
                then_narrows.insert(ident.name.clone(), class_symbol.symbol_type.clone());

                return Ok(TypeNarrowingInfo {
                    then_narrows,
                    else_narrows: HashMap::new(),
                });
            }
        }

        Ok(TypeNarrowingInfo::default())
    }

    fn analyze_and_narrowing(
        &mut self,
        bin: &BinaryExpr,
        symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        let left_info = self.analyze_type_narrowing(&bin.left, symbol_table)?;
        let right_info = self.analyze_type_narrowing(&bin.right, symbol_table)?;

        let mut then_narrows = left_info.then_narrows;
        then_narrows.extend(right_info.then_narrows);

        Ok(TypeNarrowingInfo {
            then_narrows,
            else_narrows: HashMap::new(),
        })
    }

    fn analyze_or_narrowing(
        &mut self,
        bin: &BinaryExpr,
        symbol_table: &SymbolTable,
    ) -> Result<TypeNarrowingInfo, RaccoonError> {
        let left_info = self.analyze_type_narrowing(&bin.left, symbol_table)?;
        let right_info = self.analyze_type_narrowing(&bin.right, symbol_table)?;

        let mut else_narrows = left_info.else_narrows;
        else_narrows.extend(right_info.else_narrows);

        Ok(TypeNarrowingInfo {
            then_narrows: HashMap::new(),
            else_narrows,
        })
    }

    pub fn infer_common_type(
        &self,
        types: &[Type],
        _position: Position,
    ) -> Result<Type, RaccoonError> {
        if types.is_empty() {
            return Ok(PrimitiveType::unknown());
        }

        if types.len() == 1 {
            return Ok(types[0].clone());
        }

        let first = &types[0];
        if types.iter().all(|t| t.equals(first)) {
            return Ok(first.clone());
        }

        if types.iter().all(|t| t.is_assignable_to(first)) {
            return Ok(first.clone());
        }

        if types.iter().all(|t| self.type_checker.is_numeric_type(t)) {
            let mut widest = types[0].clone();
            for ty in &types[1..] {
                widest = self.type_checker.get_wider_numeric_type(&widest, ty);
            }
            return Ok(widest);
        }

        let non_nullable_types: Vec<Type> = types
            .iter()
            .filter_map(|t| {
                if let Type::Nullable(n) = t {
                    Some(n.inner_type.clone())
                } else if !matches!(t, Type::Primitive(p) if p.kind == TypeKind::Null) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect();

        let has_null = types
            .iter()
            .any(|t| matches!(t, Type::Primitive(p) if p.kind == TypeKind::Null));

        if non_nullable_types.len() == 1 && has_null {
            return Ok(Type::Nullable(Box::new(NullableType {
                inner_type: non_nullable_types[0].clone(),
            })));
        }

        let mut unique_types = Vec::new();
        for ty in types {
            if !unique_types.iter().any(|t: &Type| t.equals(ty)) {
                unique_types.push(ty.clone());
            }
        }

        if unique_types.len() == 1 {
            Ok(unique_types[0].clone())
        } else if unique_types.len() <= 5 {
            Ok(Type::Union(Box::new(UnionType::new(unique_types))))
        } else {
            Ok(PrimitiveType::any())
        }
    }

    pub fn infer_type_arguments(
        &self,
        type_params: &[TypeParameter],
        param_types: &[Type],
        arg_types: &[Type],
        _position: Position,
    ) -> Result<HashMap<String, Type>, RaccoonError> {
        let mut substitutions = HashMap::new();

        for (param_type, arg_type) in param_types.iter().zip(arg_types.iter()) {
            self.collect_type_substitutions(param_type, arg_type, &mut substitutions)?;
        }

        for type_param in type_params {
            if !substitutions.contains_key(&type_param.name) {
                let inferred = if let Some(constraint) = &type_param.constraint {
                    (**constraint).clone()
                } else {
                    PrimitiveType::unknown()
                };
                substitutions.insert(type_param.name.clone(), inferred);
            }
        }

        Ok(substitutions)
    }

    fn collect_type_substitutions(
        &self,
        param_type: &Type,
        arg_type: &Type,
        substitutions: &mut HashMap<String, Type>,
    ) -> Result<(), RaccoonError> {
        match param_type {
            Type::TypeParam(type_param) => {
                if let Some(existing) = substitutions.get(&type_param.name) {
                    if !existing.equals(arg_type) {
                        if self.type_checker.is_numeric_type(existing)
                            && self.type_checker.is_numeric_type(arg_type)
                        {
                            let wider =
                                self.type_checker.get_wider_numeric_type(existing, arg_type);
                            substitutions.insert(type_param.name.clone(), wider);
                        }
                    }
                } else {
                    substitutions.insert(type_param.name.clone(), arg_type.clone());
                }
                Ok(())
            }

            Type::Array(list_type) => {
                if let Type::Array(arg_list) = arg_type {
                    self.collect_type_substitutions(
                        &list_type.element_type,
                        &arg_list.element_type,
                        substitutions,
                    )
                } else {
                    Ok(())
                }
            }

            Type::Generic(generic_type) => {
                if let Type::Generic(arg_generic) = arg_type {
                    self.collect_type_substitutions(
                        &generic_type.base,
                        &arg_generic.base,
                        substitutions,
                    )?;

                    for (param_arg, arg_arg) in
                        generic_type.type_args.iter().zip(&arg_generic.type_args)
                    {
                        self.collect_type_substitutions(param_arg, arg_arg, substitutions)?;
                    }
                }
                Ok(())
            }

            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeNarrowingInfo {
    pub then_narrows: HashMap<String, Type>,

    pub else_narrows: HashMap<String, Type>,
}
