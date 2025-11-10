use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ArrayValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// ObjectType - Object literal type handler
// ============================================================================

pub struct ObjectType;

#[async_trait]
impl TypeHandler for ObjectType {
    fn type_name(&self) -> &str {
        "object"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
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
                let keys: Vec<RuntimeValue> = obj
                    .properties
                    .keys()
                    .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    keys,
                    PrimitiveType::str(),
                )))
            }
            "values" => {
                let values: Vec<RuntimeValue> = obj.properties.values().cloned().collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    values,
                    PrimitiveType::any(),
                )))
            }
            "entries" => {
                let entries: Vec<RuntimeValue> = obj
                    .properties
                    .iter()
                    .map(|(k, v)| {
                        let pair = vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()];
                        RuntimeValue::Array(ArrayValue::new(pair, PrimitiveType::any()))
                    })
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    entries,
                    PrimitiveType::any(),
                )))
            }
            "size" | "length" => Ok(RuntimeValue::Int(
                IntValue::new(obj.properties.len() as i64),
            )),
            "hasOwnProperty" => {
                if _args.len() != 1 {
                    return Err(RaccoonError::new(
                        "hasOwnProperty requires 1 argument (key)".to_string(),
                        position,
                        file,
                    ));
                }
                match &_args[0] {
                    RuntimeValue::Str(s) => {
                        let has_prop = obj.properties.contains_key(&s.value);
                        Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(has_prop)))
                    }
                    _ => Err(RaccoonError::new(
                        "hasOwnProperty requires string argument".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "toString" | "toStr" => Ok(RuntimeValue::Str(StrValue::new(obj.to_string()))),
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on object", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!("Static method '{}' not found on object type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "keys"
                | "values"
                | "entries"
                | "size"
                | "length"
                | "hasOwnProperty"
                | "toString"
                | "toStr"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
