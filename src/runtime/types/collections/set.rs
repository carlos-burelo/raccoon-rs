use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, IntValue, ListValue, NullValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// SetType - Unique collection (Set<T>)
// ============================================================================

pub struct SetType;

#[async_trait]
impl TypeHandler for SetType {
    fn type_name(&self) -> &str {
        "set"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let set = match value {
            RuntimeValue::List(list) => list,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected set, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "add" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "add requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let item = &args[0];
                // Check if item already exists
                let exists = set.elements.iter().any(|e| e.equals(item));
                if !exists {
                    set.elements.push(item.clone());
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "remove" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "remove requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let item = &args[0];
                set.elements.retain(|e| !e.equals(item));
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "contains" | "has" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "contains requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let item = &args[0];
                let exists = set.elements.iter().any(|e| e.equals(item));
                Ok(RuntimeValue::Bool(BoolValue::new(exists)))
            }
            "size" | "length" => Ok(RuntimeValue::Int(IntValue::new(set.elements.len() as i64))),
            "clear" => {
                set.elements.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "toList" => Ok(RuntimeValue::List(ListValue::new(
                set.elements.clone(),
                PrimitiveType::any(),
            ))),
            "union" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "union requires 1 argument (set)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::List(other) => {
                        let mut result = set.elements.clone();
                        for item in &other.elements {
                            if !result.iter().any(|e| e.equals(item)) {
                                result.push(item.clone());
                            }
                        }
                        Ok(RuntimeValue::List(ListValue::new(
                            result,
                            PrimitiveType::any(),
                        )))
                    }
                    _ => Err(RaccoonError::new(
                        "union requires set argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "intersection" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "intersection requires 1 argument (set)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::List(other) => {
                        let result: Vec<RuntimeValue> = set
                            .elements
                            .iter()
                            .filter(|e| other.elements.iter().any(|o| o.equals(e)))
                            .cloned()
                            .collect();
                        Ok(RuntimeValue::List(ListValue::new(
                            result,
                            PrimitiveType::any(),
                        )))
                    }
                    _ => Err(RaccoonError::new(
                        "intersection requires set argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on set", method),
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
            "from" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "from requires 1 argument (list)".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut unique = Vec::new();
                        for item in &list.elements {
                            if !unique.iter().any(|e: &RuntimeValue| e.equals(item)) {
                                unique.push(item.clone());
                            }
                        }
                        Ok(RuntimeValue::List(ListValue::new(
                            unique,
                            PrimitiveType::any(),
                        )))
                    }
                    _ => Err(RaccoonError::new(
                        "from requires list argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on set type", method),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "add"
                | "remove"
                | "contains"
                | "has"
                | "size"
                | "length"
                | "clear"
                | "toList"
                | "union"
                | "intersection"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "from")
    }
}
