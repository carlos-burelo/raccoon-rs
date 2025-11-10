use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::{BoolValue, IntValue, ListValue, NullValue, RuntimeValue};
use crate::tokens::Position;
use async_recursion::async_recursion;

use super::helpers::Helpers;
use super::Interpreter;

pub struct Builtins;

impl Builtins {
    #[async_recursion(?Send)]
    pub async fn handle_list_functional_method(
        interpreter: &mut Interpreter,
        object: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
    ) -> Result<RuntimeValue, RaccoonError> {
        let list = match object {
            RuntimeValue::List(l) => l,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected list, got {}", object.get_name()),
                    position,
                    interpreter.file.clone(),
                ));
            }
        };

        match method {
            "map" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "map requires a callback function".to_string(),
                        position,
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                let mut mapped = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                let mut filtered = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    if interpreter.is_truthy(&result) {
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                if list.elements.is_empty() && args.len() < 2 {
                    return Err(RaccoonError::new(
                        "reduce of empty array with no initial value".to_string(),
                        position,
                        interpreter.file.clone(),
                    ));
                }

                let mut accumulator = if args.len() >= 2 {
                    args[1].clone()
                } else {
                    list.elements[0].clone()
                };

                let start_index = if args.len() >= 2 { 0 } else { 1 };

                for (index, element) in list.elements.iter().enumerate().skip(start_index) {
                    accumulator = Helpers::call_function(
                        interpreter,
                        callback,
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    Helpers::call_function(
                        interpreter,
                        callback,
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    if interpreter.is_truthy(&result) {
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    if interpreter.is_truthy(&result) {
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    if interpreter.is_truthy(&result) {
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
                        interpreter.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = Helpers::call_function(
                        interpreter,
                        callback,
                        vec![
                            element.clone(),
                            RuntimeValue::Int(IntValue::new(index as i64)),
                        ],
                        position,
                    )
                    .await?;

                    if !interpreter.is_truthy(&result) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(false)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on list", method),
                position,
                interpreter.file.clone(),
            )),
        }
    }
}
