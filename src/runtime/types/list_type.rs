use super::TypeHandler;
use crate::error::RaccoonError;
use crate::runtime::{IntValue, ListValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;

pub struct ListType;

impl TypeHandler for ListType {
    fn type_name(&self) -> &str {
        "list"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let list = match value {
            RuntimeValue::List(l) => l,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected list, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "push" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "push requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                list.elements.push(args[0].clone());
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "pop" => {
                if let Some(element) = list.elements.pop() {
                    Ok(element)
                } else {
                    Err(RaccoonError::new(
                        "Cannot pop from empty list".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "concat" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "concat requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::List(other) = &args[0] {
                    let mut combined = list.elements.clone();
                    combined.extend(other.elements.clone());
                    Ok(RuntimeValue::List(ListValue::new(
                        combined,
                        list.element_type.clone(),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "concat requires list argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "length" | "len" => Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64))),
            "reverse" => {
                let mut reversed = list.elements.clone();
                reversed.reverse();
                Ok(RuntimeValue::List(ListValue::new(
                    reversed,
                    list.element_type.clone(),
                )))
            }
            "clear" => {
                list.elements.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(list.to_string()))),

            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on list", method),
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
            format!("Static method '{}' not found on list type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "push" | "pop" | "concat" | "length" | "len" | "reverse" | "clear" | "toStr"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }
}
