/// Refactored SetType using helpers and metadata system
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{ArrayValue, BoolValue, IntValue, NullValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// SetType - Unique collection (Set<T>)
// ============================================================================

pub struct SetType;

impl SetType {
    /// Helper to extract set (list) from RuntimeValue
    fn extract_set_mut<'a>(
        value: &'a mut RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<&'a mut ArrayValue, RaccoonError> {
        match value {
            RuntimeValue::Array(list) => Ok(list),
            _ => Err(RaccoonError::new(
                format!("Expected set, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

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
        let set = Self::extract_set_mut(value, position, file.clone())?;

        match method {
            "add" => {
                require_args(&args, 1, method, position, file)?;
                let item = &args[0];
                // Check if item already exists
                let exists = set.elements.iter().any(|e| e.equals(item));
                if !exists {
                    set.elements.push(item.clone());
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "remove" => {
                require_args(&args, 1, method, position, file)?;
                let item = &args[0];
                set.elements.retain(|e| !e.equals(item));
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "contains" | "has" => {
                require_args(&args, 1, method, position, file)?;
                let item = &args[0];
                let exists = set.elements.iter().any(|e| e.equals(item));
                Ok(RuntimeValue::Bool(BoolValue::new(exists)))
            }
            "size" | "length" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(set.elements.len() as i64)))
            }
            "clear" => {
                require_args(&args, 0, method, position, file)?;
                set.elements.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "isEmpty" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(set.elements.is_empty())))
            }
            "toList" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Array(ArrayValue::new(
                    set.elements.clone(),
                    PrimitiveType::any(),
                )))
            }
            "union" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_array(&args[0], "other", position, file)?;
                let mut result = set.elements.clone();
                for item in &other.elements {
                    if !result.iter().any(|e| e.equals(item)) {
                        result.push(item.clone());
                    }
                }
                Ok(RuntimeValue::Array(ArrayValue::new(
                    result,
                    PrimitiveType::any(),
                )))
            }
            "intersection" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_array(&args[0], "other", position, file)?;
                let result: Vec<RuntimeValue> = set
                    .elements
                    .iter()
                    .filter(|e| other.elements.iter().any(|o| o.equals(e)))
                    .cloned()
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    result,
                    PrimitiveType::any(),
                )))
            }
            "difference" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_array(&args[0], "other", position, file)?;
                let result: Vec<RuntimeValue> = set
                    .elements
                    .iter()
                    .filter(|e| !other.elements.iter().any(|o| o.equals(e)))
                    .cloned()
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    result,
                    PrimitiveType::any(),
                )))
            }
            _ => Err(method_not_found_error("set", method, position, file)),
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
                require_args(&args, 1, method, position, file.clone())?;
                let list = extract_array(&args[0], "array", position, file)?;
                let mut unique = Vec::new();
                for item in &list.elements {
                    if !unique.iter().any(|e: &RuntimeValue| e.equals(item)) {
                        unique.push(item.clone());
                    }
                }
                Ok(RuntimeValue::Array(ArrayValue::new(
                    unique,
                    PrimitiveType::any(),
                )))
            }
            _ => Err(static_method_not_found_error("set", method, position, file)),
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
                | "isEmpty"
                | "toList"
                | "union"
                | "intersection"
                | "difference"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "from")
    }
}
