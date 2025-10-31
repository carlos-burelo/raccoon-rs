use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ListValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// TupleType - Fixed-size heterogeneous collection (Tuple<T1, T2, ...>)
// ============================================================================

pub struct TupleType;

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
        let tuple = match value {
            RuntimeValue::List(list) => list,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected tuple, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "get" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "get requires 1 argument (index)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Int(i) => {
                        let index = i.value as usize;
                        if index < tuple.elements.len() {
                            Ok(tuple.elements[index].clone())
                        } else {
                            Err(RaccoonError::new(
                                format!("Index {} out of bounds for tuple of length {}", index, tuple.elements.len()),
                                position,
                                file,
                            ))
                        }
                    }
                    _ => Err(RaccoonError::new(
                        "get requires int argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "length" | "size" => Ok(RuntimeValue::Int(IntValue::new(tuple.elements.len() as i64))),
            "toString" | "toStr" => {
                let items: Vec<String> = tuple
                    .elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect();
                Ok(RuntimeValue::Str(StrValue::new(format!("({})", items.join(", ")))))
            }
            "toList" => Ok(RuntimeValue::List(ListValue::new(tuple.elements.clone(), PrimitiveType::any()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on tuple", method),
                position,
                file,
            )),
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
                Ok(RuntimeValue::List(ListValue::new(args, PrimitiveType::any())))
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on tuple type", method),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "get" | "length" | "size" | "toString" | "toStr" | "toList"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "of")
    }
}
