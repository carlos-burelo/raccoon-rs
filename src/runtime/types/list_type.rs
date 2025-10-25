use super::{CallbackExecutor, TypeHandler};
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::{BoolValue, IntValue, ListValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct ListType;

#[async_trait]
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
            "push"
                | "pop"
                | "concat"
                | "length"
                | "len"
                | "reverse"
                | "clear"
                | "toStr"
                | "map"
                | "filter"
                | "reduce"
                | "forEach"
                | "find"
                | "findIndex"
                | "some"
                | "every"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }

    async fn call_async_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        callback_executor: &CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        let list = match value {
            RuntimeValue::List(l) => l,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected list, got {}", value.get_name()),
                    position,
                    file.clone(),
                ));
            }
        };

        match method {
            "map" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "map requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                let mut mapped = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;
                    mapped.push(result);
                }

                let element_type = if mapped.is_empty() {
                    PrimitiveType::any()
                } else {
                    mapped[0].get_type()
                };

                Ok(RuntimeValue::List(ListValue::new(mapped, element_type)))
            }

            "filter" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "filter requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                let mut filtered = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    let is_truthy = match result {
                        RuntimeValue::Bool(b) => b.value,
                        RuntimeValue::Null(_) => false,
                        RuntimeValue::Int(i) => i.value != 0,
                        RuntimeValue::Float(f) => f.value != 0.0,
                        RuntimeValue::Str(s) => !s.value.is_empty(),
                        _ => true,
                    };

                    if is_truthy {
                        filtered.push(element.clone());
                    }
                }

                Ok(RuntimeValue::List(ListValue::new(
                    filtered,
                    list.element_type.clone(),
                )))
            }

            "reduce" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "reduce requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                if list.elements.is_empty() && args.len() < 2 {
                    return Err(RaccoonError::new(
                        "reduce of empty array with no initial value".to_string(),
                        position,
                        file,
                    ));
                }

                let mut accumulator = if args.len() >= 2 {
                    args[1].clone()
                } else {
                    list.elements[0].clone()
                };

                let start_index = if args.len() >= 2 { 0 } else { 1 };

                for (index, element) in list.elements.iter().enumerate().skip(start_index) {
                    accumulator = callback_executor(
                        callback.clone(),
                        vec![
                            accumulator,
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;
                }

                Ok(accumulator)
            }

            "forEach" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "forEach requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }

            "find" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "find requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    let is_truthy = match result {
                        RuntimeValue::Bool(b) => b.value,
                        RuntimeValue::Null(_) => false,
                        RuntimeValue::Int(i) => i.value != 0,
                        RuntimeValue::Float(f) => f.value != 0.0,
                        RuntimeValue::Str(s) => !s.value.is_empty(),
                        _ => true,
                    };

                    if is_truthy {
                        return Ok(element.clone());
                    }
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }

            "findIndex" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "findIndex requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    let is_truthy = match result {
                        RuntimeValue::Bool(b) => b.value,
                        RuntimeValue::Null(_) => false,
                        RuntimeValue::Int(i) => i.value != 0,
                        RuntimeValue::Float(f) => f.value != 0.0,
                        RuntimeValue::Str(s) => !s.value.is_empty(),
                        _ => true,
                    };

                    if is_truthy {
                        return Ok(RuntimeValue::Int(IntValue::new(index as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }

            "some" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "some requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    let is_truthy = match result {
                        RuntimeValue::Bool(b) => b.value,
                        RuntimeValue::Null(_) => false,
                        RuntimeValue::Int(i) => i.value != 0,
                        RuntimeValue::Float(f) => f.value != 0.0,
                        RuntimeValue::Str(s) => !s.value.is_empty(),
                        _ => true,
                    };

                    if is_truthy {
                        return Ok(RuntimeValue::Bool(BoolValue::new(true)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }

            "every" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "every requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = callback_executor(
                        callback.clone(),
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    let is_truthy = match result {
                        RuntimeValue::Bool(b) => b.value,
                        RuntimeValue::Null(_) => false,
                        RuntimeValue::Int(i) => i.value != 0,
                        RuntimeValue::Float(f) => f.value != 0.0,
                        RuntimeValue::Str(s) => !s.value.is_empty(),
                        _ => true,
                    };

                    if !is_truthy {
                        return Ok(RuntimeValue::Bool(BoolValue::new(false)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }

            _ => self.call_instance_method(value, method, args, position, file),
        }
    }

    fn has_async_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "map" | "filter" | "reduce" | "forEach" | "find" | "findIndex" | "some" | "every"
        )
    }
}
