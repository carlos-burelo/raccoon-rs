use crate::{
    ast::types::*,
    error::RaccoonError,
    symbol_table::{SymbolKind, SymbolTable},
};

pub struct TypeResolver<'a> {
    symbol_table: &'a SymbolTable,
    file: Option<String>,
}

impl<'a> TypeResolver<'a> {
    pub fn new(symbol_table: &'a SymbolTable, file: Option<String>) -> Self {
        Self { symbol_table, file }
    }

    pub fn resolve(&self, type_: &Type) -> Result<Type, RaccoonError> {
        match type_ {
            Type::TypeRef(type_ref) => {
                let symbol = self.symbol_table.lookup(&type_ref.name).ok_or_else(|| {
                    RaccoonError::new(
                        format!("Undefined type: {}", type_ref.name),
                        (0, 0),
                        self.file.clone(),
                    )
                })?;

                match symbol.kind {
                    SymbolKind::TypeAlias
                    | SymbolKind::Class
                    | SymbolKind::Interface
                    | SymbolKind::Enum => Ok(symbol.symbol_type.clone()),
                    _ => Err(RaccoonError::new(
                        format!("'{}' is not a type", type_ref.name),
                        (0, 0),
                        self.file.clone(),
                    )),
                }
            }

            Type::Array(list_type) => Ok(Type::Array(Box::new(ArrayType {
                element_type: self.resolve(&list_type.element_type)?,
            }))),

            Type::Map(map_type) => Ok(Type::Map(Box::new(MapType {
                key_type: self.resolve(&map_type.key_type)?,
                value_type: self.resolve(&map_type.value_type)?,
            }))),

            Type::Nullable(nullable_type) => Ok(Type::Nullable(Box::new(NullableType {
                inner_type: self.resolve(&nullable_type.inner_type)?,
            }))),

            Type::Union(union_type) => {
                let resolved_types: Result<Vec<Type>, RaccoonError> =
                    union_type.types.iter().map(|t| self.resolve(t)).collect();

                Ok(Type::Union(Box::new(UnionType::new(resolved_types?))))
            }

            Type::Function(fn_type) => {
                let resolved_params: Result<Vec<Type>, RaccoonError> =
                    fn_type.params.iter().map(|p| self.resolve(p)).collect();

                Ok(Type::Function(Box::new(FunctionType {
                    params: resolved_params?,
                    return_type: self.resolve(&fn_type.return_type)?,
                    is_variadic: fn_type.is_variadic,
                })))
            }

            Type::Future(future_type) => Ok(Type::Future(Box::new(FutureType {
                inner_type: self.resolve(&future_type.inner_type)?,
            }))),

            Type::Generic(generic_type) => {
                let resolved_base = self.resolve(&generic_type.base)?;
                let resolved_args: Result<Vec<Type>, RaccoonError> = generic_type
                    .type_args
                    .iter()
                    .map(|arg| self.resolve(arg))
                    .collect();

                Ok(Type::Generic(Box::new(GenericType {
                    base: resolved_base,
                    type_args: resolved_args?,
                })))
            }

            _ => Ok(type_.clone()),
        }
    }
}
