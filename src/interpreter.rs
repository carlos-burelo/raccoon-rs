use crate::ast::nodes::*;
use crate::ast::types::{PrimitiveType, Type};
use crate::error::RaccoonError;
use crate::runtime::{
    BoolValue, Environment, FloatValue, FunctionValue, IntValue, ListValue, ModuleRegistry,
    NullValue, ObjectValue, RuntimeValue, StrValue, TypeRegistry,
};
use crate::tokens::{BinaryOperator, Position};
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
    #[allow(dead_code)]
    module_registry: ModuleRegistry,
    type_registry: TypeRegistry,
}

impl Interpreter {
    pub fn new(file: Option<String>) -> Self {
        let mut env = Environment::new(file.clone());
        let module_registry = ModuleRegistry::new();
        let type_registry = TypeRegistry::new();
        Self::setup_global_functions(&mut env);
        Self {
            environment: env,
            file,
            module_registry,
            type_registry,
        }
    }

    fn setup_global_functions(env: &mut Environment) {
        let print_fn = RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", arg.to_string());
                }
                println!();
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        ));
        let _ = env.declare("print".to_string(), print_fn);

        let len_fn = RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
                    RuntimeValue::List(l) => {
                        RuntimeValue::Int(IntValue::new(l.elements.len() as i64))
                    }
                    RuntimeValue::Map(m) => {
                        RuntimeValue::Int(IntValue::new(m.entries.len() as i64))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::any()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ));
        let _ = env.declare("len".to_string(), len_fn);
    }

    pub fn interpret(&mut self, program: &Program) -> Result<RuntimeValue, RaccoonError> {
        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &program.stmts {
            match self.execute_stmt_internal(stmt) {
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

    fn execute_stmt_internal(&mut self, stmt: &Stmt) -> Result<InterpreterResult, RaccoonError> {
        match stmt {
            Stmt::Program(program) => self.interpret(program).map(InterpreterResult::Value),
            Stmt::VarDecl(decl) => self.execute_var_decl(decl).map(InterpreterResult::Value),
            Stmt::FnDecl(decl) => self.execute_fn_decl(decl).map(InterpreterResult::Value),
            Stmt::Block(block) => self.execute_block_internal(block),
            Stmt::IfStmt(if_stmt) => self.execute_if_stmt_internal(if_stmt),
            Stmt::WhileStmt(while_stmt) => self.execute_while_stmt(while_stmt),
            Stmt::ForStmt(for_stmt) => self.execute_for_stmt(for_stmt),
            Stmt::ForInStmt(for_in) => self.execute_for_in_stmt(for_in),
            Stmt::ReturnStmt(ret) => self.execute_return_stmt(ret),
            Stmt::BreakStmt(_) => Ok(InterpreterResult::Break),
            Stmt::ContinueStmt(_) => Ok(InterpreterResult::Continue),
            Stmt::ExprStmt(expr_stmt) => self
                .evaluate_expr(&expr_stmt.expression)
                .map(InterpreterResult::Value),
            Stmt::ClassDecl(decl) => self.execute_class_decl(decl).map(InterpreterResult::Value),
            Stmt::InterfaceDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::EnumDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TypeAliasDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::ImportDecl(import_decl) => self.execute_import_decl(import_decl),
            Stmt::ExportDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TryStmt(try_stmt) => self.execute_try_stmt(try_stmt),
            Stmt::ThrowStmt(throw) => self.execute_throw_stmt(throw),
        }
    }

    fn execute_var_decl(&mut self, decl: &VarDecl) -> Result<RuntimeValue, RaccoonError> {
        let value = if let Some(init) = &decl.initializer {
            self.evaluate_expr(init)?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        match &decl.pattern {
            VarPattern::Identifier(name) => {
                self.environment.declare(name.clone(), value.clone())?;
            }
            VarPattern::Destructuring(pattern) => {
                self.destructure_pattern(pattern, &value, decl.position)?;
            }
        }

        Ok(value)
    }

    fn execute_fn_decl(&mut self, decl: &FnDecl) -> Result<RuntimeValue, RaccoonError> {
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

    fn execute_class_decl(&mut self, decl: &ClassDecl) -> Result<RuntimeValue, RaccoonError> {
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
                    false,
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

    fn execute_block_internal(&mut self, block: &Block) -> Result<InterpreterResult, RaccoonError> {
        self.environment.push_scope();

        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &block.statements {
            match self.execute_stmt_internal(stmt)? {
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

    fn execute_if_stmt_internal(
        &mut self,
        if_stmt: &IfStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let condition = self.evaluate_expr(&if_stmt.condition)?;

        if self.is_truthy(&condition) {
            self.execute_stmt_internal(&if_stmt.then_branch)
        } else if let Some(else_branch) = &if_stmt.else_branch {
            self.execute_stmt_internal(else_branch)
        } else {
            Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )))
        }
    }

    fn execute_while_stmt(
        &mut self,
        while_stmt: &WhileStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        loop {
            let condition = self.evaluate_expr(&while_stmt.condition)?;
            if !self.is_truthy(&condition) {
                break;
            }

            match self.execute_stmt_internal(&while_stmt.body)? {
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

    fn execute_for_stmt(&mut self, for_stmt: &ForStmt) -> Result<InterpreterResult, RaccoonError> {
        self.environment.push_scope();

        if let Some(init) = &for_stmt.initializer {
            self.execute_stmt_internal(init)?;
        }

        loop {
            if let Some(condition) = &for_stmt.condition {
                let cond_value = self.evaluate_expr(condition)?;
                if !self.is_truthy(&cond_value) {
                    break;
                }
            }

            match self.execute_stmt_internal(&for_stmt.body)? {
                InterpreterResult::Value(_) => {}
                InterpreterResult::Break => break,
                InterpreterResult::Continue => {}
                InterpreterResult::Return(v) => {
                    self.environment.pop_scope();
                    return Ok(InterpreterResult::Return(v));
                }
            }

            if let Some(increment) = &for_stmt.increment {
                self.evaluate_expr(increment)?;
            }
        }

        self.environment.pop_scope();

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    fn execute_for_in_stmt(
        &mut self,
        for_in: &ForInStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let iterable = self.evaluate_expr(&for_in.iterable)?;

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
            self.environment.assign(&for_in.variable, element)?;

            match self.execute_stmt_internal(&for_in.body)? {
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

    fn execute_return_stmt(&mut self, ret: &ReturnStmt) -> Result<InterpreterResult, RaccoonError> {
        let value = if let Some(expr) = &ret.value {
            self.evaluate_expr(expr)?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        Ok(InterpreterResult::Return(value))
    }

    fn execute_try_stmt(&mut self, try_stmt: &TryStmt) -> Result<InterpreterResult, RaccoonError> {
        let result = self.execute_block_internal(&try_stmt.try_block);

        match result {
            Ok(value) => {
                if let Some(finally_block) = &try_stmt.finally_block {
                    self.execute_block_internal(finally_block)?;
                }
                Ok(value)
            }
            Err(error) => {
                for catch_clause in &try_stmt.catch_clauses {
                    self.environment.push_scope();
                    let error_value = RuntimeValue::Str(StrValue::new(error.message.clone()));
                    self.environment
                        .declare(catch_clause.error_var.clone(), error_value)?;

                    let result = self.execute_block_internal(&catch_clause.body);
                    self.environment.pop_scope();

                    if let Some(finally_block) = &try_stmt.finally_block {
                        self.execute_block_internal(finally_block)?;
                    }

                    return result;
                }

                if let Some(finally_block) = &try_stmt.finally_block {
                    self.execute_block_internal(finally_block)?;
                }

                Err(error)
            }
        }
    }

    fn execute_throw_stmt(&mut self, throw: &ThrowStmt) -> Result<InterpreterResult, RaccoonError> {
        let value = self.evaluate_expr(&throw.value)?;
        Err(RaccoonError::new(
            value.to_string(),
            throw.position,
            self.file.clone(),
        ))
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<RuntimeValue, RaccoonError> {
        match expr {
            Expr::IntLiteral(lit) => Ok(RuntimeValue::Int(IntValue::new(lit.value))),
            Expr::FloatLiteral(lit) => Ok(RuntimeValue::Float(FloatValue::new(lit.value))),
            Expr::StrLiteral(lit) => Ok(RuntimeValue::Str(StrValue::new(lit.value.clone()))),
            Expr::BoolLiteral(lit) => Ok(RuntimeValue::Bool(BoolValue::new(lit.value))),
            Expr::NullLiteral(_) => Ok(RuntimeValue::Null(NullValue::new())),
            Expr::Identifier(ident) => self.environment.get(&ident.name),
            Expr::Binary(binary) => self.evaluate_binary_expr(binary),
            Expr::Unary(unary) => self.evaluate_unary_expr(unary),
            Expr::Assignment(assign) => self.evaluate_assignment(assign),
            Expr::Call(call) => self.evaluate_call_expr(call),
            Expr::ListLiteral(list) => self.evaluate_list_literal(list),
            Expr::ObjectLiteral(obj) => self.evaluate_object_literal(obj),
            Expr::Member(member) => self.evaluate_member_expr(member),
            Expr::Index(index) => self.evaluate_index_expr(index),
            Expr::Conditional(cond) => self.evaluate_conditional_expr(cond),
            Expr::UnaryUpdate(update) => self.evaluate_unary_update(update),
            Expr::TemplateStr(template) => self.evaluate_template_str(template),
            Expr::ArrowFn(arrow) => self.evaluate_arrow_fn(arrow),
            Expr::TypeOf(typeof_expr) => self.evaluate_typeof_expr(typeof_expr),
            Expr::InstanceOf(instanceof) => self.evaluate_instanceof_expr(instanceof),
            Expr::OptionalChaining(opt_chain) => self.evaluate_optional_chaining(opt_chain),
            Expr::NullAssertion(null_assert) => self.evaluate_null_assertion(null_assert),
            Expr::MethodCall(method_call) => self.evaluate_method_call(method_call),
            Expr::This(_) => self.evaluate_this_expr(),
            Expr::Super(_) => self.evaluate_super_expr(),
            Expr::New(new_expr) => self.evaluate_new_expr(new_expr),
            Expr::TaggedTemplate(tagged) => self.evaluate_tagged_template(tagged),
            Expr::Range(range) => self.evaluate_range_expr(range),
            Expr::NullCoalescing(null_coal) => self.evaluate_null_coalescing(null_coal),
            _ => Err(RaccoonError::new(
                format!("Expression not yet implemented: {:?}", expr),
                (0, 0),
                self.file.clone(),
            )),
        }
    }

    fn apply_binary_op(
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

    fn evaluate_binary_expr(&mut self, binary: &BinaryExpr) -> Result<RuntimeValue, RaccoonError> {
        let left = self.evaluate_expr(&binary.left)?;
        let right = self.evaluate_expr(&binary.right)?;

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

    fn evaluate_unary_expr(&mut self, unary: &UnaryExpr) -> Result<RuntimeValue, RaccoonError> {
        let operand = self.evaluate_expr(&unary.operand)?;

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

    fn evaluate_assignment(&mut self, assign: &Assignment) -> Result<RuntimeValue, RaccoonError> {
        use crate::tokens::TokenType;

        let final_value = if assign.operator != TokenType::Assign {
            let current_value = self.evaluate_expr(&assign.target)?;
            let right_value = self.evaluate_expr(&assign.value)?;

            match assign.operator {
                TokenType::PlusAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Add,
                    assign.position,
                )?,
                TokenType::MinusAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Subtract,
                    assign.position,
                )?,
                TokenType::MultiplyAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Multiply,
                    assign.position,
                )?,
                TokenType::DivideAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Divide,
                    assign.position,
                )?,
                TokenType::ModuloAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Modulo,
                    assign.position,
                )?,
                TokenType::AmpersandAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::BitwiseAnd,
                    assign.position,
                )?,
                TokenType::BitwiseOrAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::BitwiseOr,
                    assign.position,
                )?,
                TokenType::BitwiseXorAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::BitwiseXor,
                    assign.position,
                )?,
                TokenType::LeftShiftAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::LeftShift,
                    assign.position,
                )?,
                TokenType::RightShiftAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::RightShift,
                    assign.position,
                )?,
                TokenType::UnsignedRightShiftAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::UnsignedRightShift,
                    assign.position,
                )?,
                TokenType::ExponentAssign => self.apply_binary_op(
                    current_value,
                    right_value,
                    BinaryOperator::Exponent,
                    assign.position,
                )?,
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
            self.evaluate_expr(&assign.value)?
        };

        match &*assign.target {
            Expr::Identifier(ident) => {
                self.environment.assign(&ident.name, final_value.clone())?;
                Ok(final_value)
            }
            Expr::Member(member) => {
                let mut object = self.evaluate_expr(&member.object)?;

                match &mut object {
                    RuntimeValue::Object(obj) => {
                        obj.properties
                            .insert(member.property.clone(), final_value.clone());
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
                                        )?;
                                    }
                                }
                            }

                            for stmt in &accessor.body {
                                match self.execute_stmt_internal(stmt)? {
                                    InterpreterResult::Return(_) => break,
                                    _ => {}
                                }
                            }

                            self.environment.pop_scope();
                            return Ok(final_value);
                        }

                        instance
                            .properties
                            .borrow_mut()
                            .insert(member.property.clone(), final_value.clone());
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
                let mut object = self.evaluate_expr(&index_expr.object)?;
                let idx = self.evaluate_expr(&index_expr.index)?;

                match (&mut object, idx) {
                    (RuntimeValue::List(list), RuntimeValue::Int(i)) => {
                        if i.value < 0 || i.value >= list.elements.len() as i64 {
                            return Err(RaccoonError::new(
                                format!("[1409] Index out of bounds: {}", i.value),
                                assign.position,
                                self.file.clone(),
                            ));
                        }
                        list.elements[i.value as usize] = final_value.clone();
                        Ok(final_value)
                    }
                    (RuntimeValue::Map(map), key) => {
                        map.entries.insert(key.to_string(), final_value.clone());
                        Ok(final_value)
                    }
                    _ => Err(RaccoonError::new(
                        "Invalid index assignment target".to_string(),
                        assign.position,
                        self.file.clone(),
                    )),
                }
            }
            _ => Err(RaccoonError::new(
                "Invalid assignment target".to_string(),
                assign.position,
                self.file.clone(),
            )),
        }
    }

    fn evaluate_call_expr(&mut self, call: &CallExpr) -> Result<RuntimeValue, RaccoonError> {
        if matches!(call.callee.as_ref(), Expr::Super(_)) {
            return self.evaluate_super_call(&call.args);
        }

        let callee = self.evaluate_expr(&call.callee)?;

        let mut args = Vec::new();
        for arg in &call.args {
            args.push(self.evaluate_expr(arg)?);
        }

        match callee {
            RuntimeValue::Function(func) => {
                self.environment.push_scope();

                for (i, param) in func.parameters.iter().enumerate() {
                    let value = if i < args.len() {
                        args[i].clone()
                    } else if let Some(default_expr) = &param.default_value {
                        self.evaluate_expr(default_expr)?
                    } else {
                        self.environment.pop_scope();
                        return Err(RaccoonError::new(
                            format!("Missing required argument for parameter {}", i),
                            (0, 0),
                            self.file.clone(),
                        ));
                    };

                    match &param.pattern {
                        VarPattern::Identifier(name) => {
                            self.environment.declare(name.clone(), value)?;
                        }
                        VarPattern::Destructuring(pattern) => {
                            if let Err(e) = self.destructure_pattern(pattern, &value, (0, 0)) {
                                self.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
                    match self.execute_stmt_internal(stmt)? {
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
            _ => Err(RaccoonError::new(
                "Attempted to call a non-function value".to_string(),
                (0, 0),
                self.file.clone(),
            )),
        }
    }

    fn evaluate_list_literal(&mut self, list: &ListLiteral) -> Result<RuntimeValue, RaccoonError> {
        let mut elements = Vec::new();
        for elem in &list.elements {
            elements.push(self.evaluate_expr(elem)?);
        }

        let element_type = if elements.is_empty() {
            PrimitiveType::any()
        } else {
            elements[0].get_type()
        };

        Ok(RuntimeValue::List(ListValue::new(elements, element_type)))
    }

    fn evaluate_object_literal(
        &mut self,
        obj: &ObjectLiteral,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut properties = HashMap::new();
        for (key, value) in &obj.properties {
            properties.insert(key.clone(), self.evaluate_expr(value)?);
        }

        Ok(RuntimeValue::Object(ObjectValue::new(
            properties,
            PrimitiveType::any(),
        )))
    }

    fn evaluate_member_expr(&mut self, member: &MemberExpr) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&member.object)?;

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
                if let Some(value) = instance.properties.borrow().get(&member.property) {
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
                        match self.execute_stmt_internal(stmt)? {
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

    fn evaluate_index_expr(&mut self, index: &IndexExpr) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&index.object)?;
        let idx = self.evaluate_expr(&index.index)?;

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
            _ => Err(RaccoonError::new(
                "Invalid index operation".to_string(),
                index.position,
                self.file.clone(),
            )),
        }
    }

    fn evaluate_conditional_expr(
        &mut self,
        cond: &ConditionalExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let condition = self.evaluate_expr(&cond.condition)?;

        if self.is_truthy(&condition) {
            self.evaluate_expr(&cond.then_expr)
        } else {
            self.evaluate_expr(&cond.else_expr)
        }
    }

    fn evaluate_unary_update(
        &mut self,
        update: &UnaryUpdateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let Expr::Identifier(ident) = &*update.operand {
            let current = self.environment.get(&ident.name)?;

            match current {
                RuntimeValue::Int(v) => {
                    let new_value = match update.operator {
                        UpdateOperator::Increment => v.value + 1,
                        UpdateOperator::Decrement => v.value - 1,
                    };

                    let new_runtime_value = RuntimeValue::Int(IntValue::new(new_value));
                    self.environment
                        .assign(&ident.name, new_runtime_value.clone())?;

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
                    self.environment
                        .assign(&ident.name, new_runtime_value.clone())?;

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

    fn evaluate_template_str(
        &mut self,
        template: &TemplateStrExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut result = String::new();

        for part in &template.parts {
            match part {
                TemplateStrPart::String(s) => result.push_str(&s.value),
                TemplateStrPart::Expr(expr) => {
                    let value = self.evaluate_expr(expr)?;
                    result.push_str(&value.to_string());
                }
            }
        }

        Ok(RuntimeValue::Str(StrValue::new(result)))
    }

    fn evaluate_arrow_fn(&mut self, arrow: &ArrowFnExpr) -> Result<RuntimeValue, RaccoonError> {
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

    fn evaluate_typeof_expr(
        &mut self,
        typeof_expr: &TypeOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&typeof_expr.operand)?;

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

    fn evaluate_instanceof_expr(
        &mut self,
        instanceof: &InstanceOfExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&instanceof.operand)?;

        if let RuntimeValue::ClassInstance(instance) = value {
            Ok(RuntimeValue::Bool(BoolValue::new(
                instance.class_name == instanceof.type_name,
            )))
        } else {
            Ok(RuntimeValue::Bool(BoolValue::new(false)))
        }
    }

    fn evaluate_optional_chaining(
        &mut self,
        opt_chain: &OptionalChainingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let object = self.evaluate_expr(&opt_chain.object)?;

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
                if let Some(value) = instance.properties.borrow().get(&opt_chain.property) {
                    Ok(value.clone())
                } else {
                    Ok(RuntimeValue::Null(NullValue::new()))
                }
            }
            _ => Ok(RuntimeValue::Null(NullValue::new())),
        }
    }

    fn evaluate_null_assertion(
        &mut self,
        null_assert: &NullAssertionExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = self.evaluate_expr(&null_assert.operand)?;

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

    fn evaluate_this_expr(&mut self) -> Result<RuntimeValue, RaccoonError> {
        match self.environment.get("this") {
            Ok(value) => Ok(value),
            Err(_) => Err(RaccoonError::new(
                "Cannot use 'this' outside of a class method".to_string(),
                (0, 0),
                self.file.clone(),
            )),
        }
    }

    fn evaluate_super_expr(&mut self) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            "Cannot use 'super' outside of a class method".to_string(),
            (0, 0),
            self.file.clone(),
        ))
    }

    fn evaluate_super_call(&mut self, args: &[Expr]) -> Result<RuntimeValue, RaccoonError> {
        let this_instance = self.environment.get("this")?;

        if let RuntimeValue::ClassInstance(instance) = this_instance {
            let class_name = instance.class_name.clone();

            let class_value = self.environment.get(&class_name)?;

            if let RuntimeValue::Class(class) = class_value {
                if let Some(ref superclass_name) = class.declaration.superclass {
                    let superclass_value = self.environment.get(superclass_name)?;

                    if let RuntimeValue::Class(superclass) = superclass_value {
                        if let Some(ref super_constructor) = superclass.declaration.constructor {
                            let mut arg_values = Vec::new();
                            for arg in args {
                                arg_values.push(self.evaluate_expr(arg)?);
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
                                            self.destructure_pattern(pattern, arg, (0, 0))
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
                                                let value = self.evaluate_expr(&assign.value)?;
                                                instance
                                                    .properties
                                                    .borrow_mut()
                                                    .insert(member.property.clone(), value);
                                                continue;
                                            }
                                        }
                                    }
                                }

                                match self.execute_stmt_internal(stmt)? {
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

    fn evaluate_range_expr(&mut self, range: &RangeExpr) -> Result<RuntimeValue, RaccoonError> {
        let start = self.evaluate_expr(&range.start)?;
        let end = self.evaluate_expr(&range.end)?;

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

    fn evaluate_null_coalescing(
        &mut self,
        null_coal: &NullCoalescingExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let left = self.evaluate_expr(&null_coal.left)?;

        if matches!(left, RuntimeValue::Null(_)) {
            self.evaluate_expr(&null_coal.right)
        } else {
            Ok(left)
        }
    }

    fn evaluate_new_expr(&mut self, new_expr: &NewExpr) -> Result<RuntimeValue, RaccoonError> {
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

        let class_value = self.environment.get(&new_expr.class_name)?;

        match class_value {
            RuntimeValue::Class(class) => {
                let mut properties = HashMap::new();
                let mut methods = HashMap::new();

                if let Some(ref superclass_name) = class.declaration.superclass {
                    if let Ok(RuntimeValue::Class(superclass)) =
                        self.environment.get(superclass_name)
                    {
                        for prop in &superclass.declaration.properties {
                            let value = if let Some(init) = &prop.initializer {
                                self.evaluate_expr(init)?
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
                        self.evaluate_expr(init)?
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
                        self.environment.get(superclass_name)
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
                        args.push(self.evaluate_expr(arg)?);
                    }

                    self.environment.push_scope();

                    for (param, arg) in constructor.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)) {
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
                                        let value = self.evaluate_expr(&assign.value)?;
                                        instance
                                            .properties
                                            .borrow_mut()
                                            .insert(member.property.clone(), value);
                                        continue;
                                    }
                                }
                            }
                        }

                        match self.execute_stmt_internal(stmt)? {
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

    fn evaluate_tagged_template(
        &mut self,
        tagged: &TaggedTemplateExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let tag = self.evaluate_expr(&tagged.tag)?;

        let mut strings = Vec::new();
        let mut values = Vec::new();

        for part in &tagged.template.parts {
            match part {
                TemplateStrPart::String(s) => {
                    strings.push(RuntimeValue::Str(StrValue::new(s.value.clone())));
                }
                TemplateStrPart::Expr(expr) => {
                    values.push(self.evaluate_expr(expr)?);
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
                            if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)) {
                                self.environment.pop_scope();
                                return Err(e);
                            }
                        }
                    }
                }

                let mut result = RuntimeValue::Null(NullValue::new());
                for stmt in &func.body {
                    match self.execute_stmt_internal(stmt)? {
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
            _ => Err(RaccoonError::new(
                "Tagged template tag must be a function".to_string(),
                tagged.position,
                self.file.clone(),
            )),
        }
    }

    fn evaluate_method_call(
        &mut self,
        method_call: &MethodCallExpr,
    ) -> Result<RuntimeValue, RaccoonError> {
        let mut object = self.evaluate_expr(&method_call.object)?;

        let mut args = Vec::new();
        for arg in &method_call.args {
            args.push(self.evaluate_expr(arg)?);
        }

        let var_name = if let Expr::Identifier(ident) = method_call.object.as_ref() {
            Some(ident.name.clone())
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

                    for (param, arg) in static_method.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)) {
                                    self.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &static_method.body {
                        match self.execute_stmt_internal(stmt)? {
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

            RuntimeValue::Str(_)
            | RuntimeValue::List(_)
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

                            for (param, arg) in func.parameters.iter().zip(args.iter()) {
                                match &param.pattern {
                                    VarPattern::Identifier(name) => {
                                        self.environment.declare(name.clone(), arg.clone())?;
                                    }
                                    VarPattern::Destructuring(pattern) => {
                                        if let Err(e) =
                                            self.destructure_pattern(pattern, arg, (0, 0))
                                        {
                                            self.environment.pop_scope();
                                            return Err(e);
                                        }
                                    }
                                }
                            }

                            let mut result = RuntimeValue::Null(NullValue::new());
                            for stmt in &func.body {
                                match self.execute_stmt_internal(stmt)? {
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

                    for (param, arg) in method.parameters.iter().zip(args.iter()) {
                        match &param.pattern {
                            VarPattern::Identifier(name) => {
                                self.environment.declare(name.clone(), arg.clone())?;
                            }
                            VarPattern::Destructuring(pattern) => {
                                if let Err(e) = self.destructure_pattern(pattern, arg, (0, 0)) {
                                    self.environment.pop_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }

                    let mut result = RuntimeValue::Null(NullValue::new());
                    for stmt in &method.body {
                        match self.execute_stmt_internal(stmt)? {
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
            if let Some(name) = var_name {
                self.environment.assign(&name, object.clone())?;
            }

            if let Some(property_name) = member_info {
                if let Ok(RuntimeValue::ClassInstance(instance)) = self.environment.get("this") {
                    instance
                        .properties
                        .borrow_mut()
                        .insert(property_name, object);
                }
            }
        }

        result
    }

    fn destructure_pattern(
        &mut self,
        pattern: &DestructuringPattern,
        value: &RuntimeValue,
        position: Position,
    ) -> Result<(), RaccoonError> {
        match pattern {
            DestructuringPattern::List(list_pattern) => {
                self.destructure_list_pattern(list_pattern, value, position)
            }
            DestructuringPattern::Object(obj_pattern) => {
                self.destructure_object_pattern(obj_pattern, value, position)
            }
        }
    }

    fn destructure_list_pattern(
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
                        self.destructure_list_pattern(nested_list, &elements[index], position)?;
                    }
                    ListPatternElement::Object(nested_obj) => {
                        self.destructure_object_pattern(nested_obj, &elements[index], position)?;
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

    fn destructure_object_pattern(
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
                    .borrow()
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
                    self.destructure_list_pattern(nested_list, &prop_value, position)?;
                }
                ObjectPatternValue::Object(nested_obj) => {
                    self.destructure_object_pattern(nested_obj, &prop_value, position)?;
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
                    for (key, val) in inst.properties.borrow().iter() {
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

    fn execute_import_decl(
        &mut self,
        import_decl: &ImportDecl,
    ) -> Result<InterpreterResult, RaccoonError> {
        let module_spec = &import_decl.module_specifier;

        if let Some(namespace_name) = &import_decl.namespace_import {
            let namespace_obj = self.get_module_namespace(module_spec)?;
            self.environment
                .declare(namespace_name.clone(), namespace_obj)?;
            return Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            )));
        }

        for spec in &import_decl.named_imports {
            let imported_name = &spec.imported;
            let local_name = spec.local.as_ref().unwrap_or(imported_name);

            let value = self.get_module_export(module_spec, imported_name)?;
            self.environment.declare(local_name.clone(), value)?;
        }

        if let Some(default_name) = &import_decl.default_import {
            let value = self.get_module_export(module_spec, "default")?;
            self.environment.declare(default_name.clone(), value)?;
        }

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    fn get_module_namespace(&self, module_spec: &str) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            let properties = crate::runtime::std::ModuleRegistry::get_module_exports(module_spec)
                .ok_or_else(|| {
                RaccoonError::new(
                    format!("Unknown module: {}", module_spec),
                    (0, 0),
                    self.file.clone(),
                )
            })?;

            Ok(RuntimeValue::Object(ObjectValue::new(
                properties,
                PrimitiveType::any(),
            )))
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

    fn get_module_export(
        &self,
        module_spec: &str,
        export_name: &str,
    ) -> Result<RuntimeValue, RaccoonError> {
        if module_spec.starts_with("std:") {
            crate::runtime::std::ModuleRegistry::get_module_export(module_spec, export_name)
                .ok_or_else(|| {
                    RaccoonError::new(
                        format!("{} does not export '{}'", module_spec, export_name),
                        (0, 0),
                        self.file.clone(),
                    )
                })
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
}
