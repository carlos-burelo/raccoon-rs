use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::{CallbackExecutor, TypeHandler};
use crate::runtime::{ArrayValue, BoolValue, IntValue, NullValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct ArrayType;

impl ArrayType {
    fn extract_array_mut<'a>(
        value: &'a mut RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<&'a mut ArrayValue, RaccoonError> {
        match value {
            RuntimeValue::Array(a) => Ok(a),
            _ => Err(RaccoonError::new(
                format!("Expected list, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

#[async_trait]
impl TypeHandler for ArrayType {
    fn type_name(&self) -> &str {
        "array"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let list = Self::extract_array_mut(value, position, file.clone())?;

        match method {
            "push" => {
                require_args(&args, 1, method, position, file)?;
                list.elements.push(args[0].clone());
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "pop" => {
                require_args(&args, 0, method, position, file.clone())?;
                list.elements.pop().ok_or_else(|| {
                    RaccoonError::new("Cannot pop from empty list".to_string(), position, file)
                })
            }
            "shift" => {
                require_args(&args, 0, method, position, file.clone())?;
                if list.elements.is_empty() {
                    return Err(RaccoonError::new(
                        "Cannot shift from empty list".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(list.elements.remove(0))
            }
            "unshift" => {
                require_min_args(&args, 1, method, position, file)?;
                for (i, arg) in args.iter().enumerate() {
                    list.elements.insert(i, arg.clone());
                }
                Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64)))
            }
            "clear" => {
                require_args(&args, 0, method, position, file)?;
                list.elements.clear();
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            "splice" => {
                require_min_args(&args, 1, method, position, file.clone())?;
                let start = extract_int(&args[0], "start", position, file.clone())?.max(0) as usize;
                let delete_count = if args.len() > 1 {
                    extract_int(&args[1], "deleteCount", position, file.clone())?.max(0) as usize
                } else {
                    list.elements.len() - start
                };

                let mut removed = Vec::new();
                let actual_start = start.min(list.elements.len());
                let actual_count = delete_count.min(list.elements.len() - actual_start);

                for _ in 0..actual_count {
                    if actual_start < list.elements.len() {
                        removed.push(list.elements.remove(actual_start));
                    }
                }

                for (i, arg) in args.iter().skip(2).enumerate() {
                    list.elements.insert(actual_start + i, arg.clone());
                }

                Ok(RuntimeValue::Array(ArrayValue::new(
                    removed,
                    list.element_type.clone(),
                )))
            }
            "fill" => {
                require_min_args(&args, 1, method, position, file.clone())?;
                let value = &args[0];
                let start = if args.len() > 1 {
                    extract_int(&args[1], "start", position, file.clone())?.max(0) as usize
                } else {
                    0
                };
                let end = if args.len() > 2 {
                    extract_int(&args[2], "end", position, file)?.max(0) as usize
                } else {
                    list.elements.len()
                };

                for i in start..end.min(list.elements.len()) {
                    list.elements[i] = value.clone();
                }

                Ok(RuntimeValue::Null(NullValue::new()))
            }

            "concat" => {
                require_args(&args, 1, method, position, file.clone())?;
                let other = extract_array(&args[0], "other", position, file)?;
                let mut combined = list.elements.clone();
                combined.extend(other.elements.clone());
                Ok(RuntimeValue::Array(ArrayValue::new(
                    combined,
                    list.element_type.clone(),
                )))
            }
            "reverse" => {
                require_args(&args, 0, method, position, file)?;
                let mut reversed = list.elements.clone();
                reversed.reverse();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    reversed,
                    list.element_type.clone(),
                )))
            }
            "slice" => {
                require_args_range(&args, 1, 2, method, position, file.clone())?;
                let start = extract_int(&args[0], "start", position, file.clone())? as isize;
                let end = if args.len() == 2 {
                    Some(extract_int(&args[1], "end", position, file)? as isize)
                } else {
                    None
                };

                let len = list.elements.len() as isize;
                let real_start = if start < 0 {
                    (len + start).max(0) as usize
                } else {
                    (start as usize).min(list.elements.len())
                };
                let real_end = match end {
                    Some(e) => {
                        if e < 0 {
                            (len + e).max(0) as usize
                        } else {
                            (e as usize).min(list.elements.len())
                        }
                    }
                    None => list.elements.len(),
                };

                if real_start <= real_end {
                    Ok(RuntimeValue::Array(ArrayValue::new(
                        list.elements[real_start..real_end].to_vec(),
                        list.element_type.clone(),
                    )))
                } else {
                    Ok(RuntimeValue::Array(ArrayValue::new(
                        vec![],
                        list.element_type.clone(),
                    )))
                }
            }
            "flat" => {
                let depth = if args.is_empty() {
                    1
                } else {
                    extract_int(&args[0], "depth", position, file)?.max(0) as usize
                };

                fn flatten_recursive(elements: &[RuntimeValue], depth: usize) -> Vec<RuntimeValue> {
                    if depth == 0 {
                        return elements.to_vec();
                    }

                    let mut result = Vec::new();
                    for elem in elements {
                        match elem {
                            RuntimeValue::Array(inner_list) => {
                                result.extend(flatten_recursive(&inner_list.elements, depth - 1));
                            }
                            _ => result.push(elem.clone()),
                        }
                    }
                    result
                }

                let flattened = flatten_recursive(&list.elements, depth);
                Ok(RuntimeValue::Array(ArrayValue::new(
                    flattened,
                    PrimitiveType::any(),
                )))
            }
            "unique" => {
                require_args(&args, 0, method, position, file)?;
                let mut unique_elements = Vec::new();
                for elem in &list.elements {
                    let mut is_unique = true;
                    for unique_elem in &unique_elements {
                        if elem.equals(unique_elem) {
                            is_unique = false;
                            break;
                        }
                    }
                    if is_unique {
                        unique_elements.push(elem.clone());
                    }
                }
                Ok(RuntimeValue::Array(ArrayValue::new(
                    unique_elements,
                    list.element_type.clone(),
                )))
            }

            "indexOf" => {
                require_args(&args, 1, method, position, file)?;
                for (i, elem) in list.elements.iter().enumerate() {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Int(IntValue::new(i as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }
            "lastIndexOf" => {
                require_args(&args, 1, method, position, file)?;
                for (i, elem) in list.elements.iter().enumerate().rev() {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Int(IntValue::new(i as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }
            "includes" => {
                require_args(&args, 1, method, position, file)?;
                for elem in &list.elements {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(true)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }
            "at" => {
                require_args(&args, 1, method, position, file.clone())?;
                let index = extract_int(&args[0], "index", position, file)?;
                let len = list.elements.len() as i64;
                let actual_index = if index < 0 { len + index } else { index };

                if actual_index < 0 || actual_index >= len {
                    Ok(RuntimeValue::Null(NullValue::new()))
                } else {
                    Ok(list.elements[actual_index as usize].clone())
                }
            }

            "first" => {
                require_args(&args, 0, method, position, file)?;
                Ok(list
                    .elements
                    .first()
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())))
            }
            "last" => {
                require_args(&args, 0, method, position, file)?;
                Ok(list
                    .elements
                    .last()
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())))
            }

            "length" | "len" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64)))
            }
            "isEmpty" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(list.elements.is_empty())))
            }

            "join" => {
                require_args(&args, 1, method, position, file.clone())?;
                let separator = extract_str(&args[0], "separator", position, file)?;
                let parts: Vec<String> = list.elements.iter().map(|v| v.to_string()).collect();
                Ok(RuntimeValue::Str(StrValue::new(parts.join(separator))))
            }
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(list.to_string())))
            }

            _ => Err(method_not_found_error("array", method, position, file)),
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
            "array", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "push"
                | "pop"
                | "shift"
                | "unshift"
                | "clear"
                | "splice"
                | "fill"
                | "concat"
                | "reverse"
                | "slice"
                | "flat"
                | "unique"
                | "indexOf"
                | "lastIndexOf"
                | "includes"
                | "at"
                | "first"
                | "last"
                | "length"
                | "len"
                | "isEmpty"
                | "join"
                | "toStr"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }

    fn has_async_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "map"
                | "filter"
                | "reduce"
                | "forEach"
                | "find"
                | "findIndex"
                | "some"
                | "every"
                | "flatMap"
        )
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
        let list = Self::extract_array_mut(value, position, file.clone())?;

        match method {
            "map" => {
                require_args(&args, 1, method, position, file)?;
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

                Ok(RuntimeValue::Array(ArrayValue::new(mapped, element_type)))
            }

            "filter" => {
                require_args(&args, 1, method, position, file)?;
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

                    if to_truthy(&result) {
                        filtered.push(element.clone());
                    }
                }

                Ok(RuntimeValue::Array(ArrayValue::new(
                    filtered,
                    list.element_type.clone(),
                )))
            }

            "reduce" => {
                require_args(&args, 1, method, position, file.clone())?;
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
                require_args(&args, 1, method, position, file)?;
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
                require_args(&args, 1, method, position, file)?;
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

                    if to_truthy(&result) {
                        return Ok(element.clone());
                    }
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }

            "findIndex" => {
                require_args(&args, 1, method, position, file)?;
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

                    if to_truthy(&result) {
                        return Ok(RuntimeValue::Int(IntValue::new(index as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }

            "some" => {
                require_args(&args, 1, method, position, file)?;
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

                    if to_truthy(&result) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(true)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }

            "every" => {
                require_args(&args, 1, method, position, file)?;
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

                    if !to_truthy(&result) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(false)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }

            "flatMap" => {
                require_args(&args, 1, method, position, file)?;
                let callback = &args[0];
                let mut result_elements = Vec::new();

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

                    match result {
                        RuntimeValue::Array(inner_list) => {
                            result_elements.extend(inner_list.elements);
                        }
                        _ => result_elements.push(result),
                    }
                }

                Ok(RuntimeValue::Array(ArrayValue::new(
                    result_elements,
                    PrimitiveType::any(),
                )))
            }

            _ => self.call_instance_method(value, method, args, position, file),
        }
    }
}
