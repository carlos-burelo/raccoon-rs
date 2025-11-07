use crate::runtime::types::{CallbackExecutor, TypeHandler};
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

            "indexOf" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "indexOf requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                for (i, elem) in list.elements.iter().enumerate() {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Int(IntValue::new(i as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }

            "includes" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "includes requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                for elem in &list.elements {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(true)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(false)))
            }

            "shift" => {
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
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "unshift requires at least 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                for (i, arg) in args.iter().enumerate() {
                    list.elements.insert(i, arg.clone());
                }
                Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64)))
            }

            "splice" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "splice requires at least 1 argument (start index)".to_string(),
                        position,
                        file,
                    ));
                }
                let start = match &args[0] {
                    RuntimeValue::Int(i) => i.value.max(0) as usize,
                    _ => {
                        return Err(RaccoonError::new(
                            "splice start must be an integer".to_string(),
                            position,
                            file,
                        ));
                    }
                };

                let delete_count = if args.len() > 1 {
                    match &args[1] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => {
                            return Err(RaccoonError::new(
                                "splice deleteCount must be an integer".to_string(),
                                position,
                                file,
                            ));
                        }
                    }
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

                // Insert new elements
                for (i, arg) in args.iter().skip(2).enumerate() {
                    list.elements.insert(actual_start + i, arg.clone());
                }

                Ok(RuntimeValue::List(ListValue::new(
                    removed,
                    list.element_type.clone(),
                )))
            }

            "at" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "at requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                let index = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "at requires integer argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };

                let len = list.elements.len() as i64;
                let actual_index = if index < 0 { len + index } else { index };

                if actual_index < 0 || actual_index >= len {
                    Ok(RuntimeValue::Null(NullValue::new()))
                } else {
                    Ok(list.elements[actual_index as usize].clone())
                }
            }

            "fill" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "fill requires at least 1 argument (value)".to_string(),
                        position,
                        file,
                    ));
                }

                let value = &args[0];
                let start = if args.len() > 1 {
                    match &args[1] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let end = if args.len() > 2 {
                    match &args[2] {
                        RuntimeValue::Int(i) => (i.value as usize).min(list.elements.len()),
                        _ => list.elements.len(),
                    }
                } else {
                    list.elements.len()
                };

                for i in start..end.min(list.elements.len()) {
                    list.elements[i] = value.clone();
                }

                Ok(RuntimeValue::Null(NullValue::new()))
            }

            "flat" => {
                let depth = if args.is_empty() {
                    1
                } else {
                    match &args[0] {
                        RuntimeValue::Int(i) => i.value.max(0) as usize,
                        _ => 1,
                    }
                };

                fn flatten_recursive(
                    elements: &[RuntimeValue],
                    depth: usize,
                ) -> Vec<RuntimeValue> {
                    if depth == 0 {
                        return elements.to_vec();
                    }

                    let mut result = Vec::new();
                    for elem in elements {
                        match elem {
                            RuntimeValue::List(inner_list) => {
                                result.extend(flatten_recursive(&inner_list.elements, depth - 1));
                            }
                            _ => result.push(elem.clone()),
                        }
                    }
                    result
                }

                let flattened = flatten_recursive(&list.elements, depth);
                Ok(RuntimeValue::List(ListValue::new(
                    flattened,
                    PrimitiveType::any(),
                )))
            }

            "slice" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(RaccoonError::new(
                        "slice requires 1 or 2 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                let start = match &args[0] {
                    RuntimeValue::Int(i) => i.value as isize,
                    _ => {
                        return Err(RaccoonError::new(
                            "slice requires integer arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let end = if args.len() == 2 {
                    match &args[1] {
                        RuntimeValue::Int(i) => Some(i.value as isize),
                        _ => {
                            return Err(RaccoonError::new(
                                "slice requires integer arguments".to_string(),
                                position,
                                file,
                            ));
                        }
                    }
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
                    Ok(RuntimeValue::List(ListValue::new(
                        list.elements[real_start..real_end].to_vec(),
                        list.element_type.clone(),
                    )))
                } else {
                    Ok(RuntimeValue::List(ListValue::new(
                        vec![],
                        list.element_type.clone(),
                    )))
                }
            }

            "join" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "join requires 1 argument (separator)".to_string(),
                        position,
                        file,
                    ));
                }
                let separator = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "join requires string argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let parts: Vec<String> = list.elements.iter().map(|v| v.to_string()).collect();
                Ok(RuntimeValue::Str(StrValue::new(parts.join(separator))))
            }

            "isEmpty" => Ok(RuntimeValue::Bool(BoolValue::new(list.elements.is_empty()))),

            "first" => {
                if let Some(elem) = list.elements.first() {
                    Ok(elem.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }

            "last" => {
                if let Some(elem) = list.elements.last() {
                    Ok(elem.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }

            "lastIndexOf" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "lastIndexOf requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                for (i, elem) in list.elements.iter().enumerate().rev() {
                    if elem.equals(&args[0]) {
                        return Ok(RuntimeValue::Int(IntValue::new(i as i64)));
                    }
                }
                Ok(RuntimeValue::Int(IntValue::new(-1)))
            }

            "unique" => {
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
                Ok(RuntimeValue::List(ListValue::new(
                    unique_elements,
                    list.element_type.clone(),
                )))
            }

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
                | "indexOf"
                | "lastIndexOf"
                | "includes"
                | "shift"
                | "unshift"
                | "splice"
                | "at"
                | "fill"
                | "flat"
                | "flatMap"
                | "slice"
                | "join"
                | "isEmpty"
                | "first"
                | "last"
                | "unique"
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

            "flatMap" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "flatMap requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

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

                    // Flatten the result if it's a list
                    match result {
                        RuntimeValue::List(inner_list) => {
                            result_elements.extend(inner_list.elements);
                        }
                        _ => result_elements.push(result),
                    }
                }

                Ok(RuntimeValue::List(ListValue::new(
                    result_elements,
                    PrimitiveType::any(),
                )))
            }

            _ => self.call_instance_method(value, method, args, position, file),
        }
    }

    fn has_async_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "map" | "filter" | "reduce" | "forEach" | "find" | "findIndex" | "some" | "every" | "flatMap"
        )
    }
}
