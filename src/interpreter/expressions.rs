use crate::ast::nodes::*;
use crate::ast::types::{PrimitiveType, Type};
use crate::error::RaccoonError;
use crate::runtime::*;
use crate::tokens::BinaryOperator;
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::builtins::Builtins;
use super::helpers::Helpers;
use super::operators;
use super::{Interpreter, InterpreterResult};

pub struct Expressions;

impl Expressions {
    #[async_recursion(?Send)]
    pub async fn evaluate_expr(
        interpreter: &mut Interpreter,
        expr: &Expr,
    ) -> Result<RuntimeValue, RaccoonError> {
        match expr {
            Expr::IntLiteral(lit) => Ok(RuntimeValue::Int(IntValue::new(lit.value))),
            Expr::BigIntLiteral(lit) => {
                // Parse BigInt value (remove 'n' suffix and underscores, handle different bases)
                let clean_value = lit.value.trim_end_matches('n').replace('_', "");
                let value = if clean_value.starts_with("0b") || clean_value.starts_with("0B") {
                    i128::from_str_radix(&clean_value[2..], 2).unwrap_or(0)
                } else if clean_value.starts_with("0o") || clean_value.starts_with("0O") {
                    i128::from_str_radix(&clean_value[2..], 8).unwrap_or(0)
                } else if clean_value.starts_with("0x") || clean_value.starts_with("0X") {
                    i128::from_str_radix(&clean_value[2..], 16).unwrap_or(0)
                } else {
                    clean_value.parse::<i128>().unwrap_or(0)
                };
                Ok(RuntimeValue::BigInt(BigIntValue::new(value)))
            }
            Expr::FloatLiteral(lit) => Ok(RuntimeValue::Float(FloatValue::new(lit.value))),
            Expr::StrLiteral(lit) => Ok(RuntimeValue::Str(StrValue::new(lit.value.clone()))),
            Expr::BoolLiteral(lit) => Ok(RuntimeValue::Bool(BoolValue::new(lit.value))),
            Expr::NullLiteral(_) => Ok(RuntimeValue::Null(NullValue::new())),
            Expr::Identifier(ident) => {
                // First check for builtin types (like Future, Promise)
                if let Some(builtin_type) = interpreter.get_builtin_type(&ident.name) {
                    return Ok(builtin_type);
                }
                // Otherwise look up in environment
                interpreter.environment.get(&ident.name, ident.position)
            }
            Expr::Binary(binary) => Self::evaluate_binary_expr(interpreter, binary).await,
            Expr::Unary(unary) => Self::evaluate_unary_expr(interpreter, unary).await,
            Expr::Assignment(assign) => Self::evaluate_assignment(interpreter, assign).await,
            Expr::Call(call) => Self::evaluate_call_expr(interpreter, call).await,
            Expr::ListLiteral(list) => Self::evaluate_list_literal(interpreter, list).await,
            Expr::ObjectLiteral(obj) => Self::evaluate_object_literal(interpreter, obj).await,
            Expr::Member(member) => Self::evaluate_member_expr(interpreter, member).await,
            Expr::Index(index) => Self::evaluate_index_expr(interpreter, index).await,
            Expr::Conditional(cond) => Self::evaluate_conditional_expr(interpreter, cond).await,
            Expr::UnaryUpdate(update) => Self::evaluate_unary_update(interpreter, update).await,
            Expr::TemplateStr(template) => Self::evaluate_template_str(interpreter, template).await,
            Expr::ArrowFn(arrow) => Self::evaluate_arrow_fn(interpreter, arrow).await,
            Expr::TypeOf(typeof_expr) => Self::evaluate_typeof_expr(interpreter, typeof_expr).await,
            Expr::InstanceOf(instanceof) => {
                Self::evaluate_instanceof_expr(interpreter, instanceof).await
            }
            Expr::OptionalChaining(opt_chain) => {
                Self::evaluate_optional_chaining(interpreter, opt_chain).await
            }
            Expr::NullAssertion(null_assert) => {
                Self::evaluate_null_assertion(interpreter, null_assert).await
            }
            Expr::MethodCall(method_call) => {
                Self::evaluate_method_call(interpreter, method_call).await
            }
            Expr::This(_) => Self::evaluate_this_expr(interpreter).await,
            Expr::Super(_) => Self::evaluate_super_expr(interpreter).await,
            Expr::New(new_expr) => Self::evaluate_new_expr(interpreter, new_expr).await,
            Expr::TaggedTemplate(tagged) => {
                Self::evaluate_tagged_template(interpreter, tagged).await
            }
            Expr::Range(range) => Self::evaluate_range_expr(interpreter, range).await,
            Expr::NullCoalescing(null_coal) => {
                Self::evaluate_null_coalescing(interpreter, null_coal).await
            }
            Expr::Await(await_expr) => Self::evaluate_await_expr(interpreter, await_expr).await,
            Expr::Spread(_) => Err(RaccoonError::new(
                "Spread operator cannot be used outside of function calls",
                (1, 1),
                None::<String>,
            )),
            Expr::Match(match_expr) => Self::evaluate_match_expr(interpreter, match_expr).await,
            Expr::Class(class_expr) => Self::evaluate_class_expr(interpreter, class_expr).await,
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_binary_expr(
        interpreter: &mut Interpreter,
        binary: &BinaryExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let left = Self::evaluate_expr(interpreter, &binary.left).await?;
        let right = Self::evaluate_expr(interpreter, &binary.right).await?;

        operators::apply_binary_operation(
            left,
            right,
            binary.operator,
            binary.position,
            &interpreter.file,
            &interpreter.call_stack,
            |v| interpreter.is_truthy(v),
        )
    }

    #[async_recursion(?Send)]
    async fn evaluate_unary_expr(
        interpreter: &mut Interpreter,
        unary: &UnaryExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let operand = Self::evaluate_expr(interpreter, &unary.operand).await?;

        operators::apply_unary_operation(
            operand,
            unary.operator,
            unary.position,
            &interpreter.file,
            |v| interpreter.is_truthy(v),
        )
    }

    #[async_recursion(?Send)]
    async fn evaluate_assignment(
        interpreter: &mut Interpreter,
        assign: &Assignment,
    ) -> Result<RuntimeValue, RaccoonError> {
        use crate::tokens::TokenType;

        let final_value = if assign.operator != TokenType::Assign {
            let current_value = Self::evaluate_expr(interpreter, &assign.target).await?;
            let right_value = Self::evaluate_expr(interpreter, &assign.value).await?;

            match assign.operator {
                TokenType::PlusAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Add,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::MinusAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Subtract,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::MultiplyAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Multiply,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::DivideAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Divide,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::ModuloAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Modulo,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::AmpersandAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseAnd,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::BitwiseOrAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseOr,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::BitwiseXorAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseXor,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::LeftShiftAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::LeftShift,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::RightShiftAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::RightShift,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::UnsignedRightShiftAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::UnsignedRightShift,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                TokenType::ExponentAssign => {
                    operators::apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Exponent,
                        assign.position,
                        &interpreter.file,
                        &interpreter.call_stack,
                    )
                    .await?
                }
                _ => {
                    return Err(RaccoonError::new(
                        format!(
                            "Unknown compound assignment operator: {:?}",
                            assign.operator
                        ),
                        assign.position,
                        interpreter.file.clone(),
                    ));
                }
            }
        } else {
            Self::evaluate_expr(interpreter, &assign.value).await?
        };

        match &*assign.target {
            Expr::Identifier(ident) => {
                interpreter
                    .environment
                    .assign(&ident.name, final_value.clone(), ident.position)?;
                Ok(final_value)
            }
            Expr::Member(member) => {
                let mut object = Self::evaluate_expr(interpreter, &member.object).await?;

                match &mut object {
                    RuntimeValue::Object(obj) => {
                        obj.properties
                            .insert(member.property.clone(), final_value.clone());

                        if let Expr::Identifier(ident) = &*member.object {
                            interpreter.environment.assign(
                                &ident.name,
                                object.clone(),
                                ident.position,
                            )?;
                        }

                        Ok(final_value)
                    }
                    RuntimeValue::ClassInstance(instance) => {
                        if let Some(accessor) = instance.accessors.iter().find(|a| {
                            a.name == member.property && matches!(a.kind, AccessorKind::Set)
                        }) {
                            interpreter.environment.push_scope();

                            interpreter.environment.declare(
                                "this".to_string(),
                                RuntimeValue::ClassInstance(instance.clone()),
                            )?;

                            if let Some(param) = accessor.parameters.first() {
                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        interpreter
                                            .environment
                                            .declare(name.clone(), final_value.clone())?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        Helpers::destructure_pattern(
                                            interpreter,
                                            pattern,
                                            &final_value,
                                            assign.position,
                                        )
                                        .await?;
                                    }
                                }
                            }

                            for stmt in &accessor.body {
                                match interpreter.execute_stmt_internal(stmt).await? {
                                    InterpreterResult::Return(_) => break,
                                    _ => {}
                                }
                            }

                            interpreter.environment.pop_scope();
                            return Ok(final_value);
                        }

                        instance
                            .properties
                            .write()
                            .unwrap()
                            .insert(member.property.clone(), final_value.clone());

                        if let Expr::Identifier(ident) = &*member.object {
                            interpreter.environment.assign(
                                &ident.name,
                                object.clone(),
                                ident.position,
                            )?;
                        }

                        Ok(final_value)
                    }
                    _ => Err(RaccoonError::new(
                        "Cannot assign to property of non-object".to_string(),
                        assign.position,
                        interpreter.file.clone(),
                    )),
                }
            }
            Expr::Index(index_expr) => {
                let mut object = Self::evaluate_expr(interpreter, &index_expr.object).await?;
                let idx = Self::evaluate_expr(interpreter, &index_expr.index).await?;

                match (&mut object, &idx) {
                    (RuntimeValue::List(list), RuntimeValue::Int(i)) => {
                        if i.value < 0 || i.value >= list.elements.len() as i64 {
                            return Err(RaccoonError::new(
                                format!("[1409] Index out of bounds: {}", i.value),
                                assign.position,
                                interpreter.file.clone(),
                            ));
                        }
                        list.elements[i.value as usize] = final_value.clone();
                    }
                    (RuntimeValue::Map(map), key) => {
                        map.entries.insert(key.to_string(), final_value.clone());
                    }
                    (RuntimeValue::Object(obj), RuntimeValue::Str(key)) => {
                        obj.properties
                            .insert(key.value.clone(), final_value.clone());
                    }
                    (RuntimeValue::ClassInstance(inst), RuntimeValue::Str(key)) => {
                        inst.properties
                            .write()
                            .unwrap()
                            .insert(key.value.clone(), final_value.clone());
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Invalid index assignment target".to_string(),
                            assign.position,
                            interpreter.file.clone(),
                        ));
                    }
                }

                if let Expr::Identifier(ident) = &*index_expr.object {
                    interpreter
                        .environment
                        .assign(&ident.name, object.clone(), ident.position)?;
                }

                Ok(final_value)
            }
            _ => Err(RaccoonError::new(
                "Invalid assignment target".to_string(),
                assign.position,
                interpreter.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_call_expr(
        interpreter: &mut Interpreter,
        call: &CallExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if interpreter.recursion_depth >= interpreter.max_recursion_depth {
            return Err(RaccoonError::with_call_stack(
                format!(
                    "Maximum recursion depth exceeded ({})",
                    interpreter.max_recursion_depth
                ),
                call.position,
                interpreter.file.clone(),
                interpreter.call_stack.clone(),
            ));
        }

        if matches!(call.callee.as_ref(), Expr::Super(_)) {
            return Self::evaluate_super_call(interpreter, &call.args).await;
        }

        let callee = Self::evaluate_expr(interpreter, &call.callee).await?;

        let mut args = Vec::new();
        for arg in &call.args {
            if let Expr::Spread(spread_expr) = arg {
                let spread_value = Self::evaluate_expr(interpreter, &spread_expr.argument).await?;

                if let RuntimeValue::List(list) = spread_value {
                    args.extend(list.elements.clone());
                } else {
                    print!("Spread operator can only be applied to arrays");
                    print!(" at position {:?}", spread_expr.position);
                    print!(" in file {:?}", interpreter.file.clone());
                    print!("\n");
                    println!("Got value: {:?}", spread_value);
                    println!("Full call expression: {:?}", call);

                    return Err(RaccoonError::new(
                        "Spread operator can only be applied to arrays",
                        spread_expr.position,
                        None::<String>,
                    ));
                }
            } else {
                args.push(Self::evaluate_expr(interpreter, arg).await?);
            }
        }

        let mut named_args = HashMap::new();
        for (name, expr) in &call.named_args {
            named_args.insert(name.clone(), Self::evaluate_expr(interpreter, expr).await?);
        }

        match callee {
            RuntimeValue::Function(func) => {
                interpreter.environment.push_scope();

                let mut positional_index = 0;

                for (i, param) in func.parameters.iter().enumerate() {
                    let param_name = match &param.pattern {
                        VarPattern::Identifier(name) => name.clone(),
                        VarPattern::Destructuring(_) => {
                            format!("__param_{}", i)
                        }
                    };

                    let value = if param.is_rest {
                        let mut rest_args = Vec::new();

                        while positional_index < args.len() {
                            rest_args.push(args[positional_index].clone());
                            positional_index += 1;
                        }

                        let element_type = match &param.param_type {
                            Type::List(list_type) => list_type.element_type.clone(),
                            _ => PrimitiveType::any(),
                        };

                        RuntimeValue::List(ListValue::new(rest_args, element_type))
                    } else if let Some(named_value) = named_args.get(&param_name) {
                        named_value.clone()
                    } else if positional_index < args.len() {
                        let current_arg = &args[positional_index];
                        let should_use_default =
                            if let VarPattern::Destructuring(DestructuringPattern::List(_)) =
                                &param.pattern
                            {
                                !matches!(current_arg, RuntimeValue::List(_))
                                    && param.default_value.is_some()
                            } else {
                                false
                            };

                        if should_use_default {
                            Self::evaluate_expr(interpreter, param.default_value.as_ref().unwrap())
                                .await?
                        } else {
                            let arg = args[positional_index].clone();
                            positional_index += 1;
                            arg
                        }
                    } else if let Some(default_expr) = &param.default_value {
                        Self::evaluate_expr(interpreter, default_expr).await?
                    } else if param.is_optional {
                        RuntimeValue::Null(crate::runtime::values::NullValue)
                    } else {
                        interpreter.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter '{}'", param_name),
                            call.position,
                            interpreter.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            interpreter.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) =
                                Helpers::destructure_pattern(interpreter, pattern, &value, (0, 0))
                                    .await
                            {
                                interpreter.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let is_async = func.is_async;
                let fn_type = func.fn_type.clone();
                let function_name = func
                    .name
                    .clone()
                    .unwrap_or_else(|| "<anonymous>".to_string());

                // For async functions, create a pending future and execute in spawn_local
                if is_async {
                    let return_type = match &fn_type {
                        Type::Function(fn_type) => fn_type.return_type.clone(),
                        _ => PrimitiveType::any(),
                    };

                    let future_value = FutureValue::new(return_type.clone());
                    let future_clone = future_value.clone();

                    // Clone the necessary data for the async task
                    let body = func.body.clone();
                    let file = interpreter.file.clone();
                    let type_registry = std::sync::Arc::clone(&interpreter.type_registry);
                    let stdlib_loader = std::sync::Arc::clone(&interpreter.stdlib_loader);
                    let decorator_registry = interpreter.decorator_registry.clone();
                    let registrar = interpreter.registrar.clone();
                    let module_registry = interpreter.module_registry.clone();
                    let max_recursion_depth = interpreter.max_recursion_depth;

                    // Snapshot the current environment (includes the function scope)
                    let env_snapshot = interpreter.environment.clone();

                    // Pop the scope in the parent interpreter
                    interpreter.environment.pop_scope();

                    // Spawn the async task
                    tokio::task::spawn_local(async move {
                        // Create a new interpreter context for this async function
                        let mut async_interpreter = Interpreter {
                            file,
                            environment: env_snapshot,
                            type_registry,
                            stdlib_loader,
                            recursion_depth: 0,
                            max_recursion_depth,
                            decorator_registry,
                            registrar,
                            module_registry,
                            call_stack: CallStack::new(),
                        };

                        let mut result = RuntimeValue::Null(NullValue::new());

                        // Execute the function body
                        for stmt in &body {
                            match async_interpreter.execute_stmt_internal(stmt).await {
                                Ok(InterpreterResult::Value(v)) => result = v,
                                Ok(InterpreterResult::Return(v)) => {
                                    future_clone.resolve(v);
                                    return;
                                }
                                Ok(_) => {
                                    future_clone.reject(
                                        "Unexpected break/continue in function".to_string(),
                                    );
                                    return;
                                }
                                Err(e) => {
                                    future_clone.reject(e.message);
                                    return;
                                }
                            }
                        }

                        future_clone.resolve(result);
                    });

                    return Ok(RuntimeValue::Future(future_value));
                }

                // Synchronous function execution (original logic)
                let stack_frame = crate::runtime::StackFrame::new(
                    function_name,
                    call.position,
                    interpreter.file.clone(),
                );
                interpreter.call_stack.push(stack_frame);
                interpreter.recursion_depth += 1;

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
                    match interpreter.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(v) => result = v,
                        InterpreterResult::Return(v) => {
                            interpreter.call_stack.pop();
                            interpreter.recursion_depth -= 1;
                            interpreter.environment.pop_scope();
                            return Ok(v);
                        }
                        _ => {
                            let stack = interpreter.call_stack.clone();
                            interpreter.call_stack.pop();
                            interpreter.recursion_depth -= 1;
                            interpreter.environment.pop_scope();
                            return Err(RaccoonError::with_call_stack(
                                "Unexpected break/continue in function".to_string(),
                                (0, 0),
                                interpreter.file.clone(),
                                stack,
                            ));
                        }
                    }
                }

                interpreter.call_stack.pop();
                interpreter.recursion_depth -= 1;
                interpreter.environment.pop_scope();
                Ok(result)
            }
            RuntimeValue::NativeFunction(func) => Ok((func.implementation)(args)),
            RuntimeValue::NativeAsyncFunction(func) => {
                let result = (func.implementation)(args).await;
                let return_type = match &func.fn_type {
                    Type::Function(fn_type) => fn_type.return_type.clone(),
                    _ => PrimitiveType::any(),
                };
                Ok(RuntimeValue::Future(FutureValue::new_resolved(
                    result,
                    return_type,
                )))
            }
            _ => Err(RaccoonError::new(
                "Attempted to call a non-function value".to_string(),
                (0, 0),
                interpreter.file.clone(),
            )),
        }
    }

    async fn evaluate_list_literal(
        interpreter: &mut Interpreter,
        list: &ListLiteral,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut elements = Vec::new();
        for elem in &list.elements {
            // Handle spread expressions in array literals
            if let Expr::Spread(spread) = elem {
                let spread_value = Self::evaluate_expr(interpreter, &spread.argument).await?;
                if let RuntimeValue::List(list_val) = spread_value {
                    elements.extend(list_val.elements);
                }
            } else {
                elements.push(Self::evaluate_expr(interpreter, elem).await?);
            }
        }

        let element_type = if elements.is_empty() {
            PrimitiveType::any()
        } else {
            elements[0].get_type()
        };

        Ok(RuntimeValue::List(ListValue::new(elements, element_type)))
    }

    async fn evaluate_object_literal(
        interpreter: &mut Interpreter,
        obj: &ObjectLiteral,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut properties = HashMap::new();

        for prop in &obj.properties {
            match prop {
                ObjectLiteralProperty::KeyValue { key, value } => {
                    properties.insert(key.clone(), Self::evaluate_expr(interpreter, value).await?);
                }
                ObjectLiteralProperty::Spread(expr) => {
                    // Evaluate the spread expression
                    let spread_value = Self::evaluate_expr(interpreter, expr).await?;
                    // Merge properties from the spread object
                    if let RuntimeValue::Object(ref obj_val) = spread_value {
                        for (k, v) in &obj_val.properties {
                            properties.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }

        Ok(RuntimeValue::Object(ObjectValue::new(
            properties,
            PrimitiveType::any(),
        )))
    }

    #[async_recursion(?Send)]
    async fn evaluate_member_expr(
        interpreter: &mut Interpreter,
        member: &MemberExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = Self::evaluate_expr(interpreter, &member.object).await?;

        match object {
            RuntimeValue::Class(class) => {
                if let Some(static_prop) = class.static_properties.get(&member.property) {
                    return Ok(static_prop.clone());
                }

                if let Some(static_method) = class.static_methods.get(&member.property) {
                    Ok(RuntimeValue::Function((**static_method).clone()))
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static member '{}' not found on class '{}'",
                            member.property, class.class_name
                        ),
                        member.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::Object(obj) => {
                if let Some(value) = obj.properties.get(&member.property) {
                    Ok(value.clone())
                } else {
                    Err(RaccoonError::new(
                        format!("Property '{}' not found", member.property),
                        member.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::ClassInstance(instance) => {
                if let Some(value) = instance.properties.read().unwrap().get(&member.property) {
                    return Ok(value.clone());
                }

                if let Some(accessor) = instance
                    .accessors
                    .iter()
                    .find(|a| a.name == member.property && matches!(a.kind, AccessorKind::Get))
                {
                    interpreter.environment.push_scope();

                    interpreter.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &accessor.body {
                        match interpreter.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Return(value) => {
                                result = value;
                                break;
                            }
                            _ => {}
                        }
                    }

                    interpreter.environment.pop_scope();
                    return Ok(result);
                }

                Err(RaccoonError::new(
                    format!("Property '{}' not found on class instance", member.property),
                    member.position,
                    interpreter.file.clone(),
                ))
            }
            RuntimeValue::Str(s) => match member.property.as_str() {
                "length" => Ok(RuntimeValue::Int(IntValue::new(s.value.len() as i64))),
                "isEmpty" => Ok(RuntimeValue::Bool(BoolValue::new(s.value.is_empty()))),
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on string", member.property),
                    member.position,
                    interpreter.file.clone(),
                )),
            },
            RuntimeValue::List(list) => match member.property.as_str() {
                "length" => Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64))),
                "first" => {
                    if list.elements.is_empty() {
                        Err(RaccoonError::new(
                            "List is empty".to_string(),
                            member.position,
                            interpreter.file.clone(),
                        ))
                    } else {
                        Ok(list.elements[0].clone())
                    }
                }
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on list", member.property),
                    member.position,
                    interpreter.file.clone(),
                )),
            },
            RuntimeValue::Map(map) => match member.property.as_str() {
                "size" => Ok(RuntimeValue::Int(IntValue::new(map.entries.len() as i64))),
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on map", member.property),
                    member.position,
                    interpreter.file.clone(),
                )),
            },
            RuntimeValue::EnumObject(enum_obj) => {
                if let Some(enum_value) = enum_obj.members.get(&member.property) {
                    Ok(RuntimeValue::Enum(EnumValue::new(
                        enum_obj.enum_name.clone(),
                        member.property.clone(),
                        enum_value.clone(),
                        enum_obj.enum_type.clone(),
                    )))
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Enum member '{}' not found on enum '{}'",
                            member.property, enum_obj.enum_name
                        ),
                        member.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::PrimitiveTypeObject(type_obj) => {
                if let Some(static_method) = type_obj.static_methods.get(&member.property) {
                    Ok(RuntimeValue::NativeFunction((**static_method).clone()))
                } else if let Some(static_prop) = type_obj.static_properties.get(&member.property) {
                    Ok(static_prop.clone())
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static member '{}' not found on type '{}'",
                            member.property, type_obj.type_name
                        ),
                        member.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::Type(type_obj) => {
                if let Some(static_method) = type_obj.get_static_method(&member.property) {
                    Ok(static_method.clone())
                } else if let Some(static_prop) = type_obj.get_static_property(&member.property) {
                    Ok(static_prop.clone())
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static member '{}' not found on type '{}'",
                            member.property,
                            type_obj.name()
                        ),
                        member.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            _ => Err(RaccoonError::new(
                format!("Cannot access property '{}' on type", member.property),
                member.position,
                interpreter.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_index_expr(
        interpreter: &mut Interpreter,
        index: &IndexExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = Self::evaluate_expr(interpreter, &index.object).await?;
        let idx = Self::evaluate_expr(interpreter, &index.index).await?;

        match (object, idx) {
            (RuntimeValue::List(list), RuntimeValue::Int(i)) => {
                if i.value < 0 || i.value >= list.elements.len() as i64 {
                    println!("Index value: {}", i.value);
                    println!("List length: {}", list.elements.len());
                    println!("List: {:?}", list.elements);

                    return Err(RaccoonError::new(
                        "[1614] Index out of bounds".to_string(),
                        index.position,
                        interpreter.file.clone(),
                    ));
                }
                Ok(list.elements[i.value as usize].clone())
            }
            (RuntimeValue::Str(s), RuntimeValue::Int(i)) => {
                let chars: Vec<char> = s.value.chars().collect();
                if i.value < 0 || i.value >= chars.len() as i64 {
                    return Err(RaccoonError::new(
                        format!(
                            "String index {} out of bounds (length: {})",
                            i.value,
                            chars.len()
                        ),
                        index.position,
                        interpreter.file.clone(),
                    ));
                }
                Ok(RuntimeValue::Str(StrValue::new(
                    chars[i.value as usize].to_string(),
                )))
            }
            (RuntimeValue::Map(map), RuntimeValue::Str(key)) => {
                if let Some(value) = map.entries.get(&key.value) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            (RuntimeValue::Object(obj), RuntimeValue::Str(key)) => {
                if let Some(value) = obj.properties.get(&key.value) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            (RuntimeValue::ClassInstance(inst), RuntimeValue::Str(key)) => {
                if let Some(value) = inst.properties.read().unwrap().get(&key.value) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            _ => Err(RaccoonError::new(
                "Invalid index operation".to_string(),
                index.position,
                interpreter.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_conditional_expr(
        interpreter: &mut Interpreter,
        cond: &ConditionalExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let condition = Self::evaluate_expr(interpreter, &cond.condition).await?;

        if interpreter.is_truthy(&condition) {
            Self::evaluate_expr(interpreter, &cond.then_expr).await
        } else {
            Self::evaluate_expr(interpreter, &cond.else_expr).await
        }
    }

    async fn evaluate_unary_update(
        interpreter: &mut Interpreter,
        update: &UnaryUpdateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let Expr::Identifier(ident) = &*update.operand {
            let current = interpreter.environment.get(&ident.name, ident.position)?;

            match current {
                RuntimeValue::Int(v) => {
                    let new_value = match update.operator {
                        UpdateOperator::Increment => v.value + 1,
                        UpdateOperator::Decrement => v.value - 1,
                    };

                    let new_runtime_value = RuntimeValue::Int(IntValue::new(new_value));
                    interpreter.environment.assign(
                        &ident.name,
                        new_runtime_value.clone(),
                        ident.position,
                    )?;

                    if update.is_prefix {
                        Ok(new_runtime_value)
                    } else {
                        Ok(RuntimeValue::Int(IntValue::new(v.value)))
                    }
                }
                RuntimeValue::Float(v) => {
                    let new_value = match update.operator {
                        UpdateOperator::Increment => v.value + 1.0,
                        UpdateOperator::Decrement => v.value - 1.0,
                    };

                    let new_runtime_value = RuntimeValue::Float(FloatValue::new(new_value));
                    interpreter.environment.assign(
                        &ident.name,
                        new_runtime_value.clone(),
                        ident.position,
                    )?;

                    if update.is_prefix {
                        Ok(new_runtime_value)
                    } else {
                        Ok(RuntimeValue::Float(FloatValue::new(v.value)))
                    }
                }
                _ => Err(RaccoonError::new(
                    "Invalid operand for update operator".to_string(),
                    update.position,
                    interpreter.file.clone(),
                )),
            }
        } else {
            Err(RaccoonError::new(
                "Invalid target for update operator".to_string(),
                update.position,
                interpreter.file.clone(),
            ))
        }
    }

    async fn evaluate_template_str(
        interpreter: &mut Interpreter,
        template: &TemplateStrExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut result = String::new();

        for part in &template.parts {
            match part {
                TemplateStrPart::String(s) => result.push_str(&s.value),
                TemplateStrPart::Expr(expr) => {
                    let value = Self::evaluate_expr(interpreter, expr).await?;
                    result.push_str(&value.to_string());
                }
            }
        }

        Ok(RuntimeValue::Str(StrValue::new(result)))
    }

    async fn evaluate_arrow_fn(
        _interpreter: &mut Interpreter,
        arrow: &ArrowFnExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let body = match &arrow.body {
            ArrowFnBody::Expr(expr) => {
                vec![Stmt::ReturnStmt(ReturnStmt {
                    value: Some((**expr).clone()),
                    position: arrow.position,
                })]
            }
            ArrowFnBody::Block(stmts) => stmts.clone(),
        };

        let return_type = arrow
            .return_type
            .clone()
            .unwrap_or(PrimitiveType::unknown());

        let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
            params: arrow
                .parameters
                .iter()
                .map(|p| p.param_type.clone())
                .collect(),
            return_type,
            is_variadic: arrow.parameters.iter().any(|p| p.is_rest),
        }));

        Ok(RuntimeValue::Function(FunctionValue::new(
            arrow.parameters.clone(),
            body,
            arrow.is_async,
            fn_type,
        )))
    }

    async fn evaluate_typeof_expr(
        interpreter: &mut Interpreter,
        typeof_expr: &TypeOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = Self::evaluate_expr(interpreter, &typeof_expr.operand).await?;

        let type_name = match value {
            RuntimeValue::Int(_) => "int",
            RuntimeValue::BigInt(_) => "bigint",
            RuntimeValue::Float(_) => "float",
            RuntimeValue::Decimal(_) => "decimal",
            RuntimeValue::Str(_) => "str",
            RuntimeValue::Bool(_) => "bool",
            RuntimeValue::Null(_) => "null",
            RuntimeValue::List(_) => "list",
            RuntimeValue::Map(_) => "map",
            RuntimeValue::Object(_) => "object",
            RuntimeValue::Class(ref c) => {
                return Ok(RuntimeValue::Str(StrValue::new(format!(
                    "class {}",
                    c.class_name
                ))));
            }
            RuntimeValue::ClassInstance(ref c) => {
                return Ok(RuntimeValue::Str(StrValue::new(c.class_name.clone())));
            }
            RuntimeValue::Function(_) => "function",
            RuntimeValue::NativeFunction(_) => "function",
            RuntimeValue::NativeAsyncFunction(_) => "function",
            RuntimeValue::Future(_) => "future",
            RuntimeValue::Enum(ref e) => {
                return Ok(RuntimeValue::Str(StrValue::new(e.enum_name.clone())));
            }
            RuntimeValue::EnumObject(ref e) => {
                return Ok(RuntimeValue::Str(StrValue::new(format!(
                    "enum {}",
                    e.enum_name
                ))));
            }
            RuntimeValue::PrimitiveTypeObject(ref p) => {
                return Ok(RuntimeValue::Str(StrValue::new(format!(
                    "type {}",
                    p.type_name
                ))));
            }
            RuntimeValue::Type(ref t) => {
                return Ok(RuntimeValue::Str(StrValue::new(format!(
                    "type {}",
                    t.name()
                ))));
            }
            RuntimeValue::Dynamic(d) => {
                return Ok(RuntimeValue::Str(StrValue::new(d.type_name().to_string())));
            }
        };

        Ok(RuntimeValue::Str(StrValue::new(type_name.to_string())))
    }

    async fn evaluate_instanceof_expr(
        interpreter: &mut Interpreter,
        instanceof: &InstanceOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = Self::evaluate_expr(interpreter, &instanceof.operand).await?;

        if let RuntimeValue::ClassInstance(instance) = value {
            Ok(RuntimeValue::Bool(BoolValue::new(
                instance.class_name == instanceof.type_name,
            )))
        } else {
            Ok(RuntimeValue::Bool(BoolValue::new(false)))
        }
    }

    async fn evaluate_optional_chaining(
        interpreter: &mut Interpreter,
        opt_chain: &OptionalChainingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = Self::evaluate_expr(interpreter, &opt_chain.object).await?;

        if matches!(object, RuntimeValue::Null(_)) {
            return Ok(RuntimeValue::Null(NullValue::new()));
        }

        match object {
            RuntimeValue::Object(obj) => {
                if let Some(value) = obj.properties.get(&opt_chain.property) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            RuntimeValue::ClassInstance(instance) => {
                if let Some(value) = instance.properties.read().unwrap().get(&opt_chain.property) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            _ => Ok(RuntimeValue::Null(NullValue::new())),
        }
    }

    async fn evaluate_null_assertion(
        interpreter: &mut Interpreter,
        null_assert: &NullAssertionExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = Self::evaluate_expr(interpreter, &null_assert.operand).await?;

        if matches!(value, RuntimeValue::Null(_)) {
            Err(RaccoonError::new(
                "Null assertion failed: value is null".to_string(),
                null_assert.position,
                interpreter.file.clone(),
            ))
        } else {
            Ok(value)
        }
    }

    async fn evaluate_this_expr(
        interpreter: &mut Interpreter,
    ) -> Result<RuntimeValue, RaccoonError> {
        match interpreter.environment.get("this", (0, 0)) {
            Ok(value) => Ok(value),
            Err(_) => Err(RaccoonError::new(
                "Cannot use 'this' outside of a class method".to_string(),
                (0, 0),
                interpreter.file.clone(),
            )),
        }
    }

    async fn evaluate_super_expr(
        interpreter: &mut Interpreter,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            "Cannot use 'super' outside of a class method".to_string(),
            (0, 0),
            interpreter.file.clone(),
        ))
    }

    async fn evaluate_super_call(
        interpreter: &mut Interpreter,
        args: &[Expr],
    ) -> Result<RuntimeValue, RaccoonError> {
        let this_instance = interpreter.environment.get("this", (0, 0))?;

        if let RuntimeValue::ClassInstance(instance) = this_instance {
            let class_name = instance.class_name.clone();

            let class_value = interpreter.environment.get(&class_name, (0, 0))?;

            // Extract ClassValue from either RuntimeValue::Class or RuntimeValue::Type
            let class = match &class_value {
                RuntimeValue::Class(c) => c.clone(),
                RuntimeValue::Type(type_obj) => {
                    if let Some(RuntimeValue::Class(c)) = type_obj.get_constructor() {
                        c.clone()
                    } else {
                        return Err(RaccoonError::new(
                            format!("Type '{}' does not have a valid constructor", class_name),
                            (0, 0),
                            interpreter.file.clone(),
                        ));
                    }
                }
                _ => {
                    return Err(RaccoonError::new(
                        format!("Class '{}' not found", class_name),
                        (0, 0),
                        interpreter.file.clone(),
                    ));
                }
            };

            if let Some(ref superclass_name) = class.declaration.superclass {
                let superclass_value = interpreter.environment.get(superclass_name, (0, 0))?;

                // Extract superclass ClassValue from either RuntimeValue::Class or RuntimeValue::Type
                let superclass = match &superclass_value {
                    RuntimeValue::Class(sc) => sc.clone(),
                    RuntimeValue::Type(type_obj) => {
                        if let Some(RuntimeValue::Class(sc)) = type_obj.get_constructor() {
                            sc.clone()
                        } else {
                            return Err(RaccoonError::new(
                                format!(
                                    "Superclass '{}' does not have a valid constructor",
                                    superclass_name
                                ),
                                (0, 0),
                                interpreter.file.clone(),
                            ));
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            format!("'{}' is not a class", superclass_name),
                            (0, 0),
                            interpreter.file.clone(),
                        ));
                    }
                };

                if let Some(ref super_constructor) = superclass.declaration.constructor {
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(Self::evaluate_expr(interpreter, arg).await?);
                    }

                    interpreter.environment.push_scope();

                    for (param, arg) in super_constructor.parameters.iter().zip(arg_values.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                interpreter.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) =
                                    Helpers::destructure_pattern(interpreter, pattern, arg, (0, 0))
                                        .await
                                {
                                    interpreter.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    interpreter.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    for stmt in &super_constructor.body {
                        if let Stmt::ExprStmt(expr_stmt) = stmt {
                            if let Expr::Assignment(assign) = &expr_stmt.expression {
                                if let Expr::Member(member) = &*assign.target {
                                    if let Expr::This(_) = &*member.object {
                                        let value =
                                            Self::evaluate_expr(interpreter, &assign.value).await?;
                                        instance
                                            .properties
                                            .write()
                                            .unwrap()
                                            .insert(member.property.clone(), value);
                                        continue;
                                    }
                                }
                            }
                        }

                        match interpreter.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Return(_) => break,
                            _ => {}
                        }
                    }

                    interpreter.environment.pop_scope();

                    return Ok(RuntimeValue::Null(NullValue::new()));
                } else {
                    return Err(RaccoonError::new(
                        format!("Superclass '{}' has no constructor", superclass_name),
                        (0, 0),
                        interpreter.file.clone(),
                    ));
                }
            } else {
                return Err(RaccoonError::new(
                    "Cannot use 'super' in class without superclass".to_string(),
                    (0, 0),
                    interpreter.file.clone(),
                ));
            }
        } else {
            return Err(RaccoonError::new(
                "Cannot use 'super' outside of a class constructor".to_string(),
                (0, 0),
                interpreter.file.clone(),
            ));
        }
    }

    async fn evaluate_range_expr(
        interpreter: &mut Interpreter,
        range: &RangeExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let start = Self::evaluate_expr(interpreter, &range.start).await?;
        let end = Self::evaluate_expr(interpreter, &range.end).await?;

        match (start, end) {
            (RuntimeValue::Int(s), RuntimeValue::Int(e)) => {
                let mut elements = Vec::new();
                for i in s.value..=e.value {
                    elements.push(RuntimeValue::Int(IntValue::new(i)));
                }
                Ok(RuntimeValue::List(ListValue::new(
                    elements,
                    PrimitiveType::int(),
                )))
            }
            _ => Err(RaccoonError::new(
                "Range operator requires integer operands".to_string(),
                range.position,
                interpreter.file.clone(),
            )),
        }
    }

    async fn evaluate_null_coalescing(
        interpreter: &mut Interpreter,
        null_coal: &NullCoalescingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let left = Self::evaluate_expr(interpreter, &null_coal.left).await?;

        if matches!(left, RuntimeValue::Null(_)) {
            Self::evaluate_expr(interpreter, &null_coal.right).await
        } else {
            Ok(left)
        }
    }

    async fn evaluate_new_expr(
        interpreter: &mut Interpreter,
        new_expr: &NewExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if new_expr.class_name == "Map" {
            if new_expr.type_args.len() != 2 {
                return Err(RaccoonError::new(
                    "Map requires exactly two type arguments".to_string(),
                    new_expr.position,
                    interpreter.file.clone(),
                ));
            }

            let key_type = new_expr.type_args[0].clone();
            let value_type = new_expr.type_args[1].clone();

            return Ok(RuntimeValue::Map(crate::runtime::MapValue::new(
                HashMap::new(),
                key_type,
                value_type,
            )));
        }

        let class_value = interpreter
            .environment
            .get(&new_expr.class_name, new_expr.position)?;

        // Extract ClassValue from either RuntimeValue::Class or RuntimeValue::Type
        let class = match &class_value {
            RuntimeValue::Class(c) => c.clone(),
            RuntimeValue::Type(type_obj) => {
                if let Some(RuntimeValue::Class(c)) = type_obj.get_constructor() {
                    c.clone()
                } else {
                    return Err(RaccoonError::new(
                        format!(
                            "Type '{}' does not have a valid constructor",
                            new_expr.class_name
                        ),
                        new_expr.position,
                        interpreter.file.clone(),
                    ));
                }
            }
            _ => {
                return Err(RaccoonError::new(
                    format!(
                        "Class '{}' not found or not yet implemented",
                        new_expr.class_name
                    ),
                    new_expr.position,
                    interpreter.file.clone(),
                ));
            }
        };

        // Now proceed with class instantiation
        {
            let mut properties = HashMap::new();
            let mut methods = HashMap::new();

            if let Some(ref superclass_name) = class.declaration.superclass {
                let superclass_value = interpreter
                    .environment
                    .get(superclass_name, new_expr.position)
                    .ok();
                let superclass = match superclass_value {
                    Some(RuntimeValue::Class(sc)) => Some(sc),
                    Some(RuntimeValue::Type(type_obj)) => {
                        if let Some(RuntimeValue::Class(sc)) = type_obj.get_constructor() {
                            Some(sc.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(superclass) = superclass {
                    for prop in &superclass.declaration.properties {
                        let value = if let Some(init) = &prop.initializer {
                            Self::evaluate_expr(interpreter, init).await?
                        } else {
                            RuntimeValue::Null(NullValue::new())
                        };
                        properties.insert(prop.name.clone(), value);
                    }

                    for method in &superclass.declaration.methods {
                        if !method.is_static {
                            let fn_type =
                                Type::Function(Box::new(crate::ast::types::FunctionType {
                                    params: method
                                        .parameters
                                        .iter()
                                        .map(|p| p.param_type.clone())
                                        .collect(),
                                    return_type: method
                                        .return_type
                                        .clone()
                                        .unwrap_or_else(|| PrimitiveType::unknown()),
                                    is_variadic: method.parameters.iter().any(|p| p.is_rest),
                                }));

                            let function = FunctionValue::new(
                                method.parameters.clone(),
                                method.body.clone(),
                                false,
                                fn_type,
                            );

                            methods.insert(method.name.clone(), function);
                        }
                    }
                }
            }

            for prop in &class.declaration.properties {
                let value = if let Some(init) = &prop.initializer {
                    Self::evaluate_expr(interpreter, init).await?
                } else {
                    RuntimeValue::Null(NullValue::new())
                };
                properties.insert(prop.name.clone(), value);
            }

            for method in &class.declaration.methods {
                if !method.is_static {
                    let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
                        params: method
                            .parameters
                            .iter()
                            .map(|p| p.param_type.clone())
                            .collect(),
                        return_type: method
                            .return_type
                            .clone()
                            .unwrap_or_else(|| PrimitiveType::unknown()),
                        is_variadic: method.parameters.iter().any(|p| p.is_rest),
                    }));

                    let function = FunctionValue::new(
                        method.parameters.clone(),
                        method.body.clone(),
                        false,
                        fn_type,
                    );

                    methods.insert(method.name.clone(), function);
                }
            }

            let mut accessors = Vec::new();

            if let Some(ref superclass_name) = class.declaration.superclass {
                let superclass_value = interpreter
                    .environment
                    .get(superclass_name, new_expr.position)
                    .ok();
                let superclass = match superclass_value {
                    Some(RuntimeValue::Class(sc)) => Some(sc),
                    Some(RuntimeValue::Type(type_obj)) => {
                        if let Some(RuntimeValue::Class(sc)) = type_obj.get_constructor() {
                            Some(sc.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(superclass) = superclass {
                    accessors.extend(superclass.declaration.accessors.clone());
                }
            }

            accessors.extend(class.declaration.accessors.clone());

            let instance = crate::runtime::ClassInstance::new(
                class.class_name.clone(),
                properties,
                methods,
                accessors,
                class.class_type.clone(),
            );

            if let Some(constructor) = &class.declaration.constructor {
                let mut args = Vec::new();
                for arg in &new_expr.args {
                    args.push(Self::evaluate_expr(interpreter, arg).await?);
                }

                interpreter.environment.push_scope();

                let mut positional_index = 0;

                for (i, param) in constructor.parameters.iter().enumerate() {
                    let param_name = match &param.pattern {
                        VarPattern::Identifier(name) => name.clone(),
                        VarPattern::Destructuring(_) => {
                            format!("__param_{}", i)
                        }
                    };

                    let value = if param.is_rest {
                        let mut rest_args = Vec::new();
                        while positional_index < args.len() {
                            rest_args.push(args[positional_index].clone());
                            positional_index += 1;
                        }

                        let element_type = match &param.param_type {
                            Type::List(list_type) => list_type.element_type.clone(),
                            _ => PrimitiveType::any(),
                        };

                        RuntimeValue::List(ListValue::new(rest_args, element_type))
                    } else if positional_index < args.len() {
                        let arg = args[positional_index].clone();
                        positional_index += 1;
                        arg
                    } else if let Some(default_expr) = &param.default_value {
                        Self::evaluate_expr(interpreter, default_expr).await?
                    } else {
                        interpreter.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter '{}'", param_name),
                            (0, 0),
                            interpreter.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            interpreter.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) =
                                Helpers::destructure_pattern(interpreter, pattern, &value, (0, 0))
                                    .await
                            {
                                interpreter.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                interpreter.environment.declare(
                    "this".to_string(),
                    RuntimeValue::ClassInstance(instance.clone()),
                )?;

                for stmt in &constructor.body {
                    if let Stmt::ExprStmt(expr_stmt) = stmt {
                        if let Expr::Assignment(assign) = &expr_stmt.expression {
                            if let Expr::Member(member) = &*assign.target {
                                if let Expr::This(_) = &*member.object {
                                    let value =
                                        Self::evaluate_expr(interpreter, &assign.value).await?;
                                    instance
                                        .properties
                                        .write()
                                        .unwrap()
                                        .insert(member.property.clone(), value);
                                    continue;
                                }
                            }
                        }
                    }

                    match interpreter.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Return(_) => break,
                        _ => {}
                    }
                }

                interpreter.environment.pop_scope();
            }

            Ok(RuntimeValue::ClassInstance(instance))
        }
    }

    async fn evaluate_tagged_template(
        interpreter: &mut Interpreter,
        tagged: &TaggedTemplateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let tagged_position = tagged.position;
        let tag = Self::evaluate_expr(interpreter, &tagged.tag).await?;

        let mut strings = Vec::new();
        let mut values = Vec::new();

        for part in &tagged.template.parts {
            match part {
                TemplateStrPart::String(s) => {
                    strings.push(RuntimeValue::Str(StrValue::new(s.value.clone())));
                }
                TemplateStrPart::Expr(expr) => {
                    values.push(Self::evaluate_expr(interpreter, expr).await?);
                }
            }
        }

        if strings.len() == values.len() {
            strings.push(RuntimeValue::Str(StrValue::new(String::new())));
        }

        let strings_list = RuntimeValue::List(ListValue::new(strings, PrimitiveType::str()));

        let mut args = vec![strings_list];
        args.extend(values);

        match tag {
            RuntimeValue::Function(func) => {
                interpreter.environment.push_scope();

                let mut positional_index = 0;

                for (i, param) in func.parameters.iter().enumerate() {
                    let param_name = match &param.pattern {
                        VarPattern::Identifier(name) => name.clone(),
                        VarPattern::Destructuring(_) => {
                            format!("__param_{}", i)
                        }
                    };

                    let value = if param.is_rest {
                        let mut rest_args = Vec::new();
                        while positional_index < args.len() {
                            rest_args.push(args[positional_index].clone());
                            positional_index += 1;
                        }

                        let element_type = match &param.param_type {
                            Type::List(list_type) => list_type.element_type.clone(),
                            _ => PrimitiveType::any(),
                        };

                        RuntimeValue::List(ListValue::new(rest_args, element_type))
                    } else if positional_index < args.len() {
                        let arg = args[positional_index].clone();
                        positional_index += 1;
                        arg
                    } else if let Some(default_expr) = &param.default_value {
                        Self::evaluate_expr(interpreter, default_expr).await?
                    } else if param.is_optional {
                        RuntimeValue::Null(crate::runtime::values::NullValue)
                    } else {
                        interpreter.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter '{}'", param_name),
                            tagged_position,
                            interpreter.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            interpreter.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) =
                                Helpers::destructure_pattern(interpreter, pattern, &value, (0, 0))
                                    .await
                            {
                                interpreter.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
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
                                (0, 0),
                                interpreter.file.clone(),
                            ));
                        }
                    }
                }

                interpreter.environment.pop_scope();
                Ok(result)
            }
            RuntimeValue::NativeFunction(func) => Ok((func.implementation)(args)),
            RuntimeValue::NativeAsyncFunction(func) => {
                let result = (func.implementation)(args).await;
                let return_type = match &func.fn_type {
                    Type::Function(fn_type) => fn_type.return_type.clone(),
                    _ => PrimitiveType::any(),
                };
                Ok(RuntimeValue::Future(FutureValue::new_resolved(
                    result,
                    return_type,
                )))
            }
            _ => Err(RaccoonError::new(
                "Tagged template tag must be a function".to_string(),
                tagged.position,
                interpreter.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_method_call(
        interpreter: &mut Interpreter,
        method_call: &MethodCallExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut object = Self::evaluate_expr(interpreter, &method_call.object).await?;

        let mut args = Vec::new();
        for arg in &method_call.args {
            args.push(Self::evaluate_expr(interpreter, arg).await?);
        }

        let var_info = if let Expr::Identifier(ident) = method_call.object.as_ref() {
            Some((ident.name.clone(), ident.position))
        } else {
            None
        };

        let member_info = if let Expr::Member(member_expr) = method_call.object.as_ref() {
            if let Expr::This(_) = member_expr.object.as_ref() {
                Some(member_expr.property.clone())
            } else {
                None
            }
        } else {
            None
        };

        let result = match &mut object {
            RuntimeValue::Class(class) => {
                if let Some(static_method) = class.static_methods.get(&method_call.method) {
                    interpreter.environment.push_scope();

                    let is_async = static_method.is_async;
                    let fn_type = static_method.fn_type.clone();

                    let mut positional_index = 0;

                    for (i, param) in static_method.parameters.iter().enumerate() {
                        let param_name = match &param.pattern {
                            VarPattern::Identifier(name) => name.clone(),
                            VarPattern::Destructuring(_) => {
                                format!("__param_{}", i)
                            }
                        };

                        let value = if param.is_rest {
                            let mut rest_args = Vec::new();
                            while positional_index < args.len() {
                                rest_args.push(args[positional_index].clone());
                                positional_index += 1;
                            }

                            let element_type = match &param.param_type {
                                Type::List(list_type) => list_type.element_type.clone(),
                                _ => PrimitiveType::any(),
                            };

                            RuntimeValue::List(ListValue::new(rest_args, element_type))
                        } else if positional_index < args.len() {
                            let arg = args[positional_index].clone();
                            positional_index += 1;
                            arg
                        } else if let Some(default_expr) = &param.default_value {
                            Self::evaluate_expr(interpreter, default_expr).await?
                        } else {
                            interpreter.environment.pop_scope();
                            return Err(RaccoonError::new(
                                format!("Missing required argument for parameter '{}'", param_name),
                                (0, 0),
                                interpreter.file.clone(),
                            ));
                        };

                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                interpreter.environment.declare(name.clone(), value)?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = Helpers::destructure_pattern(
                                    interpreter,
                                    pattern,
                                    &value,
                                    (0, 0),
                                )
                                .await
                                {
                                    interpreter.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &static_method.body {
                        match interpreter.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Value(v) => result = v,
                            InterpreterResult::Return(v) => {
                                interpreter.environment.pop_scope();

                                if is_async {
                                    let return_type = match &fn_type {
                                        Type::Function(ft) => ft.return_type.clone(),
                                        _ => PrimitiveType::any(),
                                    };
                                    return Ok(RuntimeValue::Future(FutureValue::new_resolved(
                                        v,
                                        return_type,
                                    )));
                                } else {
                                    return Ok(v);
                                }
                            }
                            _ => {
                                interpreter.environment.pop_scope();
                                return Err(RaccoonError::new(
                                    "Unexpected break/continue in function".to_string(),
                                    (0, 0),
                                    interpreter.file.clone(),
                                ));
                            }
                        }
                    }

                    interpreter.environment.pop_scope();

                    if is_async {
                        let return_type = match &fn_type {
                            Type::Function(ft) => ft.return_type.clone(),
                            _ => PrimitiveType::any(),
                        };
                        Ok(RuntimeValue::Future(FutureValue::new_resolved(
                            result,
                            return_type,
                        )))
                    } else {
                        Ok(result)
                    }
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static method '{}' not found on class '{}'",
                            method_call.method, class.class_name
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    ))
                }
            }

            RuntimeValue::PrimitiveTypeObject(type_obj) => {
                if let Some(static_method) = type_obj.static_methods.get(&method_call.method) {
                    // Call native static method
                    Ok((static_method.implementation)(args))
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static method '{}' not found on type '{}'",
                            method_call.method, type_obj.type_name
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    ))
                }
            }

            RuntimeValue::Type(type_obj) => {
                if let Some(static_method) = type_obj.get_static_method(&method_call.method) {
                    // Call the static method (supports both native and user-defined functions)
                    match static_method {
                        RuntimeValue::NativeFunction(native_fn) => {
                            Ok((native_fn.implementation)(args))
                        }
                        RuntimeValue::Function(_) => {
                            // Call user-defined static method
                            Helpers::call_function(
                                interpreter,
                                static_method,
                                args,
                                method_call.position,
                            )
                            .await
                        }
                        _ => Err(RaccoonError::new(
                            format!(
                                "Static method '{}' on type '{}' is not callable",
                                method_call.method,
                                type_obj.name()
                            ),
                            method_call.position,
                            interpreter.file.clone(),
                        )),
                    }
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static method '{}' not found on type '{}'",
                            method_call.method,
                            type_obj.name()
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    ))
                }
            }

            RuntimeValue::List(_) => {
                if matches!(
                    method_call.method.as_str(),
                    "map"
                        | "filter"
                        | "reduce"
                        | "forEach"
                        | "find"
                        | "findIndex"
                        | "some"
                        | "every"
                ) {
                    Builtins::handle_list_functional_method(
                        interpreter,
                        &mut object,
                        &method_call.method,
                        args,
                        method_call.position,
                    )
                    .await
                } else {
                    interpreter.type_registry.call_instance_method(
                        &mut object,
                        &method_call.method,
                        args,
                        method_call.position,
                        interpreter.file.clone(),
                    )
                }
            }
            RuntimeValue::Str(_)
            | RuntimeValue::Map(_)
            | RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Decimal(_)
            | RuntimeValue::Bool(_) => interpreter.type_registry.call_instance_method(
                &mut object,
                &method_call.method,
                args,
                method_call.position,
                interpreter.file.clone(),
            ),
            RuntimeValue::Object(obj) => {
                if let Some(method) = obj.properties.get(&method_call.method) {
                    match method {
                        RuntimeValue::Function(func) => {
                            interpreter.environment.push_scope();

                            let is_async = func.is_async;
                            let fn_type = func.fn_type.clone();

                            let mut positional_index = 0;

                            for (i, param) in func.parameters.iter().enumerate() {
                                let param_name = match &param.pattern {
                                    VarPattern::Identifier(name) => name.clone(),
                                    VarPattern::Destructuring(_) => {
                                        format!("__param_{}", i)
                                    }
                                };

                                let value = if param.is_rest {
                                    let mut rest_args = Vec::new();
                                    while positional_index < args.len() {
                                        rest_args.push(args[positional_index].clone());
                                        positional_index += 1;
                                    }

                                    let element_type = match &param.param_type {
                                        Type::List(list_type) => list_type.element_type.clone(),
                                        _ => PrimitiveType::any(),
                                    };

                                    RuntimeValue::List(ListValue::new(rest_args, element_type))
                                } else if positional_index < args.len() {
                                    let arg = args[positional_index].clone();
                                    positional_index += 1;
                                    arg
                                } else if let Some(default_expr) = &param.default_value {
                                    Self::evaluate_expr(interpreter, default_expr).await?
                                } else {
                                    interpreter.environment.pop_scope();
                                    return Err(RaccoonError::new(
                                        format!(
                                            "Missing required argument for parameter '{}'",
                                            param_name
                                        ),
                                        (0, 0),
                                        interpreter.file.clone(),
                                    ));
                                };

                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        interpreter.environment.declare(name.clone(), value)?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        if let Err(e) = Helpers::destructure_pattern(
                                            interpreter,
                                            pattern,
                                            &value,
                                            (0, 0),
                                        )
                                        .await
                                        {
                                            interpreter.environment.pop_scope();
                                            return Err(e);
                                        }
                                    }
                                }
                            }

                            let mut result = RuntimeValue::Null(NullValue::new());
                            for stmt in &func.body {
                                match interpreter.execute_stmt_internal(stmt).await? {
                                    InterpreterResult::Value(v) => result = v,
                                    InterpreterResult::Return(v) => {
                                        interpreter.environment.pop_scope();

                                        if is_async {
                                            let return_type = match &fn_type {
                                                Type::Function(ft) => ft.return_type.clone(),
                                                _ => PrimitiveType::any(),
                                            };
                                            return Ok(RuntimeValue::Future(
                                                FutureValue::new_resolved(v, return_type),
                                            ));
                                        } else {
                                            return Ok(v);
                                        }
                                    }
                                    _ => {
                                        interpreter.environment.pop_scope();
                                        return Err(RaccoonError::new(
                                            "Unexpected break/continue in function".to_string(),
                                            (0, 0),
                                            interpreter.file.clone(),
                                        ));
                                    }
                                }
                            }

                            interpreter.environment.pop_scope();

                            if is_async {
                                let return_type = match &fn_type {
                                    Type::Function(ft) => ft.return_type.clone(),
                                    _ => PrimitiveType::any(),
                                };
                                Ok(RuntimeValue::Future(FutureValue::new_resolved(
                                    result,
                                    return_type,
                                )))
                            } else {
                                Ok(result)
                            }
                        }
                        RuntimeValue::NativeFunction(func) => Ok((func.implementation)(args)),
                        RuntimeValue::NativeAsyncFunction(func) => {
                            let result = (func.implementation)(args).await;

                            let return_type = match &func.fn_type {
                                Type::Function(ft) => ft.return_type.clone(),
                                _ => PrimitiveType::any(),
                            };
                            Ok(RuntimeValue::Future(FutureValue::new_resolved(
                                result,
                                return_type,
                            )))
                        }
                        _ => Err(RaccoonError::new(
                            format!("Property '{}' is not a function", method_call.method),
                            method_call.position,
                            interpreter.file.clone(),
                        )),
                    }
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Method '{}' not found on {}",
                            method_call.method,
                            object.get_name()
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::ClassInstance(instance) => {
                if let Some(method) = instance.methods.get(&method_call.method) {
                    interpreter.environment.push_scope();

                    interpreter.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    let is_async = method.is_async;
                    let fn_type = method.fn_type.clone();

                    let mut positional_index = 0;

                    for (i, param) in method.parameters.iter().enumerate() {
                        let param_name = match &param.pattern {
                            VarPattern::Identifier(name) => name.clone(),
                            VarPattern::Destructuring(_) => {
                                format!("__param_{}", i)
                            }
                        };

                        let value = if param.is_rest {
                            let mut rest_args = Vec::new();
                            while positional_index < args.len() {
                                rest_args.push(args[positional_index].clone());
                                positional_index += 1;
                            }

                            let element_type = match &param.param_type {
                                Type::List(list_type) => list_type.element_type.clone(),
                                _ => PrimitiveType::any(),
                            };

                            RuntimeValue::List(ListValue::new(rest_args, element_type))
                        } else if positional_index < args.len() {
                            let arg = args[positional_index].clone();
                            positional_index += 1;
                            arg
                        } else if let Some(default_expr) = &param.default_value {
                            Self::evaluate_expr(interpreter, default_expr).await?
                        } else {
                            interpreter.environment.pop_scope();
                            return Err(RaccoonError::new(
                                format!("Missing required argument for parameter '{}'", param_name),
                                (0, 0),
                                interpreter.file.clone(),
                            ));
                        };

                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                interpreter.environment.declare(name.clone(), value)?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = Helpers::destructure_pattern(
                                    interpreter,
                                    pattern,
                                    &value,
                                    (0, 0),
                                )
                                .await
                                {
                                    interpreter.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &method.body {
                        match interpreter.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Value(v) => result = v,
                            InterpreterResult::Return(v) => {
                                interpreter.environment.pop_scope();

                                if is_async {
                                    let return_type = match &fn_type {
                                        Type::Function(ft) => ft.return_type.clone(),
                                        _ => PrimitiveType::any(),
                                    };
                                    return Ok(RuntimeValue::Future(FutureValue::new_resolved(
                                        v,
                                        return_type,
                                    )));
                                } else {
                                    return Ok(v);
                                }
                            }
                            _ => {
                                interpreter.environment.pop_scope();
                                return Err(RaccoonError::new(
                                    "Unexpected break/continue in function".to_string(),
                                    (0, 0),
                                    interpreter.file.clone(),
                                ));
                            }
                        }
                    }

                    interpreter.environment.pop_scope();

                    if is_async {
                        let return_type = match &fn_type {
                            Type::Function(ft) => ft.return_type.clone(),
                            _ => PrimitiveType::any(),
                        };
                        Ok(RuntimeValue::Future(FutureValue::new_resolved(
                            result,
                            return_type,
                        )))
                    } else {
                        Ok(result)
                    }
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Method '{}' not found on class instance",
                            method_call.method
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    ))
                }
            }
            RuntimeValue::Future(future) => {
                match method_call.method.as_str() {
                    "then" => {
                        if args.is_empty() {
                            return Err(RaccoonError::new(
                                "Future.then() requires at least one callback".to_string(),
                                method_call.position,
                                interpreter.file.clone(),
                            ));
                        }

                        let on_fulfilled = args[0].clone();
                        let on_rejected = args.get(1).cloned();

                        // Read current state
                        let state = future.state.read().unwrap().clone();
                        drop(state);

                        // Create new future for the result
                        let new_future = FutureValue::new(PrimitiveType::any());
                        let new_future_clone = new_future.clone();

                        // Handle the then callback based on current state
                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                // Execute on_fulfilled callback
                                match on_fulfilled {
                                    RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                        let call_result = Helpers::call_function(
                                            interpreter,
                                            &on_fulfilled,
                                            vec![*value],
                                            method_call.position,
                                        )
                                        .await?;

                                        // If the callback returns a Future, pass it through without unwrapping
                                        // (Raccoon has explicit await semantics, not auto-unwrapping like JS Promises)
                                        new_future_clone.resolve(call_result);
                                    }
                                    _ => {
                                        return Err(RaccoonError::new(
                                            "Future.then() callback must be a function".to_string(),
                                            method_call.position,
                                            interpreter.file.clone(),
                                        ));
                                    }
                                }
                            }
                            FutureState::Rejected(error) => {
                                if let Some(on_rejected_cb) = on_rejected {
                                    match on_rejected_cb {
                                        RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                            let error_value = RuntimeValue::Str(StrValue::new(error));
                                            let call_result = Helpers::call_function(
                                                interpreter,
                                                &on_rejected_cb,
                                                vec![error_value],
                                                method_call.position,
                                            )
                                            .await?;
                                            new_future_clone.resolve(call_result);
                                        }
                                        _ => {
                                            return Err(RaccoonError::new(
                                                "Future.then() rejection callback must be a function".to_string(),
                                                method_call.position,
                                                interpreter.file.clone(),
                                            ));
                                        }
                                    }
                                } else {
                                    // No rejection handler, propagate the error
                                    new_future_clone.reject(error);
                                }
                            }
                            FutureState::Pending => {
                                return Err(RaccoonError::new(
                                    "Cannot call .then() on a pending Future. Use 'await' to wait for the Future to resolve.".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        Ok(RuntimeValue::Future(new_future))
                    }
                    "catch" => {
                        if args.is_empty() {
                            return Err(RaccoonError::new(
                                "Future.catch() requires a callback".to_string(),
                                method_call.position,
                                interpreter.file.clone(),
                            ));
                        }

                        let on_rejected = args[0].clone();

                        // Create new future for the result
                        let new_future = FutureValue::new(PrimitiveType::any());
                        let new_future_clone = new_future.clone();

                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                // Pass through the resolved value
                                new_future_clone.resolve(*value);
                            }
                            FutureState::Rejected(error) => {
                                // Execute on_rejected callback
                                match on_rejected {
                                    RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                        let error_value = RuntimeValue::Str(StrValue::new(error));
                                        let call_result = Helpers::call_function(
                                            interpreter,
                                            &on_rejected,
                                            vec![error_value],
                                            method_call.position,
                                        )
                                        .await?;
                                        new_future_clone.resolve(call_result);
                                    }
                                    _ => {
                                        return Err(RaccoonError::new(
                                            "Future.catch() callback must be a function".to_string(),
                                            method_call.position,
                                            interpreter.file.clone(),
                                        ));
                                    }
                                }
                            }
                            FutureState::Pending => {
                                return Err(RaccoonError::new(
                                    "Cannot call .catch() on a pending Future. Use 'await' to wait for the Future to resolve.".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        Ok(RuntimeValue::Future(new_future))
                    }
                    "finally" => {
                        if args.is_empty() {
                            return Err(RaccoonError::new(
                                "Future.finally() requires a callback".to_string(),
                                method_call.position,
                                interpreter.file.clone(),
                            ));
                        }

                        let on_finally = args[0].clone();

                        // Create new future for the result
                        let new_future = FutureValue::new(PrimitiveType::any());
                        let new_future_clone = new_future.clone();

                        let state = future.state.read().unwrap().clone();

                        // Execute the finally callback regardless of state
                        match on_finally {
                            RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                let _ = Helpers::call_function(
                                    interpreter,
                                    &on_finally,
                                    vec![],
                                    method_call.position,
                                )
                                .await?;
                            }
                            _ => {
                                return Err(RaccoonError::new(
                                    "Future.finally() callback must be a function".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        // Propagate the original state
                        match state {
                            FutureState::Resolved(value) => {
                                new_future_clone.resolve(*value);
                            }
                            FutureState::Rejected(error) => {
                                new_future_clone.reject(error);
                            }
                            FutureState::Pending => {
                                return Err(RaccoonError::new(
                                    "Cannot call .finally() on a pending Future. Use 'await' to wait for the Future to resolve.".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        Ok(RuntimeValue::Future(new_future))
                    }
                    "tap" => {
                        if args.is_empty() {
                            return Err(RaccoonError::new(
                                "Future.tap() requires a callback".to_string(),
                                method_call.position,
                                interpreter.file.clone(),
                            ));
                        }

                        let on_tap = args[0].clone();

                        // Create new future for the result
                        let new_future = FutureValue::new(PrimitiveType::any());
                        let new_future_clone = new_future.clone();

                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                // Execute the tap callback for side effects
                                match on_tap {
                                    RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                        let _ = Helpers::call_function(
                                            interpreter,
                                            &on_tap,
                                            vec![*value.clone()],
                                            method_call.position,
                                        )
                                        .await?;
                                    }
                                    _ => {
                                        return Err(RaccoonError::new(
                                            "Future.tap() callback must be a function".to_string(),
                                            method_call.position,
                                            interpreter.file.clone(),
                                        ));
                                    }
                                }
                                // Pass through the original value unchanged
                                new_future_clone.resolve(*value);
                            }
                            FutureState::Rejected(error) => {
                                // Pass through the rejection
                                new_future_clone.reject(error);
                            }
                            FutureState::Pending => {
                                return Err(RaccoonError::new(
                                    "Cannot call .tap() on a pending Future. Use 'await' to wait for the Future to resolve.".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        Ok(RuntimeValue::Future(new_future))
                    }
                    "map" => {
                        // Alias for .then() but only accepts one argument
                        if args.is_empty() {
                            return Err(RaccoonError::new(
                                "Future.map() requires a callback".to_string(),
                                method_call.position,
                                interpreter.file.clone(),
                            ));
                        }

                        let mapper = args[0].clone();

                        let new_future = FutureValue::new(PrimitiveType::any());
                        let new_future_clone = new_future.clone();

                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                match mapper {
                                    RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) => {
                                        let call_result = Helpers::call_function(
                                            interpreter,
                                            &mapper,
                                            vec![*value],
                                            method_call.position,
                                        )
                                        .await?;

                                        if let RuntimeValue::Future(inner_future) = call_result {
                                            let inner_state = inner_future.state.read().unwrap().clone();
                                            match inner_state {
                                                FutureState::Resolved(inner_val) => {
                                                    new_future_clone.resolve(*inner_val);
                                                }
                                                FutureState::Rejected(err) => {
                                                    new_future_clone.reject(err);
                                                }
                                                FutureState::Pending => {
                                                    new_future_clone.resolve(RuntimeValue::Future(inner_future));
                                                }
                                            }
                                        } else {
                                            new_future_clone.resolve(call_result);
                                        }
                                    }
                                    _ => {
                                        return Err(RaccoonError::new(
                                            "Future.map() callback must be a function".to_string(),
                                            method_call.position,
                                            interpreter.file.clone(),
                                        ));
                                    }
                                }
                            }
                            FutureState::Rejected(error) => {
                                new_future_clone.reject(error);
                            }
                            FutureState::Pending => {
                                return Err(RaccoonError::new(
                                    "Cannot call .map() on a pending Future. Use 'await' to wait for the Future to resolve.".to_string(),
                                    method_call.position,
                                    interpreter.file.clone(),
                                ));
                            }
                        }

                        Ok(RuntimeValue::Future(new_future))
                    }
                    _ => Err(RaccoonError::new(
                        format!(
                            "Method '{}' not found on Future. Available methods: then, catch, finally, tap, map",
                            method_call.method
                        ),
                        method_call.position,
                        interpreter.file.clone(),
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on type", method_call.method),
                method_call.position,
                interpreter.file.clone(),
            )),
        };

        let should_update = matches!(object, RuntimeValue::List(_) | RuntimeValue::Map(_));

        if should_update {
            if let Some((name, position)) = var_info {
                interpreter
                    .environment
                    .assign(&name, object.clone(), position)?;
            }

            if let Some(property_name) = member_info {
                if let Ok(RuntimeValue::ClassInstance(instance)) =
                    interpreter.environment.get("this", method_call.position)
                {
                    instance
                        .properties
                        .write()
                        .unwrap()
                        .insert(property_name, object);
                }
            }
        }

        result
    }

    async fn evaluate_await_expr(
        interpreter: &mut Interpreter,
        await_expr: &AwaitExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let future_value = Self::evaluate_expr(interpreter, &await_expr.expression).await?;

        match future_value {
            RuntimeValue::Future(future) => {
                // Wait for the future to complete
                match future.wait_for_completion().await {
                    Ok(value) => Ok(value),
                    Err(error) => Err(RaccoonError::new(
                        format!("Future rejected: {}", error),
                        await_expr.position,
                        interpreter.file.clone(),
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Cannot await non-future value: {}", future_value.get_name()),
                await_expr.position,
                interpreter.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_match_expr(
        interpreter: &mut Interpreter,
        match_expr: &MatchExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        // Evaluate the scrutinee (the value being matched)
        let scrutinee_value = Self::evaluate_expr(interpreter, &match_expr.scrutinee).await?;

        // Try each arm in order
        for arm in &match_expr.arms {
            // Try to match the pattern
            if let Some(bindings) = Self::match_pattern(&arm.pattern, &scrutinee_value)? {
                // Pattern matched - push new scope with bindings
                interpreter.environment.push_scope();
                for (name, value) in bindings {
                    let _ = interpreter.environment.declare(name, value);
                }

                // Evaluate the body expression
                let result = Self::evaluate_expr(interpreter, &arm.body).await;
                interpreter.environment.pop_scope();

                return result;
            }
        }

        // No arm matched
        Err(RaccoonError::new(
            "Non-exhaustive pattern match: no arm matched the value".to_string(),
            match_expr.position,
            interpreter.file.clone(),
        ))
    }

    /// Try to match a pattern against a value
    /// Returns Some(bindings) if the pattern matches, None if it doesn't
    fn match_pattern(
        pattern: &Pattern,
        value: &RuntimeValue,
    ) -> Result<Option<HashMap<String, RuntimeValue>>, RaccoonError> {
        match pattern {
            Pattern::Wildcard(_) => {
                // Wildcard matches anything with no bindings
                Ok(Some(HashMap::new()))
            }

            Pattern::Variable(name) => {
                // Variable pattern binds the value to the variable name
                let mut bindings = HashMap::new();
                bindings.insert(name.clone(), value.clone());
                Ok(Some(bindings))
            }

            Pattern::Literal(expr) => {
                // Compare literal pattern with value using == operator
                // For literals, we match if values are equal
                match (expr.as_ref(), value) {
                    // Integer literals
                    (Expr::IntLiteral(lit), RuntimeValue::Int(val)) => {
                        if lit.value as i64 == val.value {
                            Ok(Some(HashMap::new()))
                        } else {
                            Ok(None)
                        }
                    }
                    // String literals
                    (Expr::StrLiteral(lit), RuntimeValue::Str(val)) => {
                        if lit.value == val.value {
                            Ok(Some(HashMap::new()))
                        } else {
                            Ok(None)
                        }
                    }
                    // Float literals
                    (Expr::FloatLiteral(lit), RuntimeValue::Float(val)) => {
                        if (lit.value - val.value).abs() < f64::EPSILON {
                            Ok(Some(HashMap::new()))
                        } else {
                            Ok(None)
                        }
                    }
                    // Boolean literals
                    (Expr::BoolLiteral(lit), RuntimeValue::Bool(val)) => {
                        if lit.value == val.value {
                            Ok(Some(HashMap::new()))
                        } else {
                            Ok(None)
                        }
                    }
                    // Null literal
                    (Expr::NullLiteral(_), RuntimeValue::Null(_)) => Ok(Some(HashMap::new())),
                    // Type mismatch or unsupported literal pattern
                    _ => Ok(None),
                }
            }

            Pattern::List(patterns) => {
                // List pattern: [p1, p2, ...]
                if let RuntimeValue::List(list_val) = value {
                    let elements = &list_val.elements;
                    if elements.len() != patterns.len() {
                        return Ok(None); // Length mismatch
                    }

                    let mut all_bindings = HashMap::new();

                    // Try to match each element
                    for (element, pattern) in elements.iter().zip(patterns.iter()) {
                        if let Some(bindings) = Self::match_pattern(pattern, element)? {
                            // Merge bindings
                            for (name, val) in bindings {
                                all_bindings.insert(name, val);
                            }
                        } else {
                            return Ok(None); // One of the patterns didn't match
                        }
                    }

                    Ok(Some(all_bindings))
                } else {
                    Ok(None) // Value is not a list
                }
            }

            Pattern::Object(properties) => {
                // Object pattern: { x, y: pat }
                if let RuntimeValue::Object(obj_val) = value {
                    let mut all_bindings = HashMap::new();

                    // Try to match each property
                    for (key, pattern) in properties {
                        if let Some(property_value) = obj_val.properties.get(key) {
                            if let Some(bindings) = Self::match_pattern(pattern, property_value)? {
                                // Merge bindings
                                for (name, val) in bindings {
                                    all_bindings.insert(name, val);
                                }
                            } else {
                                return Ok(None); // Property pattern didn't match
                            }
                        } else {
                            return Ok(None); // Property not found in object
                        }
                    }

                    Ok(Some(all_bindings))
                } else {
                    Ok(None) // Value is not an object
                }
            }

            Pattern::Range(_, _) => {
                // Range patterns not implemented yet
                // TODO: Implement range patterns
                Ok(Some(HashMap::new()))
            }

            Pattern::Type(_) => {
                // Type patterns not implemented yet
                // TODO: Implement type patterns
                Ok(Some(HashMap::new()))
            }

            Pattern::Or(patterns) => {
                // Or pattern: try each alternative
                for alt_pattern in patterns {
                    if let Some(bindings) = Self::match_pattern(alt_pattern, value)? {
                        return Ok(Some(bindings));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Evaluate anonymous class expression
    /// Returns a ClassValue that can be instantiated with 'new'
    #[async_recursion(?Send)]
    async fn evaluate_class_expr(
        interpreter: &mut Interpreter,
        class_expr: &ClassExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        // Generate a synthetic name for the anonymous class
        let synthetic_name = format!("__AnonymousClass_{:?}", class_expr.position);

        // Extract static methods into a HashMap
        let mut static_methods = HashMap::new();
        for method in &class_expr.methods {
            if method.is_static {
                let return_type = method.return_type.clone().unwrap_or(Type::Primitive(
                    crate::ast::types::PrimitiveType::new(
                        crate::ast::types::TypeKind::Void,
                        "void",
                    ),
                ));

                let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
                    params: method
                        .parameters
                        .iter()
                        .map(|p| p.param_type.clone())
                        .collect(),
                    return_type,
                    is_variadic: method.parameters.iter().any(|p| p.is_rest),
                }));

                let function = FunctionValue::new(
                    method.parameters.clone(),
                    method.body.clone(),
                    method.is_async,
                    fn_type,
                );
                static_methods.insert(method.name.clone(), Box::new(function));
            }
        }

        // Extract static properties into a HashMap
        let mut static_properties = HashMap::new();
        for prop in &class_expr.properties {
            if let Some(init) = &prop.initializer {
                let value = Self::evaluate_expr(interpreter, init).await?;
                static_properties.insert(prop.name.clone(), value);
            } else {
                static_properties.insert(prop.name.clone(), RuntimeValue::Null(NullValue::new()));
            }
        }

        // Create a ClassDecl from the ClassExpr for storage
        let class_decl = ClassDecl {
            name: synthetic_name.clone(),
            type_parameters: class_expr.type_parameters.clone(),
            superclass: class_expr.superclass.clone(),
            properties: class_expr.properties.clone(),
            constructor: class_expr.constructor.clone(),
            methods: class_expr.methods.clone(),
            accessors: class_expr.accessors.clone(),
            decorators: Vec::new(),
            position: class_expr.position,
        };

        let class_type = Type::Primitive(crate::ast::types::PrimitiveType::new(
            crate::ast::types::TypeKind::Class,
            &synthetic_name,
        ));

        // Create and return the ClassValue
        Ok(RuntimeValue::Class(ClassValue::with_properties(
            synthetic_name,
            static_methods,
            static_properties,
            class_type,
            class_decl,
        )))
    }
}
