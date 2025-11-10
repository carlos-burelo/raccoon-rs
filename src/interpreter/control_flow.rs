use crate::ast::nodes::*;
use crate::error::RaccoonError;
use crate::runtime::{NullValue, RuntimeValue, StrValue};
use async_recursion::async_recursion;

use super::{Interpreter, InterpreterResult};

pub struct ControlFlow;

impl ControlFlow {
    #[async_recursion(?Send)]
    pub async fn execute_block_internal(
        interpreter: &mut Interpreter,
        block: &Block,
    ) -> Result<InterpreterResult, RaccoonError> {
        interpreter.environment.push_scope();

        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &block.statements {
            match interpreter.execute_stmt_internal(stmt).await? {
                InterpreterResult::Value(v) => last_value = v,
                other => {
                    interpreter.environment.pop_scope();
                    return Ok(other);
                }
            }
        }

        interpreter.environment.pop_scope();

        Ok(InterpreterResult::Value(last_value))
    }

    #[async_recursion(?Send)]
    pub async fn execute_if_stmt_internal(
        interpreter: &mut Interpreter,
        if_stmt: &IfStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let condition = interpreter.evaluate_expr(&if_stmt.condition).await?;

        if interpreter.is_truthy(&condition) {
            interpreter
                .execute_stmt_internal(&if_stmt.then_branch)
                .await
        } else if let Some(else_branch) = &if_stmt.else_branch {
            interpreter.execute_stmt_internal(else_branch).await
        } else {
            Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )))
        }
    }

    #[async_recursion(?Send)]
    pub async fn execute_while_stmt(
        interpreter: &mut Interpreter,
        while_stmt: &WhileStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        loop {
            let condition = interpreter.evaluate_expr(&while_stmt.condition).await?;
            if !interpreter.is_truthy(&condition) {
                break;
            }

            match interpreter.execute_stmt_internal(&while_stmt.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => continue,
                InterpreterResult::Return(v) => return Ok(InterpreterResult::Return(v)),
            }
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    pub async fn execute_do_while_stmt(
        interpreter: &mut Interpreter,
        do_while_stmt: &DoWhileStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        loop {
            match interpreter
                .execute_stmt_internal(&do_while_stmt.body)
                .await?
            {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => {
                    let condition = interpreter.evaluate_expr(&do_while_stmt.condition).await?;
                    if !interpreter.is_truthy(&condition) {
                        break;
                    }
                    continue;
                }
                InterpreterResult::Return(v) => return Ok(InterpreterResult::Return(v)),
            }

            let condition = interpreter.evaluate_expr(&do_while_stmt.condition).await?;
            if !interpreter.is_truthy(&condition) {
                break;
            }
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    pub async fn execute_for_stmt(
        interpreter: &mut Interpreter,
        for_stmt: &ForStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        interpreter.environment.push_scope();

        if let Some(init) = &for_stmt.initializer {
            interpreter.execute_stmt_internal(init).await?;
        }

        loop {
            if let Some(condition) = &for_stmt.condition {
                let cond_value = interpreter.evaluate_expr(condition).await?;
                if !interpreter.is_truthy(&cond_value) {
                    break;
                }
            }

            match interpreter.execute_stmt_internal(&for_stmt.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => {}
                InterpreterResult::Return(v) => {
                    interpreter.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }

            if let Some(increment) = &for_stmt.increment {
                interpreter.evaluate_expr(increment).await?;
            }
        }

        interpreter.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    pub async fn execute_for_in_stmt(
        interpreter: &mut Interpreter,
        for_in: &ForInStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let iterable = interpreter.evaluate_expr(&for_in.iterable).await?;

        let elements = match iterable {
            RuntimeValue::Array(list) => list.elements,
            _ => {
                return Err(RaccoonError::new(
                    "For-in requires an iterable value".to_string(),
                    for_in.position,
                    interpreter.file.clone(),
                ));
            }
        };

        interpreter.environment.push_scope();

        if !elements.is_empty() {
            interpreter
                .environment
                .declare(for_in.variable.clone(), elements[0].clone())?;
        }

        for element in elements {
            interpreter
                .environment
                .assign(&for_in.variable, element, for_in.position)?;

            match interpreter.execute_stmt_internal(&for_in.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => continue,
                InterpreterResult::Return(v) => {
                    interpreter.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }
        }

        interpreter.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    pub async fn execute_for_of_stmt(
        interpreter: &mut Interpreter,
        for_of: &ForOfStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let iterable = interpreter.evaluate_expr(&for_of.iterable).await?;

        let elements = match iterable {
            RuntimeValue::Array(list) => list.elements,
            RuntimeValue::Str(s) => s
                .value
                .chars()
                .map(|c| RuntimeValue::Str(StrValue::new(c.to_string())))
                .collect(),
            _ => {
                return Err(RaccoonError::new(
                    "For-of requires an iterable value (array or string)".to_string(),
                    for_of.position,
                    interpreter.file.clone(),
                ));
            }
        };

        interpreter.environment.push_scope();

        if !elements.is_empty() {
            interpreter
                .environment
                .declare(for_of.variable.clone(), elements[0].clone())?;
        }

        for element in elements {
            interpreter
                .environment
                .assign(&for_of.variable, element, for_of.position)?;

            match interpreter.execute_stmt_internal(&for_of.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => continue,
                InterpreterResult::Return(v) => {
                    interpreter.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }
        }

        interpreter.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    pub async fn execute_switch_stmt(
        interpreter: &mut Interpreter,
        switch_stmt: &SwitchStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let discriminant_value = interpreter.evaluate_expr(&switch_stmt.discriminant).await?;

        let mut matched = false;
        let mut fall_through = false;

        for case in &switch_stmt.cases {
            if !fall_through {
                if let Some(test_expr) = &case.test {
                    let test_value = interpreter.evaluate_expr(test_expr).await?;
                    matched = discriminant_value.equals(&test_value);
                } else {
                    matched = true;
                }
            }

            if matched || fall_through {
                fall_through = true;

                for stmt in &case.consequent {
                    match interpreter.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(_) => {}
                        InterpreterResult::Break => {
                            return Ok(InterpreterResult::Value(RuntimeValue::Null(
                                NullValue::new(),
                            )));
                        }
                        InterpreterResult::Continue => {
                            return Err(RaccoonError::new(
                                "Continue not allowed in switch statement".to_string(),
                                switch_stmt.position,
                                interpreter.file.clone(),
                            ));
                        }
                        InterpreterResult::Return(v) => return Ok(InterpreterResult::Return(v)),
                    }
                }
            }
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    pub async fn execute_return_stmt(
        interpreter: &mut Interpreter,
        ret: &ReturnStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let value = if let Some(expr) = &ret.value {
            interpreter.evaluate_expr(expr).await?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        Ok(InterpreterResult::Return(value))
    }

    #[async_recursion(?Send)]
    pub async fn execute_try_stmt(
        interpreter: &mut Interpreter,
        try_stmt: &TryStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let result = Self::execute_block_internal(interpreter, &try_stmt.try_block).await;

        match result {
            Ok(value) => {
                if let Some(finally_block) = &try_stmt.finally_block {
                    Self::execute_block_internal(interpreter, finally_block).await?;
                }
                Ok(value)
            }
            Err(error) => {
                for catch_clause in &try_stmt.catch_clauses {
                    interpreter.environment.push_scope();
                    let error_value = RuntimeValue::Str(StrValue::new(error.message.clone()));
                    interpreter
                        .environment
                        .declare(catch_clause.error_var.clone(), error_value)?;

                    let result =
                        Self::execute_block_internal(interpreter, &catch_clause.body).await;
                    interpreter.environment.pop_scope();

                    if let Some(finally_block) = &try_stmt.finally_block {
                        Self::execute_block_internal(interpreter, finally_block).await?;
                    }

                    return result;
                }

                if let Some(finally_block) = &try_stmt.finally_block {
                    Self::execute_block_internal(interpreter, finally_block).await?;
                }

                Err(error)
            }
        }
    }
}
