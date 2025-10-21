use crate::ast::nodes::*;
use crate::ast::types::{PrimitiveType, Type};
use crate::error::RaccoonError;
use crate::runtime::{
    BoolValue, Environment, FloatValue, FunctionValue, FutureState, FutureValue, IntValue,
    ListValue, NullValue, ObjectValue, RuntimeValue, StrValue, TypeRegistry, setup_builtins,
};
use crate::tokens::{BinaryOperator, Position};
use async_recursion::async_recursion;
use std::collections::HashMap;

pub enum InterpreterResult {
    Value(RuntimeValue),
    Return(RuntimeValue),
    Break,
    Continue,
}

impl InterpreterResult {}

pub struct Interpreter {
    pub file: Option<String>,
    environment: Environment,
    type_registry: TypeRegistry,
    stdlib_loader: std::sync::Arc<crate::runtime::StdLibLoader>,
}

impl Interpreter {
    pub fn new(file: Option<String>) -> Self {
        let mut env = Environment::new(file.clone());
        let type_registry = TypeRegistry::new();
        setup_builtins(&mut env);
        let native_bridge = std::sync::Arc::new(crate::runtime::NativeBridge::new());

        for name in ["native_print", "native_eprint"] {
            if let Some(f) = native_bridge.get(name) {
                let _ = env.declare(name.to_string(), f);
            }
        }

        for name in ["native_http_request"] {
            if let Some(f) = native_bridge.get_async(name) {
                let _ = env.declare(name.to_string(), f);
            }
        }
        let stdlib_loader = std::sync::Arc::new(crate::runtime::StdLibLoader::with_default_path());
        Self {
            environment: env,
            file,
            type_registry,
            stdlib_loader,
        }
    }

    #[async_recursion(?Send)]
    pub async fn interpret(&mut self, program: &Program) -> Result<RuntimeValue, RaccoonError> {
        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &program.stmts {
            match self.execute_stmt_internal(stmt).await {
                Ok(InterpreterResult::Value(v)) => last_value = v,
                Ok(_) => {
                    return Err(RaccoonError::new(
                        "Unexpected control flow statement".to_string(),
                        (0, 0),
                        self.file.clone(),
                    ));
                }
                Err(e) => return Err(e),
            }
        }

        Ok(last_value)
    }

    #[async_recursion(?Send)]
    async fn execute_stmt_internal(
        &mut self,
        stmt: &Stmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        match stmt {
            Stmt::Program(program) => self.interpret(program).await.map(InterpreterResult::Value),
            Stmt::VarDecl(decl) => self
                .execute_var_decl(decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::FnDecl(decl) => self
                .execute_fn_decl(decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::Block(block) => self.execute_block_internal(block).await,
            Stmt::IfStmt(if_stmt) => self.execute_if_stmt_internal(if_stmt).await,
            Stmt::WhileStmt(while_stmt) => self.execute_while_stmt(while_stmt).await,
            Stmt::ForStmt(for_stmt) => self.execute_for_stmt(for_stmt).await,
            Stmt::ForInStmt(for_in) => self.execute_for_in_stmt(for_in).await,
            Stmt::ReturnStmt(ret) => self.execute_return_stmt(ret).await,
            Stmt::BreakStmt(_) => Ok(InterpreterResult::Break),
            Stmt::ContinueStmt(_) => Ok(InterpreterResult::Continue),
            Stmt::ExprStmt(expr_stmt) => self
                .evaluate_expr(&expr_stmt.expression)
                .await
                .map(InterpreterResult::Value),
            Stmt::ClassDecl(decl) => self
                .execute_class_decl(decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::InterfaceDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::EnumDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TypeAliasDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::ImportDecl(import_decl) => self.execute_import_decl(import_decl).await,
            Stmt::ExportDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TryStmt(try_stmt) => self.execute_try_stmt(try_stmt).await,
            Stmt::ThrowStmt(throw) => self.execute_throw_stmt(throw).await,
        }
    }

    async fn execute_var_decl(&mut self, decl: &VarDecl) -> Result<RuntimeValue, RaccoonError> {
        let value = if let Some(init) = &decl.initializer {
            self.evaluate_expr(init).await?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        match &decl.pattern {
            VarPattern::Identifier(name) => {
                self.environment.declare(name.clone(), value.clone())?;
            }
            VarPattern::Destructuring(pattern) => {
                self.destructure_pattern(pattern, &value, decl.position)
                    .await?;
            }
        }

        Ok(value)
    }

    async fn execute_fn_decl(&mut self, decl: &FnDecl) -> Result<RuntimeValue, RaccoonError> {
        let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
            params: decl
                .parameters
                .iter()
                .map(|p| p.param_type.clone())
                .collect(),
            return_type: decl.return_type.clone(),
            is_variadic: false,
        }));

        let function = RuntimeValue::Function(FunctionValue::new(
            decl.parameters.clone(),
            decl.body.clone(),
            decl.is_async,
            fn_type,
        ));

        self.environment.declare(decl.name.clone(), function)?;

        Ok(RuntimeValue::Null(NullValue::new()))
    }

    async fn execute_class_decl(&mut self, decl: &ClassDecl) -> Result<RuntimeValue, RaccoonError> {
        let class_type = PrimitiveType::any();

        let mut static_methods = HashMap::new();
        for method in &decl.methods {
            if method.is_static {
                let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
                    params: method
                        .parameters
                        .iter()
                        .map(|p| p.param_type.clone())
                        .collect(),
                    return_type: method.return_type.clone(),
                    is_variadic: false,
                }));

                let function = Box::new(FunctionValue::new(
                    method.parameters.clone(),
                    method.body.clone(),
                    method.is_async,
                    fn_type,
                ));

                static_methods.insert(method.name.clone(), function);
            }
        }

        let class_value = RuntimeValue::Class(crate::runtime::ClassValue::new(
            decl.name.clone(),
            static_methods,
            class_type,
            decl.clone(),
        ));

        self.environment.declare(decl.name.clone(), class_value)?;

        Ok(RuntimeValue::Null(NullValue::new()))
    }

    #[async_recursion(?Send)]
    async fn execute_block_internal(
        &mut self,
        block: &Block,
    ) -> Result<InterpreterResult, RaccoonError> {
        self.environment.push_scope();

        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &block.statements {
            match self.execute_stmt_internal(stmt).await? {
                InterpreterResult::Value(v) => last_value = v,
                other => {
                    self.environment.pop_scope();
                    return Ok(other);
                }
            }
        }

        self.environment.pop_scope();

        Ok(InterpreterResult::Value(last_value))
    }

    #[async_recursion(?Send)]
    async fn execute_if_stmt_internal(
        &mut self,
        if_stmt: &IfStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let condition = self.evaluate_expr(&if_stmt.condition).await?;

        if self.is_truthy(&condition) {
            self.execute_stmt_internal(&if_stmt.then_branch).await
        } else if let Some(else_branch) = &if_stmt.else_branch {
            self.execute_stmt_internal(else_branch).await
        } else {
            Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )))
        }
    }

    #[async_recursion(?Send)]
    async fn execute_while_stmt(
        &mut self,
        while_stmt: &WhileStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        loop {
            let condition = self.evaluate_expr(&while_stmt.condition).await?;
            if !self.is_truthy(&condition) {
                break;
            }

            match self.execute_stmt_internal(&while_stmt.body).await? {
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
    async fn execute_for_stmt(
        &mut self,
        for_stmt: &ForStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        self.environment.push_scope();

        if let Some(init) = &for_stmt.initializer {
            self.execute_stmt_internal(init).await?;
        }

        loop {
            if let Some(condition) = &for_stmt.condition {
                let cond_value = self.evaluate_expr(condition).await?;
                if !self.is_truthy(&cond_value) {
                    break;
                }
            }

            match self.execute_stmt_internal(&for_stmt.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => {}
                InterpreterResult::Return(v) => {
                    self.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }

            if let Some(increment) = &for_stmt.increment {
                self.evaluate_expr(increment).await?;
            }
        }

        self.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    #[async_recursion(?Send)]
    async fn execute_for_in_stmt(
        &mut self,
        for_in: &ForInStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let iterable = self.evaluate_expr(&for_in.iterable).await?;

        let elements = match iterable {
            RuntimeValue::List(list) => list.elements,
            _ => {
                return Err(RaccoonError::new(
                    "For-in requires an iterable value".to_string(),
                    for_in.position,
                    self.file.clone(),
                ));
            }
        };

        self.environment.push_scope();

        if !elements.is_empty() {
            self.environment
                .declare(for_in.variable.clone(), elements[0].clone())?;
        }

        for element in elements {
            self.environment
                .assign(&for_in.variable, element, for_in.position)?;

            match self.execute_stmt_internal(&for_in.body).await? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => continue,
                InterpreterResult::Return(v) => {
                    self.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }
        }

        self.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    async fn execute_return_stmt(
        &mut self,
        ret: &ReturnStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let value = if let Some(expr) = &ret.value {
            self.evaluate_expr(expr).await?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        Ok(InterpreterResult::Return(value))
    }

    #[async_recursion(?Send)]
    async fn execute_try_stmt(
        &mut self,
        try_stmt: &TryStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let result = self.execute_block_internal(&try_stmt.try_block).await;

        match result {
            Ok(value) => {
                if let Some(finally_block) = &try_stmt.finally_block {
                    self.execute_block_internal(finally_block).await?;
                }
                Ok(value)
            }
            Err(error) => {
                for catch_clause in &try_stmt.catch_clauses {
                    self.environment.push_scope();
                    let error_value = RuntimeValue::Str(StrValue::new(error.message.clone()));
                    self.environment
                        .declare(catch_clause.error_var.clone(), error_value)?;

                    let result = self.execute_block_internal(&catch_clause.body).await;
                    self.environment.pop_scope();

                    if let Some(finally_block) = &try_stmt.finally_block {
                        self.execute_block_internal(finally_block).await?;
                    }

                    return result;
                }

                if let Some(finally_block) = &try_stmt.finally_block {
                    self.execute_block_internal(finally_block).await?;
                }

                Err(error)
            }
        }
    }

    async fn execute_throw_stmt(
        &mut self,
        throw: &ThrowStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let value = self.evaluate_expr(&throw.value).await?;
        Err(RaccoonError::new(
            value.to_string(),
            throw.position,
            self.file.clone(),
        ))
    }

    #[async_recursion(?Send)]
    async fn evaluate_expr(&mut self, expr: &Expr) -> Result<RuntimeValue, RaccoonError> {
        match expr {
            Expr::IntLiteral(lit) => Ok(RuntimeValue::Int(IntValue::new(lit.value))),
            Expr::FloatLiteral(lit) => Ok(RuntimeValue::Float(FloatValue::new(lit.value))),
            Expr::StrLiteral(lit) => Ok(RuntimeValue::Str(StrValue::new(lit.value.clone()))),
            Expr::BoolLiteral(lit) => Ok(RuntimeValue::Bool(BoolValue::new(lit.value))),
            Expr::NullLiteral(_) => Ok(RuntimeValue::Null(NullValue::new())),
            Expr::Identifier(ident) => self.environment.get(&ident.name, ident.position),
            Expr::Binary(binary) => self.evaluate_binary_expr(binary).await,
            Expr::Unary(unary) => self.evaluate_unary_expr(unary).await,
            Expr::Assignment(assign) => self.evaluate_assignment(assign).await,
            Expr::Call(call) => self.evaluate_call_expr(call).await,
            Expr::ListLiteral(list) => self.evaluate_list_literal(list).await,
            Expr::ObjectLiteral(obj) => self.evaluate_object_literal(obj).await,
            Expr::Member(member) => self.evaluate_member_expr(member).await,
            Expr::Index(index) => self.evaluate_index_expr(index).await,
            Expr::Conditional(cond) => self.evaluate_conditional_expr(cond).await,
            Expr::UnaryUpdate(update) => self.evaluate_unary_update(update).await,
            Expr::TemplateStr(template) => self.evaluate_template_str(template).await,
            Expr::ArrowFn(arrow) => self.evaluate_arrow_fn(arrow).await,
            Expr::TypeOf(typeof_expr) => self.evaluate_typeof_expr(typeof_expr).await,
            Expr::InstanceOf(instanceof) => self.evaluate_instanceof_expr(instanceof).await,
            Expr::OptionalChaining(opt_chain) => self.evaluate_optional_chaining(opt_chain).await,
            Expr::NullAssertion(null_assert) => self.evaluate_null_assertion(null_assert).await,
            Expr::MethodCall(method_call) => self.evaluate_method_call(method_call).await,
            Expr::This(_) => self.evaluate_this_expr().await,
            Expr::Super(_) => self.evaluate_super_expr().await,
            Expr::New(new_expr) => self.evaluate_new_expr(new_expr).await,
            Expr::TaggedTemplate(tagged) => self.evaluate_tagged_template(tagged).await,
            Expr::Range(range) => self.evaluate_range_expr(range).await,
            Expr::NullCoalescing(null_coal) => self.evaluate_null_coalescing(null_coal).await,
            Expr::Await(await_expr) => self.evaluate_await_expr(await_expr).await,
        }
    }

    async fn apply_binary_op(
        &self,
        left: RuntimeValue,
        right: RuntimeValue,
        operator: BinaryOperator,
        position: Position,
    ) -> Result<RuntimeValue, RaccoonError> {
        match operator {
            BinaryOperator::Add => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value + r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 + r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value + r.value as f64),
                )),
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => Ok(RuntimeValue::Str(
                    StrValue::new(format!("{}{}", l.value, r.value)),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for addition".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Subtract => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value - r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 - r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value - r.value as f64),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for subtraction".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Multiply => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value * r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 * r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value * r.value as f64),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for multiplication".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Divide => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value as f64 / r.value as f64,
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    if r.value == 0.0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    if r.value == 0.0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value as f64 / r.value,
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value / r.value as f64,
                    )))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for division".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Modulo => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Modulo by zero".to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Int(IntValue::new(l.value % r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for modulo".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::BitwiseAnd => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value & r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise AND requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::BitwiseOr => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value | r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise OR requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::BitwiseXor => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value ^ r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise XOR requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::LeftShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value << r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Left shift requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::RightShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value >> r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Right shift requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::UnsignedRightShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(
                    IntValue::new((l.value as u64 >> r.value) as i64),
                )),
                _ => Err(RaccoonError::new(
                    "Unsigned right shift requires integer operands".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Exponent => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value < 0 {
                        return Err(RaccoonError::new(
                            "Integer exponentiation with negative exponent not supported"
                                .to_string(),
                            position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Int(IntValue::new(
                        l.value.pow(r.value as u32),
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value.powf(r.value))))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new((l.value as f64).powf(r.value)),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value.powf(r.value as f64)),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for exponentiation".to_string(),
                    position,
                    self.file.clone(),
                )),
            },
            _ => Err(RaccoonError::new(
                format!("Operator {:?} not supported in apply_binary_op", operator),
                position,
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_binary_expr(
        &mut self,
        binary: &BinaryExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let left = self.evaluate_expr(&binary.left).await?;
        let right = self.evaluate_expr(&binary.right).await?;

        match binary.operator {
            BinaryOperator::Add => match (&left, &right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value + r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 + r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value + r.value as f64),
                )),
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => Ok(RuntimeValue::Str(
                    StrValue::new(format!("{}{}", l.value, r.value)),
                )),

                (RuntimeValue::Str(l), r) => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "{}{}",
                    l.value,
                    r.to_string()
                )))),
                (l, RuntimeValue::Str(r)) => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "{}{}",
                    l.to_string(),
                    r.value
                )))),
                _ => Err(RaccoonError::new(
                    "Invalid operands for addition".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Subtract => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value - r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 - r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value - r.value as f64),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for subtraction".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Multiply => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value * r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value as f64 * r.value),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value * r.value as f64),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for multiplication".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Divide => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value as f64 / r.value as f64,
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    if r.value == 0.0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    if r.value == 0.0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value as f64 / r.value,
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Division by zero".to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Float(FloatValue::new(
                        l.value / r.value as f64,
                    )))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for division".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Modulo => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value == 0 {
                        return Err(RaccoonError::new(
                            "Modulo by zero".to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Int(IntValue::new(l.value % r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for modulo".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Equal => Ok(RuntimeValue::Bool(BoolValue::new(left.equals(&right)))),
            BinaryOperator::NotEqual => {
                Ok(RuntimeValue::Bool(BoolValue::new(!left.equals(&right))))
            }
            BinaryOperator::LessThan => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for less than comparison".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::LessEqual => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for less than or equal comparison".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::GreaterThan => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for greater than comparison".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::GreaterEqual => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for greater than or equal comparison".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::And => {
                if !self.is_truthy(&left) {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            BinaryOperator::Or => {
                if self.is_truthy(&left) {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            BinaryOperator::BitwiseAnd => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value & r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise AND requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::BitwiseOr => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value | r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise OR requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::BitwiseXor => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value ^ r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Bitwise XOR requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::LeftShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value << r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Left shift requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::RightShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Int(IntValue::new(l.value >> r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Right shift requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::UnsignedRightShift => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(
                    IntValue::new((l.value as u64 >> r.value) as i64),
                )),
                _ => Err(RaccoonError::new(
                    "Unsigned right shift requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Exponent => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    if r.value < 0 {
                        return Err(RaccoonError::new(
                            "Integer exponentiation with negative exponent not supported"
                                .to_string(),
                            binary.position,
                            self.file.clone(),
                        ));
                    }
                    Ok(RuntimeValue::Int(IntValue::new(
                        l.value.pow(r.value as u32),
                    )))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Float(FloatValue::new(l.value.powf(r.value))))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new((l.value as f64).powf(r.value)),
                )),
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Float(
                    FloatValue::new(l.value.powf(r.value as f64)),
                )),
                _ => Err(RaccoonError::new(
                    "Invalid operands for exponentiation".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::Range => match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    let mut elements = Vec::new();
                    for i in l.value..=r.value {
                        elements.push(RuntimeValue::Int(IntValue::new(i)));
                    }
                    Ok(RuntimeValue::List(ListValue::new(
                        elements,
                        PrimitiveType::int(),
                    )))
                }
                _ => Err(RaccoonError::new(
                    "Range operator requires integer operands".to_string(),
                    binary.position,
                    self.file.clone(),
                )),
            },
            BinaryOperator::NullCoalesce => {
                if matches!(left, RuntimeValue::Null(_)) {
                    Ok(right)
                } else {
                    Ok(left)
                }
            }
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_unary_expr(
        &mut self,
        unary: &UnaryExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let operand = self.evaluate_expr(&unary.operand).await?;

        match unary.operator {
            crate::tokens::UnaryOperator::Negate => match operand {
                RuntimeValue::Int(v) => Ok(RuntimeValue::Int(IntValue::new(-v.value))),
                RuntimeValue::Float(v) => Ok(RuntimeValue::Float(FloatValue::new(-v.value))),
                _ => Err(RaccoonError::new(
                    "Invalid operand for unary minus".to_string(),
                    unary.position,
                    self.file.clone(),
                )),
            },
            crate::tokens::UnaryOperator::Not => Ok(RuntimeValue::Bool(BoolValue::new(
                !self.is_truthy(&operand),
            ))),
            crate::tokens::UnaryOperator::BitwiseNot => match operand {
                RuntimeValue::Int(v) => Ok(RuntimeValue::Int(IntValue::new(!v.value))),
                _ => Err(RaccoonError::new(
                    "Invalid operand for bitwise not".to_string(),
                    unary.position,
                    self.file.clone(),
                )),
            },
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_assignment(
        &mut self,
        assign: &Assignment,
    ) -> Result<RuntimeValue, RaccoonError> {
        use crate::tokens::TokenType;

        let final_value = if assign.operator != TokenType::Assign {
            let current_value = self.evaluate_expr(&assign.target).await?;
            let right_value = self.evaluate_expr(&assign.value).await?;

            match assign.operator {
                TokenType::PlusAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Add,
                        assign.position,
                    )
                    .await?
                }
                TokenType::MinusAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Subtract,
                        assign.position,
                    )
                    .await?
                }
                TokenType::MultiplyAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Multiply,
                        assign.position,
                    )
                    .await?
                }
                TokenType::DivideAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Divide,
                        assign.position,
                    )
                    .await?
                }
                TokenType::ModuloAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Modulo,
                        assign.position,
                    )
                    .await?
                }
                TokenType::AmpersandAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseAnd,
                        assign.position,
                    )
                    .await?
                }
                TokenType::BitwiseOrAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseOr,
                        assign.position,
                    )
                    .await?
                }
                TokenType::BitwiseXorAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::BitwiseXor,
                        assign.position,
                    )
                    .await?
                }
                TokenType::LeftShiftAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::LeftShift,
                        assign.position,
                    )
                    .await?
                }
                TokenType::RightShiftAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::RightShift,
                        assign.position,
                    )
                    .await?
                }
                TokenType::UnsignedRightShiftAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::UnsignedRightShift,
                        assign.position,
                    )
                    .await?
                }
                TokenType::ExponentAssign => {
                    self.apply_binary_op(
                        current_value,
                        right_value,
                        BinaryOperator::Exponent,
                        assign.position,
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
                        self.file.clone(),
                    ));
                }
            }
        } else {
            self.evaluate_expr(&assign.value).await?
        };

        match &*assign.target {
            Expr::Identifier(ident) => {
                self.environment
                    .assign(&ident.name, final_value.clone(), ident.position)?;
                Ok(final_value)
            }
            Expr::Member(member) => {
                let mut object = self.evaluate_expr(&member.object).await?;

                match &mut object {
                    RuntimeValue::Object(obj) => {
                        obj.properties
                            .insert(member.property.clone(), final_value.clone());

                        if let Expr::Identifier(ident) = &*member.object {
                            self.environment
                                .assign(&ident.name, object.clone(), ident.position)?;
                        }

                        Ok(final_value)
                    }
                    RuntimeValue::ClassInstance(instance) => {
                        if let Some(accessor) = instance.accessors.iter().find(|a| {
                            a.name == member.property && matches!(a.kind, AccessorKind::Set)
                        }) {
                            self.environment.push_scope();

                            self.environment.declare(
                                "this".to_string(),
                                RuntimeValue::ClassInstance(instance.clone()),
                            )?;

                            if let Some(param) = accessor.parameters.first() {
                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        self.environment
                                            .declare(name.clone(), final_value.clone())?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        self.destructure_pattern(
                                            pattern,
                                            &final_value,
                                            assign.position,
                                        )
                                        .await?;
                                    }
                                }
                            }

                            for stmt in &accessor.body {
                                match self.execute_stmt_internal(stmt).await? {
                                    InterpreterResult::Return(_) => break,
                                    _ => {}
                                }
                            }

                            self.environment.pop_scope();
                            return Ok(final_value);
                        }

                        instance
                            .properties
                            .write()
                            .unwrap()
                            .insert(member.property.clone(), final_value.clone());

                        if let Expr::Identifier(ident) = &*member.object {
                            self.environment
                                .assign(&ident.name, object.clone(), ident.position)?;
                        }

                        Ok(final_value)
                    }
                    _ => Err(RaccoonError::new(
                        "Cannot assign to property of non-object".to_string(),
                        assign.position,
                        self.file.clone(),
                    )),
                }
            }
            Expr::Index(index_expr) => {
                let mut object = self.evaluate_expr(&index_expr.object).await?;
                let idx = self.evaluate_expr(&index_expr.index).await?;

                match (&mut object, &idx) {
                    (RuntimeValue::List(list), RuntimeValue::Int(i)) => {
                        if i.value < 0 || i.value >= list.elements.len() as i64 {
                            return Err(RaccoonError::new(
                                format!("[1409] Index out of bounds: {}", i.value),
                                assign.position,
                                self.file.clone(),
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
                            self.file.clone(),
                        ));
                    }
                }

                if let Expr::Identifier(ident) = &*index_expr.object {
                    self.environment
                        .assign(&ident.name, object.clone(), ident.position)?;
                }

                Ok(final_value)
            }
            _ => Err(RaccoonError::new(
                "Invalid assignment target".to_string(),
                assign.position,
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_call_expr(&mut self, call: &CallExpr) -> Result<RuntimeValue, RaccoonError> {
        if matches!(call.callee.as_ref(), Expr::Super(_)) {
            return self.evaluate_super_call(&call.args).await;
        }

        let callee = self.evaluate_expr(&call.callee).await?;

        let mut args = Vec::new();
        for arg in &call.args {
            args.push(self.evaluate_expr(arg).await?);
        }

        let mut named_args = HashMap::new();
        for (name, expr) in &call.named_args {
            named_args.insert(name.clone(), self.evaluate_expr(expr).await?);
        }

        match callee {
            RuntimeValue::Function(func) => {
                self.environment.push_scope();

                for (i, param) in func.parameters.iter().enumerate() {
                    let param_name = match &param.pattern {
                        VarPattern::Identifier(name) => name.clone(),
                        VarPattern::Destructuring(_) => {
                            format!("__param_{}", i)
                        }
                    };

                    let value = if let Some(named_value) = named_args.get(&param_name) {
                        named_value.clone()
                    } else if i < args.len() {
                        args[i].clone()
                    } else if let Some(default_expr) = &param.default_value {
                        self.evaluate_expr(default_expr).await?
                    } else {
                        self.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter '{}'", param_name),
                            (0, 0),
                            self.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            self.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) = self.destructure_pattern(pattern, &value, (0, 0)).await
                            {
                                self.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let is_async = func.is_async;
                let fn_type = func.fn_type.clone();

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
                    match self.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(v) => result = v,
                        InterpreterResult::Return(v) => {
                            self.environment.pop_scope();

                            if is_async {
                                let return_type = match &fn_type {
                                    Type::Function(fn_type) => fn_type.return_type.clone(),
                                    _ => PrimitiveType::any(),
                                };
                                return Ok(RuntimeValue::Future(FutureValue::new_resolved(
                                    v,
                                    return_type,
                                )));
                            }
                            return Ok(v);
                        }
                        _ => {
                            self.environment.pop_scope();
                            return Err(RaccoonError::new(
                                "Unexpected break/continue in function".to_string(),
                                (0, 0),
                                self.file.clone(),
                            ));
                        }
                    }
                }

                self.environment.pop_scope();

                if is_async {
                    let return_type = match &fn_type {
                        Type::Function(fn_type) => fn_type.return_type.clone(),
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
                self.file.clone(),
            )),
        }
    }

    async fn evaluate_list_literal(
        &mut self,
        list: &ListLiteral,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut elements = Vec::new();
        for elem in &list.elements {
            elements.push(self.evaluate_expr(elem).await?);
        }

        let element_type = if elements.is_empty() {
            PrimitiveType::any()
        } else {
            elements[0].get_type()
        };

        Ok(RuntimeValue::List(ListValue::new(elements, element_type)))
    }

    async fn evaluate_object_literal(
        &mut self,
        obj: &ObjectLiteral,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut properties = HashMap::new();
        for (key, value) in &obj.properties {
            properties.insert(key.clone(), self.evaluate_expr(value).await?);
        }

        Ok(RuntimeValue::Object(ObjectValue::new(
            properties,
            PrimitiveType::any(),
        )))
    }

    #[async_recursion(?Send)]
    async fn evaluate_member_expr(
        &mut self,
        member: &MemberExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&member.object).await?;

        match object {
            RuntimeValue::Class(class) => {
                if let Some(static_method) = class.static_methods.get(&member.property) {
                    Ok(RuntimeValue::Function((**static_method).clone()))
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Static member '{}' not found on class '{}'",
                            member.property, class.class_name
                        ),
                        member.position,
                        self.file.clone(),
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
                        self.file.clone(),
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
                    self.environment.push_scope();

                    self.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &accessor.body {
                        match self.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Return(value) => {
                                result = value;
                                break;
                            }
                            _ => {}
                        }
                    }

                    self.environment.pop_scope();
                    return Ok(result);
                }

                Err(RaccoonError::new(
                    format!("Property '{}' not found on class instance", member.property),
                    member.position,
                    self.file.clone(),
                ))
            }
            RuntimeValue::Str(s) => match member.property.as_str() {
                "length" => Ok(RuntimeValue::Int(IntValue::new(s.value.len() as i64))),
                "isEmpty" => Ok(RuntimeValue::Bool(BoolValue::new(s.value.is_empty()))),
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on string", member.property),
                    member.position,
                    self.file.clone(),
                )),
            },
            RuntimeValue::List(list) => match member.property.as_str() {
                "length" => Ok(RuntimeValue::Int(IntValue::new(list.elements.len() as i64))),
                "first" => {
                    if list.elements.is_empty() {
                        Err(RaccoonError::new(
                            "List is empty".to_string(),
                            member.position,
                            self.file.clone(),
                        ))
                    } else {
                        Ok(list.elements[0].clone())
                    }
                }
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on list", member.property),
                    member.position,
                    self.file.clone(),
                )),
            },
            RuntimeValue::Map(map) => match member.property.as_str() {
                "size" => Ok(RuntimeValue::Int(IntValue::new(map.entries.len() as i64))),
                _ => Err(RaccoonError::new(
                    format!("Property '{}' not found on map", member.property),
                    member.position,
                    self.file.clone(),
                )),
            },
            _ => Err(RaccoonError::new(
                format!("Cannot access property '{}' on type", member.property),
                member.position,
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_index_expr(
        &mut self,
        index: &IndexExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&index.object).await?;
        let idx = self.evaluate_expr(&index.index).await?;

        match (object, idx) {
            (RuntimeValue::List(list), RuntimeValue::Int(i)) => {
                if i.value < 0 || i.value >= list.elements.len() as i64 {
                    println!("Index value: {}", i.value);
                    println!("List length: {}", list.elements.len());
                    println!("List: {:?}", list.elements);

                    return Err(RaccoonError::new(
                        "[1614] Index out of bounds".to_string(),
                        index.position,
                        self.file.clone(),
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
                        self.file.clone(),
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
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_conditional_expr(
        &mut self,
        cond: &ConditionalExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let condition = self.evaluate_expr(&cond.condition).await?;

        if self.is_truthy(&condition) {
            self.evaluate_expr(&cond.then_expr).await
        } else {
            self.evaluate_expr(&cond.else_expr).await
        }
    }

    async fn evaluate_unary_update(
        &mut self,
        update: &UnaryUpdateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let Expr::Identifier(ident) = &*update.operand {
            let current = self.environment.get(&ident.name, ident.position)?;

            match current {
                RuntimeValue::Int(v) => {
                    let new_value = match update.operator {
                        UpdateOperator::Increment => v.value + 1,
                        UpdateOperator::Decrement => v.value - 1,
                    };

                    let new_runtime_value = RuntimeValue::Int(IntValue::new(new_value));
                    self.environment.assign(
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
                    self.environment.assign(
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
                    self.file.clone(),
                )),
            }
        } else {
            Err(RaccoonError::new(
                "Invalid target for update operator".to_string(),
                update.position,
                self.file.clone(),
            ))
        }
    }

    async fn evaluate_template_str(
        &mut self,
        template: &TemplateStrExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut result = String::new();

        for part in &template.parts {
            match part {
                TemplateStrPart::String(s) => result.push_str(&s.value),
                TemplateStrPart::Expr(expr) => {
                    let value = self.evaluate_expr(expr).await?;
                    result.push_str(&value.to_string());
                }
            }
        }

        Ok(RuntimeValue::Str(StrValue::new(result)))
    }

    async fn evaluate_arrow_fn(
        &mut self,
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
            is_variadic: false,
        }));

        Ok(RuntimeValue::Function(FunctionValue::new(
            arrow.parameters.clone(),
            body,
            arrow.is_async,
            fn_type,
        )))
    }

    fn is_truthy(&self, value: &RuntimeValue) -> bool {
        match value {
            RuntimeValue::Bool(b) => b.value,
            RuntimeValue::Null(_) => false,
            RuntimeValue::Int(i) => i.value != 0,
            RuntimeValue::Float(f) => f.value != 0.0,
            RuntimeValue::Str(s) => !s.value.is_empty(),
            _ => true,
        }
    }

    async fn evaluate_typeof_expr(
        &mut self,
        typeof_expr: &TypeOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&typeof_expr.operand).await?;

        let type_name = match value {
            RuntimeValue::Int(_) => "int",
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
        };

        Ok(RuntimeValue::Str(StrValue::new(type_name.to_string())))
    }

    async fn evaluate_instanceof_expr(
        &mut self,
        instanceof: &InstanceOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&instanceof.operand).await?;

        if let RuntimeValue::ClassInstance(instance) = value {
            Ok(RuntimeValue::Bool(BoolValue::new(
                instance.class_name == instanceof.type_name,
            )))
        } else {
            Ok(RuntimeValue::Bool(BoolValue::new(false)))
        }
    }

    async fn evaluate_optional_chaining(
        &mut self,
        opt_chain: &OptionalChainingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&opt_chain.object).await?;

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
        &mut self,
        null_assert: &NullAssertionExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&null_assert.operand).await?;

        if matches!(value, RuntimeValue::Null(_)) {
            Err(RaccoonError::new(
                "Null assertion failed: value is null".to_string(),
                null_assert.position,
                self.file.clone(),
            ))
        } else {
            Ok(value)
        }
    }

    async fn evaluate_this_expr(&mut self) -> Result<RuntimeValue, RaccoonError> {
        match self.environment.get("this", (0, 0)) {
            Ok(value) => Ok(value),
            Err(_) => Err(RaccoonError::new(
                "Cannot use 'this' outside of a class method".to_string(),
                (0, 0),
                self.file.clone(),
            )),
        }
    }

    async fn evaluate_super_expr(&mut self) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            "Cannot use 'super' outside of a class method".to_string(),
            (0, 0),
            self.file.clone(),
        ))
    }

    async fn evaluate_super_call(&mut self, args: &[Expr]) -> Result<RuntimeValue, RaccoonError> {
        let this_instance = self.environment.get("this", (0, 0))?;

        if let RuntimeValue::ClassInstance(instance) = this_instance {
            let class_name = instance.class_name.clone();

            let class_value = self.environment.get(&class_name, (0, 0))?;

            if let RuntimeValue::Class(class) = class_value {
                if let Some(ref superclass_name) = class.declaration.superclass {
                    let superclass_value = self.environment.get(superclass_name, (0, 0))?;

                    if let RuntimeValue::Class(superclass) = superclass_value {
                        if let Some(ref super_constructor) = superclass.declaration.constructor {
                            let mut arg_values = Vec::new();
                            for arg in args {
                                arg_values.push(self.evaluate_expr(arg).await?);
                            }

                            self.environment.push_scope();

                            for (param, arg) in
                                super_constructor.parameters.iter().zip(arg_values.iter())
                            {
                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        self.environment.declare(name.clone(), arg.clone())?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        if let Err(e) =
                                            self.destructure_pattern(pattern, arg, (0, 0)).await
                                        {
                                            self.environment.pop_scope();
                                            return Err(e);
                                        }
                                    }
                                }
                            }

                            self.environment.declare(
                                "this".to_string(),
                                RuntimeValue::ClassInstance(instance.clone()),
                            )?;

                            for stmt in &super_constructor.body {
                                if let Stmt::ExprStmt(expr_stmt) = stmt {
                                    if let Expr::Assignment(assign) = &expr_stmt.expression {
                                        if let Expr::Member(member) = &*assign.target {
                                            if let Expr::This(_) = &*member.object {
                                                let value =
                                                    self.evaluate_expr(&assign.value).await?;
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

                                match self.execute_stmt_internal(stmt).await? {
                                    InterpreterResult::Return(_) => break,
                                    _ => {}
                                }
                            }

                            self.environment.pop_scope();

                            return Ok(RuntimeValue::Null(NullValue::new()));
                        } else {
                            return Err(RaccoonError::new(
                                format!("Superclass '{}' has no constructor", superclass_name),
                                (0, 0),
                                self.file.clone(),
                            ));
                        }
                    } else {
                        return Err(RaccoonError::new(
                            format!("'{}' is not a class", superclass_name),
                            (0, 0),
                            self.file.clone(),
                        ));
                    }
                } else {
                    return Err(RaccoonError::new(
                        "Cannot use 'super' in class without superclass".to_string(),
                        (0, 0),
                        self.file.clone(),
                    ));
                }
            } else {
                return Err(RaccoonError::new(
                    format!("Class '{}' not found", class_name),
                    (0, 0),
                    self.file.clone(),
                ));
            }
        } else {
            return Err(RaccoonError::new(
                "Cannot use 'super' outside of a class constructor".to_string(),
                (0, 0),
                self.file.clone(),
            ));
        }
    }

    async fn evaluate_range_expr(
        &mut self,
        range: &RangeExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let start = self.evaluate_expr(&range.start).await?;
        let end = self.evaluate_expr(&range.end).await?;

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
                self.file.clone(),
            )),
        }
    }

    async fn evaluate_null_coalescing(
        &mut self,
        null_coal: &NullCoalescingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let left = self.evaluate_expr(&null_coal.left).await?;

        if matches!(left, RuntimeValue::Null(_)) {
            self.evaluate_expr(&null_coal.right).await
        } else {
            Ok(left)
        }
    }

    async fn evaluate_new_expr(
        &mut self,
        new_expr: &NewExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if new_expr.class_name == "Map" {
            if new_expr.type_args.len() != 2 {
                return Err(RaccoonError::new(
                    "Map requires exactly two type arguments".to_string(),
                    new_expr.position,
                    self.file.clone(),
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

        let class_value = self
            .environment
            .get(&new_expr.class_name, new_expr.position)?;

        match class_value {
            RuntimeValue::Class(class) => {
                let mut properties = HashMap::new();
                let mut methods = HashMap::new();

                if let Some(ref superclass_name) = class.declaration.superclass {
                    if let Ok(RuntimeValue::Class(superclass)) =
                        self.environment.get(superclass_name, new_expr.position)
                    {
                        for prop in &superclass.declaration.properties {
                            let value = if let Some(init) = &prop.initializer {
                                self.evaluate_expr(init).await?
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
                                        return_type: method.return_type.clone(),
                                        is_variadic: false,
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
                        self.evaluate_expr(init).await?
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
                            return_type: method.return_type.clone(),
                            is_variadic: false,
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
                    if let Ok(RuntimeValue::Class(superclass)) =
                        self.environment.get(superclass_name, new_expr.position)
                    {
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
                        args.push(self.evaluate_expr(arg).await?);
                    }

                    self.environment.push_scope();

                    for (param, arg) in constructor.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)).await
                                {
                                    self.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    self.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    for stmt in &constructor.body {
                        if let Stmt::ExprStmt(expr_stmt) = stmt {
                            if let Expr::Assignment(assign) = &expr_stmt.expression {
                                if let Expr::Member(member) = &*assign.target {
                                    if let Expr::This(_) = &*member.object {
                                        let value = self.evaluate_expr(&assign.value).await?;
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

                        match self.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Return(_) => break,
                            _ => {}
                        }
                    }

                    self.environment.pop_scope();
                }

                Ok(RuntimeValue::ClassInstance(instance))
            }
            _ => Err(RaccoonError::new(
                format!(
                    "Class '{}' not found or not yet implemented",
                    new_expr.class_name
                ),
                new_expr.position,
                self.file.clone(),
            )),
        }
    }

    async fn evaluate_tagged_template(
        &mut self,
        tagged: &TaggedTemplateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let tag = self.evaluate_expr(&tagged.tag).await?;

        let mut strings = Vec::new();
        let mut values = Vec::new();

        for part in &tagged.template.parts {
            match part {
                TemplateStrPart::String(s) => {
                    strings.push(RuntimeValue::Str(StrValue::new(s.value.clone())));
                }
                TemplateStrPart::Expr(expr) => {
                    values.push(self.evaluate_expr(expr).await?);
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
                self.environment.push_scope();

                for (param, arg) in func.parameters.iter().zip(args.iter()) {
                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            self.environment.declare(name.clone(), arg.clone())?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)).await {
                                self.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
                    match self.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(v) => result = v,
                        InterpreterResult::Return(v) => {
                            self.environment.pop_scope();
                            return Ok(v);
                        }
                        _ => {
                            self.environment.pop_scope();
                            return Err(RaccoonError::new(
                                "Unexpected break/continue in function".to_string(),
                                (0, 0),
                                self.file.clone(),
                            ));
                        }
                    }
                }

                self.environment.pop_scope();
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
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn evaluate_method_call(
        &mut self,
        method_call: &MethodCallExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut object = self.evaluate_expr(&method_call.object).await?;

        let mut args = Vec::new();
        for arg in &method_call.args {
            args.push(self.evaluate_expr(arg).await?);
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
                    self.environment.push_scope();

                    let is_async = static_method.is_async;
                    let fn_type = static_method.fn_type.clone();

                    for (param, arg) in static_method.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)).await
                                {
                                    self.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &static_method.body {
                        match self.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Value(v) => result = v,
                            InterpreterResult::Return(v) => {
                                self.environment.pop_scope();

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
                                self.environment.pop_scope();
                                return Err(RaccoonError::new(
                                    "Unexpected break/continue in function".to_string(),
                                    (0, 0),
                                    self.file.clone(),
                                ));
                            }
                        }
                    }

                    self.environment.pop_scope();

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
                        self.file.clone(),
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
                    self.handle_list_functional_method(
                        &mut object,
                        &method_call.method,
                        args,
                        method_call.position,
                    )
                    .await
                } else {
                    self.type_registry.call_instance_method(
                        &mut object,
                        &method_call.method,
                        args,
                        method_call.position,
                        self.file.clone(),
                    )
                }
            }
            RuntimeValue::Str(_)
            | RuntimeValue::Map(_)
            | RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Decimal(_)
            | RuntimeValue::Bool(_) => self.type_registry.call_instance_method(
                &mut object,
                &method_call.method,
                args,
                method_call.position,
                self.file.clone(),
            ),
            RuntimeValue::Object(obj) => {
                if let Some(method) = obj.properties.get(&method_call.method) {
                    match method {
                        RuntimeValue::Function(func) => {
                            self.environment.push_scope();

                            let is_async = func.is_async;
                            let fn_type = func.fn_type.clone();

                            for (param, arg) in func.parameters.iter().zip(args.iter()) {
                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        self.environment.declare(name.clone(), arg.clone())?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        if let Err(e) =
                                            self.destructure_pattern(pattern, arg, (0, 0)).await
                                        {
                                            self.environment.pop_scope();
                                            return Err(e);
                                        }
                                    }
                                }
                            }

                            let mut result = RuntimeValue::Null(NullValue::new());
                            for stmt in &func.body {
                                match self.execute_stmt_internal(stmt).await? {
                                    InterpreterResult::Value(v) => result = v,
                                    InterpreterResult::Return(v) => {
                                        self.environment.pop_scope();

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
                                        self.environment.pop_scope();
                                        return Err(RaccoonError::new(
                                            "Unexpected break/continue in function".to_string(),
                                            (0, 0),
                                            self.file.clone(),
                                        ));
                                    }
                                }
                            }

                            self.environment.pop_scope();

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
                            self.file.clone(),
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
                        self.file.clone(),
                    ))
                }
            }
            RuntimeValue::ClassInstance(instance) => {
                if let Some(method) = instance.methods.get(&method_call.method) {
                    self.environment.push_scope();

                    self.environment.declare(
                        "this".to_string(),
                        RuntimeValue::ClassInstance(instance.clone()),
                    )?;

                    let is_async = method.is_async;
                    let fn_type = method.fn_type.clone();

                    for (param, arg) in method.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)).await
                                {
                                    self.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &method.body {
                        match self.execute_stmt_internal(stmt).await? {
                            InterpreterResult::Value(v) => result = v,
                            InterpreterResult::Return(v) => {
                                self.environment.pop_scope();

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
                                self.environment.pop_scope();
                                return Err(RaccoonError::new(
                                    "Unexpected break/continue in function".to_string(),
                                    (0, 0),
                                    self.file.clone(),
                                ));
                            }
                        }
                    }

                    self.environment.pop_scope();

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
                        self.file.clone(),
                    ))
                }
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on type", method_call.method),
                method_call.position,
                self.file.clone(),
            )),
        };

        let should_update = matches!(object, RuntimeValue::List(_) | RuntimeValue::Map(_));

        if should_update {
            if let Some((name, position)) = var_info {
                self.environment.assign(&name, object.clone(), position)?;
            }

            if let Some(property_name) = member_info {
                if let Ok(RuntimeValue::ClassInstance(instance)) =
                    self.environment.get("this", method_call.position)
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

    async fn destructure_pattern(
        &mut self,
        pattern: &DestructuringPattern,
        value: &RuntimeValue,
        position: Position,
    ) -> Result<(), RaccoonError> {
        match pattern {
            DestructuringPattern::List(list_pattern) => {
                self.destructure_list_pattern(list_pattern, value, position)
                    .await
            }
            DestructuringPattern::Object(obj_pattern) => {
                self.destructure_object_pattern(obj_pattern, value, position)
                    .await
            }
        }
    }

    #[async_recursion(?Send)]
    async fn destructure_list_pattern(
        &mut self,
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
                    self.file.clone(),
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
                        self.file.clone(),
                    ));
                }

                match elem_pat {
                    ListPatternElement::Identifier(id) => {
                        self.environment
                            .declare(id.name.clone(), elements[index].clone())?;
                    }
                    ListPatternElement::List(nested_list) => {
                        self.destructure_list_pattern(nested_list, &elements[index], position)
                            .await?;
                    }
                    ListPatternElement::Object(nested_obj) => {
                        self.destructure_object_pattern(nested_obj, &elements[index], position)
                            .await?;
                    }
                }
            }
            index += 1;
        }

        if let Some(rest) = &pattern.rest {
            let remaining: Vec<RuntimeValue> = elements[index..].to_vec();
            let rest_value = RuntimeValue::List(ListValue::new(remaining, PrimitiveType::any()));
            self.environment
                .declare(rest.argument.name.clone(), rest_value)?;
        }

        Ok(())
    }

    #[async_recursion(?Send)]
    async fn destructure_object_pattern(
        &mut self,
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
                        self.file.clone(),
                    ));
                }
            };

            match &prop.value {
                ObjectPatternValue::Identifier(id) => {
                    self.environment.declare(id.name.clone(), prop_value)?;
                }
                ObjectPatternValue::List(nested_list) => {
                    self.destructure_list_pattern(nested_list, &prop_value, position)
                        .await?;
                }
                ObjectPatternValue::Object(nested_obj) => {
                    self.destructure_object_pattern(nested_obj, &prop_value, position)
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
            self.environment
                .declare(rest.argument.name.clone(), rest_value)?;
        }

        Ok(())
    }

    async fn execute_import_decl(
        &mut self,
        import_decl: &ImportDecl,
    ) -> Result<InterpreterResult, RaccoonError> {
        let module_spec = &import_decl.module_specifier;

        if let Some(namespace_name) = &import_decl.namespace_import {
            let namespace_obj = self.get_module_namespace(module_spec).await?;
            self.environment
                .declare(namespace_name.clone(), namespace_obj)?;
            return Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )));
        }

        for spec in &import_decl.named_imports {
            let imported_name = &spec.imported;
            let local_name = spec.local.as_ref().unwrap_or(imported_name);

            let value = self.get_module_export(module_spec, imported_name).await?;
            self.environment.declare(local_name.clone(), value)?;
        }

        if let Some(default_name) = &import_decl.default_import {
            let value = self.get_module_export(module_spec, "default").await?;
            self.environment.declare(default_name.clone(), value)?;
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    async fn get_module_namespace(&self, module_spec: &str) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            if self.stdlib_loader.module_exists(module_spec) {
                return self.stdlib_loader.load_module(module_spec).await;
            } else {
                return Err(RaccoonError::new(
                    format!("Unknown module: {}", module_spec),
                    (0, 0),
                    self.file.clone(),
                ));
            }
        } else {
            Err(RaccoonError::new(
                format!(
                    "Module system only supports std: modules currently: {}",
                    module_spec
                ),
                (0, 0),
                self.file.clone(),
            ))
        }
    }

    async fn get_module_export(
        &self,
        module_spec: &str,
        export_name: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            if self.stdlib_loader.module_exists(module_spec) {
                return self
                    .stdlib_loader
                    .get_module_export(module_spec, export_name)
                    .await;
            } else {
                return Err(RaccoonError::new(
                    format!("Unknown module: {}", module_spec),
                    (0, 0),
                    self.file.clone(),
                ));
            }
        } else {
            Err(RaccoonError::new(
                format!(
                    "Module system only supports std: modules currently: {}",
                    module_spec
                ),
                (0, 0),
                self.file.clone(),
            ))
        }
    }

    async fn evaluate_await_expr(
        &mut self,
        await_expr: &AwaitExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let future_value = self.evaluate_expr(&await_expr.expression).await?;

        match future_value {
            RuntimeValue::Future(future) => {
                let state = future.state.read().unwrap();
                match &*state {
                    FutureState::Resolved(value) => Ok((**value).clone()),
                    FutureState::Rejected(error) => Err(RaccoonError::new(
                        format!("Future rejected: {}", error),
                        await_expr.position,
                        self.file.clone(),
                    )),
                    FutureState::Pending => Err(RaccoonError::new(
                        "Cannot await pending future (async runtime not fully implemented)"
                            .to_string(),
                        await_expr.position,
                        self.file.clone(),
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                format!("Cannot await non-future value: {}", future_value.get_name()),
                await_expr.position,
                self.file.clone(),
            )),
        }
    }

    #[async_recursion(?Send)]
    async fn call_function(
        &mut self,
        func: &RuntimeValue,
        args: Vec<RuntimeValue>,
        position: Position,
    ) -> Result<RuntimeValue, RaccoonError> {
        match func {
            RuntimeValue::Function(fn_val) => {
                self.environment.push_scope();

                for (i, param) in fn_val.parameters.iter().enumerate() {
                    let value = if i < args.len() {
                        args[i].clone()
                    } else if let Some(default_expr) = &param.default_value {
                        self.evaluate_expr(default_expr).await?
                    } else {
                        self.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter {}", i),
                            position,
                            self.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            self.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) =
                                self.destructure_pattern(pattern, &value, position).await
                            {
                                self.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &fn_val.body {
                    match self.execute_stmt_internal(stmt).await? {
                        InterpreterResult::Value(v) => result = v,
                        InterpreterResult::Return(v) => {
                            self.environment.pop_scope();
                            return Ok(v);
                        }
                        _ => {
                            self.environment.pop_scope();
                            return Err(RaccoonError::new(
                                "Unexpected break/continue in function".to_string(),
                                position,
                                self.file.clone(),
                            ));
                        }
                    }
                }

                self.environment.pop_scope();
                Ok(result)
            }
            RuntimeValue::NativeFunction(fn_val) => Ok((fn_val.implementation)(args)),
            RuntimeValue::NativeAsyncFunction(fn_val) => {
                let result = (fn_val.implementation)(args).await;
                let return_type = match &fn_val.fn_type {
                    Type::Function(fn_type) => fn_type.return_type.clone(),
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
                self.file.clone(),
            )),
        }
    }

    async fn handle_list_functional_method(
        &mut self,
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
                    self.file.clone(),
                ));
            }
        };

        match method {
            "map" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "map requires a callback function".to_string(),
                        position,
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                let mut mapped = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                let mut filtered = Vec::new();

                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
                            callback,
                            vec![
                                element.clone(),
                                RuntimeValue::Int(IntValue::new(index as i64)),
                            ],
                            position,
                        )
                        .await?;

                    if self.is_truthy(&result) {
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                if list.elements.is_empty() && args.len() < 2 {
                    return Err(RaccoonError::new(
                        "reduce of empty array with no initial value".to_string(),
                        position,
                        self.file.clone(),
                    ));
                }

                let mut accumulator = if args.len() >= 2 {
                    args[1].clone()
                } else {
                    list.elements[0].clone()
                };

                let start_index = if args.len() >= 2 { 0 } else { 1 };

                for (index, element) in list.elements.iter().enumerate().skip(start_index) {
                    accumulator = self
                        .call_function(
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    self.call_function(
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
                            callback,
                            vec![
                                element.clone(),
                                RuntimeValue::Int(IntValue::new(index as i64)),
                            ],
                            position,
                        )
                        .await?;

                    if self.is_truthy(&result) {
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
                            callback,
                            vec![
                                element.clone(),
                                RuntimeValue::Int(IntValue::new(index as i64)),
                            ],
                            position,
                        )
                        .await?;

                    if self.is_truthy(&result) {
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
                            callback,
                            vec![
                                element.clone(),
                                RuntimeValue::Int(IntValue::new(index as i64)),
                            ],
                            position,
                        )
                        .await?;

                    if self.is_truthy(&result) {
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
                        self.file.clone(),
                    ));
                }

                let callback = &args[0];
                for (index, element) in list.elements.iter().enumerate() {
                    let result = self
                        .call_function(
                            callback,
                            vec![
                                element.clone(),
                                RuntimeValue::Int(IntValue::new(index as i64)),
                            ],
                            position,
                        )
                        .await?;

                    if !self.is_truthy(&result) {
                        return Ok(RuntimeValue::Bool(BoolValue::new(false)));
                    }
                }
                Ok(RuntimeValue::Bool(BoolValue::new(true)))
            }
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on list", method),
                position,
                self.file.clone(),
            )),
        }
    }

    pub async fn execute_stmt(&mut self, stmt: &Stmt) -> Result<RuntimeValue, RaccoonError> {
        match self.execute_stmt_internal(stmt).await? {
            InterpreterResult::Value(v) => Ok(v),
            InterpreterResult::Return(v) => Ok(v),
            _ => Ok(RuntimeValue::Null(NullValue::new())),
        }
    }

    pub async fn eval_expr_public(&mut self, expr: &Expr) -> Result<RuntimeValue, RaccoonError> {
        self.evaluate_expr(expr).await
    }

    pub fn get_from_env(&self, name: &str) -> Result<RuntimeValue, RaccoonError> {
        self.environment.get(name, (0, 0))
    }

    pub fn declare_in_env(
        &mut self,
        name: String,
        value: RuntimeValue,
    ) -> Result<(), RaccoonError> {
        self.environment.declare(name, value)
    }
}
