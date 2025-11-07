use crate::ast::nodes::*;
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::{
    FutureValue, ListValue, NullValue, ObjectValue, RuntimeValue,
};
use crate::tokens::Position;
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::{Interpreter, InterpreterResult};

pub struct Helpers;

impl Helpers {
    #[async_recursion(?Send)]
    pub async fn destructure_pattern(
        interpreter: &mut Interpreter,
        pattern: &DestructuringPattern,
        value: &RuntimeValue,
        position: Position,
    ) -> Result<(), RaccoonError> {
        match pattern {
            DestructuringPattern::List(list_pattern) => {
                Self::destructure_list_pattern(interpreter, list_pattern, value, position).await
            }
            DestructuringPattern::Object(obj_pattern) => {
                Self::destructure_object_pattern(interpreter, obj_pattern, value, position).await
            }
        }
    }

    #[async_recursion(?Send)]
    pub async fn destructure_list_pattern(
        interpreter: &mut Interpreter,
        pattern: &ListPattern,
        value: &RuntimeValue,
        position: Position,
    ) -> Result<(), RaccoonError> {
        let elements = match value {
            RuntimeValue::List(list) => &list.elements,
            _ => {
                return Err(RaccoonError::new(
                    format!("Cannot destructure non-list value"),
                    position,
                    interpreter.file.clone(),
                ));
            }
        };

        let mut index = 0;
        for element_pattern in &pattern.elements {
            if let Some(elem_pat) = element_pattern {
                if index >= elements.len() {
                    return Err(RaccoonError::new(
                        format!("Not enough elements to destructure"),
                        position,
                        interpreter.file.clone(),
                    ));
                }

                match elem_pat {
                    ListPatternElement::Identifier(id) => {
                        interpreter.environment
                            .declare(id.name.clone(), elements[index].clone())?;
                    }
                    ListPatternElement::List(nested_list) => {
                        Self::destructure_list_pattern(interpreter, nested_list, &elements[index], position)
                            .await?;
                    }
                    ListPatternElement::Object(nested_obj) => {
                        Self::destructure_object_pattern(interpreter, nested_obj, &elements[index], position)
                            .await?;
                    }
                }
            }
            index += 1;
        }

        if let Some(rest) = &pattern.rest {
            let remaining: Vec<RuntimeValue> = elements[index..].to_vec();
            let rest_value = RuntimeValue::List(ListValue::new(remaining, PrimitiveType::any()));
            interpreter.environment
                .declare(rest.argument.name.clone(), rest_value)?;
        }

        Ok(())
    }

    #[async_recursion(?Send)]
    pub async fn destructure_object_pattern(
        interpreter: &mut Interpreter,
        pattern: &ObjectPattern,
        value: &RuntimeValue,
        position: Position,
    ) -> Result<(), RaccoonError> {
        for prop in &pattern.properties {
            let prop_value = match value {
                RuntimeValue::Object(obj) => obj
                    .properties
                    .get(&prop.key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())),
                RuntimeValue::Map(map) => map
                    .entries
                    .get(&prop.key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())),
                RuntimeValue::ClassInstance(inst) => inst
                    .properties
                    .read()
                    .unwrap()
                    .get(&prop.key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null(NullValue::new())),
                _ => {
                    return Err(RaccoonError::new(
                        format!("Cannot destructure non-object value"),
                        position,
                        interpreter.file.clone(),
                    ));
                }
            };

            match &prop.value {
                ObjectPatternValue::Identifier(id) => {
                    interpreter.environment.declare(id.name.clone(), prop_value)?;
                }
                ObjectPatternValue::List(nested_list) => {
                    Self::destructure_list_pattern(interpreter, nested_list, &prop_value, position)
                        .await?;
                }
                ObjectPatternValue::Object(nested_obj) => {
                    Self::destructure_object_pattern(interpreter, nested_obj, &prop_value, position)
                        .await?;
                }
            }
        }

        if let Some(rest) = &pattern.rest {
            let mut remaining = HashMap::new();
            match value {
                RuntimeValue::Object(obj) => {
                    for (key, val) in &obj.properties {
                        if !pattern.properties.iter().any(|p| p.key == *key) {
                            remaining.insert(key.clone(), val.clone());
                        }
                    }
                }
                RuntimeValue::Map(map) => {
                    for (key, val) in &map.entries {
                        if !pattern.properties.iter().any(|p| p.key == *key) {
                            remaining.insert(key.clone(), val.clone());
                        }
                    }
                }
                RuntimeValue::ClassInstance(inst) => {
                    for (key, val) in inst.properties.read().unwrap().iter() {
                        if !pattern.properties.iter().any(|p| p.key == *key) {
                            remaining.insert(key.clone(), val.clone());
                        }
                    }
                }
                _ => {}
            }
            let rest_value =
                RuntimeValue::Object(ObjectValue::new(remaining, PrimitiveType::any()));
            interpreter.environment
                .declare(rest.argument.name.clone(), rest_value)?;
        }

        Ok(())
    }

    #[async_recursion(?Send)]
    pub async fn call_function(
        interpreter: &mut Interpreter,
        func: &RuntimeValue,
        args: Vec<RuntimeValue>,
        position: Position,
    ) -> Result<RuntimeValue, RaccoonError> {
        match func {
            RuntimeValue::Function(fn_val) => {
                interpreter.environment.push_scope();

                for (i, param) in fn_val.parameters.iter().enumerate() {
                    let value = if i < args.len() {
                        args[i].clone()
                    } else if let Some(default_expr) = &param.default_value {
                        interpreter.evaluate_expr(default_expr).await?
                    } else {
                        interpreter.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter {}", i),
                            position,
                            interpreter.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            interpreter.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) =
                                Self::destructure_pattern(interpreter, pattern, &value, position).await
                            {
                                interpreter.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &fn_val.body {
                    match interpreter.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(v) => result = v,
                        InterpreterResult::Return(v) => {
                            interpreter.environment.pop_scope();
                            return Ok(v);
                        }
                        _ => {
                            interpreter.environment.pop_scope();
                            return Err(RaccoonError::new(
                                "Unexpected break/continue in function".to_string(),
                                position,
                                interpreter.file.clone(),
                            ));
                        }
                    }
                }

                interpreter.environment.pop_scope();
                Ok(result)
            }
            RuntimeValue::NativeFunction(fn_val) => Ok(fn_val.call(args)),
            RuntimeValue::NativeAsyncFunction(fn_val) => {
                let result = (fn_val.implementation)(args).await;
                let return_type = match &fn_val.fn_type {
                    crate::ast::types::Type::Function(fn_type) => fn_type.return_type.clone(),
                    _ => PrimitiveType::any(),
                };
                Ok(RuntimeValue::Future(FutureValue::new_resolved(
                    result,
                    return_type,
                )))
            }
            _ => Err(RaccoonError::new(
                "Expected a function".to_string(),
                position,
                interpreter.file.clone(),
            )),
        }
    }
}
