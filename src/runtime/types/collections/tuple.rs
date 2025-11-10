use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{ArrayValue, IntValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct TupleType;

impl TupleType {
    fn extract_tuple<'a>(
        value: &'a RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<&'a ArrayValue, RaccoonError> {
        match value {
            RuntimeValue::Array(list) => Ok(list),
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
                Ok(RuntimeValue::Array(ArrayValue::new(
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
            "of" => Ok(RuntimeValue::Array(ArrayValue::new(
                args,
                PrimitiveType::any(),
            ))),
            _ => Err(static_method_not_found_error(
                "tuple", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "get" | "length" | "size" | "toString" | "toStr" | "toList" | "first" | "last"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "of")
    }
}
