/// Refactored TupleType using helpers and metadata system
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ListValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// TupleType - Fixed-size heterogeneous collection (Tuple<T1, T2, ...>)
// ============================================================================

pub struct TupleType;

impl TupleType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new(
            "tuple",
            "Fixed-size heterogeneous collection with multiple types",
        )
        .with_instance_methods(vec![
            MethodMetadata::new("get", "any", "Get element at index")
                .with_params(vec![ParamMetadata::new("index", "int")]),
            MethodMetadata::new("length", "int", "Get number of elements").with_alias("size"),
            MethodMetadata::new("toString", "str", "Convert to string").with_alias("toStr"),
            MethodMetadata::new("toList", "list", "Convert to list"),
            MethodMetadata::new("first", "any", "Get first element"),
            MethodMetadata::new("last", "any", "Get last element"),
        ])
        .with_static_methods(vec![MethodMetadata::new(
            "of",
            "tuple",
            "Create tuple from elements",
        )
        .with_params(vec![ParamMetadata::new("elements", "any").variadic()])])
    }

    /// Helper to extract tuple (list) from RuntimeValue
    fn extract_tuple<'a>(
        value: &'a RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<&'a ListValue, RaccoonError> {
        match value {
            RuntimeValue::List(list) => Ok(list),
            _ => Err(RaccoonError::new(
                format!("Expected tuple, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

#[async_trait]
impl TypeHandler for TupleType {
    fn type_name(&self) -> &str {
        "tuple"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let tuple = Self::extract_tuple(value, position, file.clone())?;

        match method {
            "get" => {
                require_args(&args, 1, method, position, file.clone())?;
                let index = extract_int(&args[0], "index", position, file.clone())? as usize;
                if index < tuple.elements.len() {
                    Ok(tuple.elements[index].clone())
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Index {} out of bounds for tuple of length {}",
                            index,
                            tuple.elements.len()
                        ),
                        position,
                        file,
                    ))
                }
            }
            "length" | "size" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(
                    IntValue::new(tuple.elements.len() as i64),
                ))
            }
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                let items: Vec<String> = tuple.elements.iter().map(|e| e.to_string()).collect();
                Ok(RuntimeValue::Str(StrValue::new(format!(
                    "({})",
                    items.join(", ")
                ))))
            }
            "toList" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::List(ListValue::new(
                    tuple.elements.clone(),
                    PrimitiveType::any(),
                )))
            }
            "first" => {
                require_args(&args, 0, method, position, file.clone())?;
                if tuple.elements.is_empty() {
                    Err(RaccoonError::new(
                        "Cannot get first element of empty tuple".to_string(),
                        position,
                        file,
                    ))
                } else {
                    Ok(tuple.elements[0].clone())
                }
            }
            "last" => {
                require_args(&args, 0, method, position, file.clone())?;
                if tuple.elements.is_empty() {
                    Err(RaccoonError::new(
                        "Cannot get last element of empty tuple".to_string(),
                        position,
                        file,
                    ))
                } else {
                    Ok(tuple.elements[tuple.elements.len() - 1].clone())
                }
            }
            _ => Err(method_not_found_error("tuple", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "of" => {
                // Creates a tuple from the provided arguments
                Ok(RuntimeValue::List(ListValue::new(
                    args,
                    PrimitiveType::any(),
                )))
            }
            _ => Err(static_method_not_found_error(
                "tuple", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
