/// ObjectType - Object literal type handler with metadata system
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ListValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ObjectType - Object literal type handler
// ============================================================================

pub struct ObjectType;

impl ObjectType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("object", "Object type for key-value data structures")
            .with_instance_methods(vec![
                MethodMetadata::new("keys", "list<str>", "Get list of object keys"),
                MethodMetadata::new("values", "list<any>", "Get list of object values"),
                MethodMetadata::new("entries", "list<list<any>>", "Get list of key-value pairs"),
                MethodMetadata::new("size", "int", "Get number of properties").with_alias("length"),
                MethodMetadata::new("hasOwnProperty", "bool", "Check if object has property")
                    .with_params(vec![ParamMetadata::new("key", "str")]),
                MethodMetadata::new("toString", "str", "Convert to string representation")
                    .with_alias("toStr"),
            ])
    }
}

#[async_trait]
impl TypeHandler for ObjectType {
    fn type_name(&self) -> &str {
        "object"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let obj = match value {
            RuntimeValue::Object(o) => o,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected object, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "keys" => {
                require_args(&args, 0, method, position, file)?;
                let keys: Vec<RuntimeValue> = obj
                    .properties
                    .keys()
                    .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                    .collect();
                Ok(RuntimeValue::List(ListValue::new(
                    keys,
                    PrimitiveType::str(),
                )))
            }
            "values" => {
                require_args(&args, 0, method, position, file)?;
                let values: Vec<RuntimeValue> = obj.properties.values().cloned().collect();
                Ok(RuntimeValue::List(ListValue::new(
                    values,
                    PrimitiveType::any(),
                )))
            }
            "entries" => {
                require_args(&args, 0, method, position, file)?;
                let entries: Vec<RuntimeValue> = obj
                    .properties
                    .iter()
                    .map(|(k, v)| {
                        let pair = vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()];
                        RuntimeValue::List(ListValue::new(pair, PrimitiveType::any()))
                    })
                    .collect();
                Ok(RuntimeValue::List(ListValue::new(
                    entries,
                    PrimitiveType::any(),
                )))
            }
            "size" | "length" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(
                    IntValue::new(obj.properties.len() as i64),
                ))
            }
            "hasOwnProperty" => {
                require_args(&args, 1, method, position, file.clone())?;
                let key = extract_str(&args[0], "key", position, file)?;
                let has_prop = obj.properties.contains_key(key);
                Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(has_prop)))
            }
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(obj.to_string())))
            }
            _ => Err(method_not_found_error("object", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error(
            "object", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
