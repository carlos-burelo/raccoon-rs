pub mod builtins;
pub mod control_flow;
pub mod declarations;
pub mod expressions;
pub mod helpers;
pub mod module_loader;
pub mod operators;

use crate::ast::nodes::*;
use crate::error::RaccoonError;
use crate::runtime::{
    ArrayValue, CallStack, DecoratorRegistry, Environment, FutureValue, ModuleRegistry, NullValue,
    Registrar, RuntimeValue, StrValue, TypeRegistry,
};
use crate::tokens::{BinaryOperator, Position};
use async_recursion::async_recursion;

pub enum InterpreterResult {
    Value(RuntimeValue),
    Return(RuntimeValue),
    Break,
    Continue,
}

impl InterpreterResult {}

pub struct Interpreter {
    pub file: Option<String>,
    pub environment: Environment,
    pub type_registry: std::sync::Arc<TypeRegistry>,
    pub stdlib_loader: std::sync::Arc<crate::runtime::StdLibLoader>,
    pub recursion_depth: usize,
    pub max_recursion_depth: usize,
    pub decorator_registry: DecoratorRegistry,
    pub registrar: std::sync::Arc<std::sync::Mutex<Registrar>>,
    pub module_registry: std::sync::Arc<ModuleRegistry>,
    pub call_stack: CallStack,
    pub use_ir: bool,
}

impl Interpreter {
    pub fn new(file: Option<String>) -> Self {
        let mut env = Environment::new(file.clone());
        let type_registry = std::sync::Arc::new(TypeRegistry::new());
        let registrar = std::sync::Arc::new(std::sync::Mutex::new(Registrar::new()));

        // Register core primitives immediately (needed for std:runtime module)
        {
            let mut reg = registrar.lock().unwrap();
            crate::runtime::natives::register_core_primitives(&mut reg);
        }

        let mut module_registry = ModuleRegistry::new();

        // Register all native modules (lazy-loaded, not yet initialized)
        module_registry.register("math", |registrar| {
            crate::runtime::natives::register_math_module(registrar)
        });
        module_registry.register("string", |registrar| {
            crate::runtime::natives::register_string_module(registrar)
        });
        module_registry.register("array", |registrar| {
            crate::runtime::natives::register_array_module(registrar)
        });
        module_registry.register("json", |registrar| {
            crate::runtime::natives::register_json_module(registrar)
        });
        module_registry.register("time", |registrar| {
            crate::runtime::natives::register_time_module(registrar)
        });
        module_registry.register("random", |registrar| {
            crate::runtime::natives::register_random_module(registrar)
        });
        module_registry.register("io", |registrar| {
            crate::runtime::natives::register_io_module(registrar)
        });
        module_registry.register("http", |registrar| {
            crate::runtime::natives::register_http_module(registrar)
        });

        // Initialize builtins only (print, println, input, len)
        Self::register_builtins(&mut env, registrar.clone());

        // Register stdlib wrapper functions
        crate::runtime::register_stdlib_wrappers(&mut env, registrar.clone());

        let stdlib_loader = std::sync::Arc::new(crate::runtime::StdLibLoader::with_default_path());
        let decorator_registry = DecoratorRegistry::new();

        Self {
            environment: env,
            file,
            type_registry,
            stdlib_loader,
            recursion_depth: 0,
            max_recursion_depth: 200,
            decorator_registry,
            registrar,
            module_registry: std::sync::Arc::new(module_registry),
            call_stack: CallStack::new(),
            use_ir: false,
        }
    }

    fn register_builtins(
        env: &mut Environment,
        _registrar: std::sync::Arc<std::sync::Mutex<Registrar>>,
    ) {
        use crate::runtime::setup_builtins;

        // Call the main setup_builtins to register all builtins:
        // - Global functions: print, println, eprint, input, len
        // - Primitive types: int, str, float, bool with static methods/properties
        // - Built-in objects: Future, Object, Type
        setup_builtins(env);
    }

    /// Load std:core module and inject its exports into the global scope
    async fn load_std_core_if_needed(&mut self) -> Result<(), RaccoonError> {
        // Check if std:core is already loaded by looking for 'print' in the environment
        if self.environment.exists("print") {
            return Ok(());
        }

        // Load the std:core module
        let core_module = self.stdlib_loader.load_module("std:core").await?;

        // Inject all exports from std:core into the global scope
        if let RuntimeValue::Object(obj) = core_module {
            for (name, value) in obj.properties.iter() {
                let _ = self.environment.declare(name.clone(), value.clone());
            }
        }

        Ok(())
    }

    /// Enable IR mode for optimized execution
    pub fn enable_ir_mode(&mut self) {
        self.use_ir = true;
    }

    /// Disable IR mode (use direct AST interpretation)
    pub fn disable_ir_mode(&mut self) {
        self.use_ir = false;
    }

    /// Interpret a program using IR compilation and VM execution
    #[async_recursion(?Send)]
    pub async fn interpret_with_ir(&mut self, program: &Program) -> Result<RuntimeValue, RaccoonError> {
        // Load std:core module on first interpret call
        if self.file.is_none() || self.file.as_ref().map_or(false, |f| f == "<root>") {
            self.load_std_core_if_needed().await?;
        }

        // Compile AST to IR
        let compiler = crate::ir::IRCompiler::new();
        let ir_program = compiler.compile(program)?;

        // Optimize IR
        let optimizer = crate::ir::IROptimizer::new(ir_program);
        let optimized_program = optimizer.optimize();

        // Execute IR with VM
        let mut vm = crate::ir::VM::new(self.environment.clone());
        let result = vm.execute(optimized_program).await?;

        Ok(result)
    }

    #[async_recursion(?Send)]
    pub async fn interpret(&mut self, program: &Program) -> Result<RuntimeValue, RaccoonError> {
        // If IR mode is enabled, use IR interpretation
        if self.use_ir {
            return self.interpret_with_ir(program).await;
        }
        // Load std:core module on first interpret call (only once per interpreter instance)
        if self.file.is_none() || self.file.as_ref().map_or(false, |f| f == "<root>") {
            self.load_std_core_if_needed().await?;
        }

        let mut last_value = RuntimeValue::Null(NullValue::new());

        for stmt in &program.stmts {
            match self.execute_stmt_internal(stmt).await {
                Ok(InterpreterResult::Value(v)) => last_value = v,
                Ok(_) => {
                    return Err(RaccoonError::new(
                        "Unexpected control flow statement",
                        stmt.position(),
                        self.file.clone(),
                    ));
                }
                Err(e) => return Err(e),
            }
        }

        Ok(last_value)
    }

    #[async_recursion(?Send)]
    pub async fn execute_stmt_internal(
        &mut self,
        stmt: &Stmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        match stmt {
            Stmt::Program(program) => self.interpret(program).await.map(InterpreterResult::Value),
            Stmt::VarDecl(decl) => declarations::Declarations::execute_var_decl(self, decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::FnDecl(decl) => declarations::Declarations::execute_fn_decl(self, decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::Block(block) => {
                control_flow::ControlFlow::execute_block_internal(self, block).await
            }
            Stmt::IfStmt(if_stmt) => {
                control_flow::ControlFlow::execute_if_stmt_internal(self, if_stmt).await
            }
            Stmt::WhileStmt(while_stmt) => {
                control_flow::ControlFlow::execute_while_stmt(self, while_stmt).await
            }
            Stmt::DoWhileStmt(do_while_stmt) => {
                control_flow::ControlFlow::execute_do_while_stmt(self, do_while_stmt).await
            }
            Stmt::ForStmt(for_stmt) => {
                control_flow::ControlFlow::execute_for_stmt(self, for_stmt).await
            }
            Stmt::ForInStmt(for_in) => {
                control_flow::ControlFlow::execute_for_in_stmt(self, for_in).await
            }
            Stmt::ForOfStmt(for_of) => {
                control_flow::ControlFlow::execute_for_of_stmt(self, for_of).await
            }
            Stmt::SwitchStmt(switch_stmt) => {
                control_flow::ControlFlow::execute_switch_stmt(self, switch_stmt).await
            }
            Stmt::ReturnStmt(ret) => {
                control_flow::ControlFlow::execute_return_stmt(self, ret).await
            }
            Stmt::BreakStmt(_) => Ok(InterpreterResult::Break),
            Stmt::ContinueStmt(_) => Ok(InterpreterResult::Continue),
            Stmt::ExprStmt(expr_stmt) => self
                .evaluate_expr(&expr_stmt.expression)
                .await
                .map(InterpreterResult::Value),
            Stmt::ClassDecl(decl) => declarations::Declarations::execute_class_decl(self, decl)
                .await
                .map(InterpreterResult::Value),
            Stmt::InterfaceDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::EnumDecl(enum_decl) => {
                declarations::Declarations::execute_enum_decl(self, enum_decl).await
            }
            Stmt::TypeAliasDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::ImportDecl(import_decl) => {
                module_loader::ModuleLoader::execute_import_decl(self, import_decl).await
            }
            Stmt::ExportDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TryStmt(try_stmt) => {
                control_flow::ControlFlow::execute_try_stmt(self, try_stmt).await
            }
            Stmt::ThrowStmt(throw) => {
                declarations::Declarations::execute_throw_stmt(self, throw).await
            }
        }
    }

    #[async_recursion(?Send)]
    pub async fn evaluate_expr(&mut self, expr: &Expr) -> Result<RuntimeValue, RaccoonError> {
        expressions::Expressions::evaluate_expr(self, expr).await
    }

    pub async fn apply_binary_op(
        &self,
        left: RuntimeValue,
        right: RuntimeValue,
        operator: BinaryOperator,
        position: Position,
    ) -> Result<RuntimeValue, RaccoonError> {
        operators::apply_binary_op(
            left,
            right,
            operator,
            position,
            &self.file,
            &self.call_stack,
        )
        .await
    }

    pub fn is_truthy(&self, value: &RuntimeValue) -> bool {
        operators::is_truthy(value)
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

    pub fn get_decorator_registry(&self) -> &DecoratorRegistry {
        &self.decorator_registry
    }

    pub fn is_in_stdlib(&self) -> bool {
        if let Some(file) = &self.file {
            file.contains("stdlib") || file.ends_with(".rcc")
        } else {
            false
        }
    }

    pub fn try_load_native_function(&mut self, name: &str) -> Option<RuntimeValue> {
        // Check if this is a module.function pattern
        if let Some(dot_pos) = name.find('.') {
            let module_name = &name[..dot_pos];

            // Try to load the module if not already loaded
            if let Some(loader) = self.module_registry.get_loader(module_name) {
                // Load the module
                let mut registrar = self.registrar.lock().unwrap();
                if !registrar.functions.contains_key(name) {
                    loader(&mut registrar);
                }
                drop(registrar);

                // Try to get the function from registrar
                let registrar = self.registrar.lock().unwrap();
                if let Some(_handler) = registrar.get_function(name) {
                    // Handler is available in registrar
                    // Return None for now - will handle this in expressions module
                    return None;
                }
            }
        }
        None
    }

    /// Get a builtin type by name (e.g., "Future", "Promise")
    /// Returns a PrimitiveTypeObject with static methods, not a global object instance
    pub fn get_builtin_type(&self, name: &str) -> Option<RuntimeValue> {
        use crate::ast::types::PrimitiveType;
        use crate::runtime::{NativeFunctionValue, PrimitiveTypeObject};
        use std::collections::HashMap;

        match name {
            "Future" => {
                let mut static_methods = HashMap::new();

                // Future.resolve(value)
                let resolve_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        let value = if args.is_empty() {
                            RuntimeValue::Null(NullValue::new())
                        } else {
                            args[0].clone()
                        };
                        let value_type = value.get_type();
                        RuntimeValue::Future(FutureValue::new_resolved(value, value_type))
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("resolve".to_string(), Box::new(resolve_fn));

                // Future.reject(reason)
                let reject_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        let reason_str = if args.is_empty() {
                            "Unknown error".to_string()
                        } else {
                            args[0].to_string()
                        };
                        RuntimeValue::Future(FutureValue::new_rejected(
                            reason_str,
                            PrimitiveType::any(),
                        ))
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("reject".to_string(), Box::new(reject_fn));

                // Future.all(futures) - combines multiple futures and waits for all
                let all_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        if args.is_empty() {
                            let list =
                                RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any()));
                            return RuntimeValue::Future(FutureValue::new_resolved(
                                list,
                                PrimitiveType::any(),
                            ));
                        }

                        match &args[0] {
                            RuntimeValue::Array(list) => {
                                let futures = list.elements.clone();
                                let result_future = FutureValue::new(PrimitiveType::any());
                                let result_clone = result_future.clone();

                                // Spawn async task to wait for all futures
                                tokio::task::spawn_local(async move {
                                    let mut results = Vec::new();

                                    // Wait for all futures sequentially (can be optimized with tokio::join!)
                                    for element in &futures {
                                        if let RuntimeValue::Future(fut) = element {
                                            match fut.wait_for_completion().await {
                                                Ok(val) => results.push(val),
                                                Err(err) => {
                                                    result_clone.reject(err);
                                                    return;
                                                }
                                            }
                                        } else {
                                            results.push(element.clone());
                                        }
                                    }

                                    let result_list = RuntimeValue::Array(ArrayValue::new(
                                        results,
                                        PrimitiveType::any(),
                                    ));
                                    result_clone.resolve(result_list);
                                });

                                RuntimeValue::Future(result_future)
                            }
                            _ => RuntimeValue::Future(FutureValue::new_rejected(
                                "Future.all requires an array".to_string(),
                                PrimitiveType::any(),
                            )),
                        }
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("all".to_string(), Box::new(all_fn));

                // Future.race(futures) - returns first resolved/rejected
                let race_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        if args.is_empty() {
                            return RuntimeValue::Future(FutureValue::new_resolved(
                                RuntimeValue::Null(NullValue::new()),
                                PrimitiveType::any(),
                            ));
                        }

                        match &args[0] {
                            RuntimeValue::Array(list) => {
                                if list.elements.is_empty() {
                                    return RuntimeValue::Future(FutureValue::new_resolved(
                                        RuntimeValue::Null(NullValue::new()),
                                        PrimitiveType::any(),
                                    ));
                                }

                                let futures = list.elements.clone();
                                let result_future = FutureValue::new(PrimitiveType::any());
                                let result_clone = result_future.clone();

                                // Spawn async task to race all futures
                                tokio::task::spawn_local(async move {
                                    // Create a vector of async tasks to race
                                    let mut tasks = Vec::new();

                                    for element in &futures {
                                        if let RuntimeValue::Future(fut) = element {
                                            let fut_clone = fut.clone();
                                            tasks.push(tokio::task::spawn_local(async move {
                                                fut_clone.wait_for_completion().await
                                            }));
                                        }
                                    }

                                    // Wait for first to complete
                                    if !tasks.is_empty() {
                                        // Use select to wait for first completion
                                        for task in tasks {
                                            if let Ok(result) = task.await {
                                                match result {
                                                    Ok(val) => {
                                                        result_clone.resolve(val);
                                                        return;
                                                    }
                                                    Err(err) => {
                                                        result_clone.reject(err);
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    result_clone.resolve(RuntimeValue::Null(NullValue::new()));
                                });

                                RuntimeValue::Future(result_future)
                            }
                            _ => RuntimeValue::Future(FutureValue::new_rejected(
                                "Future.race requires an array".to_string(),
                                PrimitiveType::any(),
                            )),
                        }
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("race".to_string(), Box::new(race_fn));

                // Future.allSettled(futures) - waits for all, returns status objects
                let all_settled_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        if args.is_empty() {
                            let list =
                                RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any()));
                            return RuntimeValue::Future(FutureValue::new_resolved(
                                list,
                                PrimitiveType::any(),
                            ));
                        }

                        match &args[0] {
                            RuntimeValue::Array(list) => {
                                let mut results = Vec::new();

                                for element in &list.elements {
                                    if let RuntimeValue::Future(fut) = element {
                                        let state = fut.state.read().unwrap();
                                        let result_obj = match &*state {
                                            crate::runtime::values::FutureState::Resolved(val) => {
                                                let mut obj_props =
                                                    std::collections::HashMap::new();
                                                obj_props.insert(
                                                    "status".to_string(),
                                                    RuntimeValue::Str(StrValue::new(
                                                        "fulfilled".to_string(),
                                                    )),
                                                );
                                                obj_props
                                                    .insert("value".to_string(), (**val).clone());
                                                RuntimeValue::Object(
                                                    crate::runtime::ObjectValue::new(
                                                        obj_props,
                                                        PrimitiveType::any(),
                                                    ),
                                                )
                                            }
                                            crate::runtime::values::FutureState::Rejected(err) => {
                                                let mut obj_props =
                                                    std::collections::HashMap::new();
                                                obj_props.insert(
                                                    "status".to_string(),
                                                    RuntimeValue::Str(StrValue::new(
                                                        "rejected".to_string(),
                                                    )),
                                                );
                                                obj_props.insert(
                                                    "reason".to_string(),
                                                    RuntimeValue::Str(StrValue::new(err.clone())),
                                                );
                                                RuntimeValue::Object(
                                                    crate::runtime::ObjectValue::new(
                                                        obj_props,
                                                        PrimitiveType::any(),
                                                    ),
                                                )
                                            }
                                            crate::runtime::values::FutureState::Pending => {
                                                let mut obj_props =
                                                    std::collections::HashMap::new();
                                                obj_props.insert(
                                                    "status".to_string(),
                                                    RuntimeValue::Str(StrValue::new(
                                                        "pending".to_string(),
                                                    )),
                                                );
                                                RuntimeValue::Object(
                                                    crate::runtime::ObjectValue::new(
                                                        obj_props,
                                                        PrimitiveType::any(),
                                                    ),
                                                )
                                            }
                                        };
                                        results.push(result_obj);
                                    } else {
                                        let mut obj_props = std::collections::HashMap::new();
                                        obj_props.insert(
                                            "status".to_string(),
                                            RuntimeValue::Str(StrValue::new(
                                                "fulfilled".to_string(),
                                            )),
                                        );
                                        obj_props.insert("value".to_string(), element.clone());
                                        results.push(RuntimeValue::Object(
                                            crate::runtime::ObjectValue::new(
                                                obj_props,
                                                PrimitiveType::any(),
                                            ),
                                        ));
                                    }
                                }

                                let result_list = RuntimeValue::Array(ArrayValue::new(
                                    results,
                                    PrimitiveType::any(),
                                ));
                                RuntimeValue::Future(FutureValue::new_resolved(
                                    result_list,
                                    PrimitiveType::any(),
                                ))
                            }
                            _ => RuntimeValue::Future(FutureValue::new_rejected(
                                "Future.allSettled requires an array".to_string(),
                                PrimitiveType::any(),
                            )),
                        }
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("allSettled".to_string(), Box::new(all_settled_fn));

                // Future.any(futures) - returns first resolved, rejects if all reject
                let any_fn = NativeFunctionValue::new(
                    |args: Vec<RuntimeValue>| {
                        if args.is_empty() {
                            return RuntimeValue::Future(FutureValue::new_rejected(
                                "All futures rejected".to_string(),
                                PrimitiveType::any(),
                            ));
                        }

                        match &args[0] {
                            RuntimeValue::Array(list) => {
                                // Find first resolved future
                                for element in &list.elements {
                                    if let RuntimeValue::Future(fut) = element {
                                        let state = fut.state.read().unwrap();
                                        if let crate::runtime::values::FutureState::Resolved(val) =
                                            &*state
                                        {
                                            return RuntimeValue::Future(
                                                FutureValue::new_resolved(
                                                    (**val).clone(),
                                                    PrimitiveType::any(),
                                                ),
                                            );
                                        }
                                    } else {
                                        // Non-future values are treated as resolved
                                        return RuntimeValue::Future(FutureValue::new_resolved(
                                            element.clone(),
                                            PrimitiveType::any(),
                                        ));
                                    }
                                }

                                // All are rejected, return the first rejection
                                for element in &list.elements {
                                    if let RuntimeValue::Future(fut) = element {
                                        let state = fut.state.read().unwrap();
                                        if let crate::runtime::values::FutureState::Rejected(err) =
                                            &*state
                                        {
                                            return RuntimeValue::Future(
                                                FutureValue::new_rejected(
                                                    err.clone(),
                                                    PrimitiveType::any(),
                                                ),
                                            );
                                        }
                                    }
                                }

                                RuntimeValue::Future(FutureValue::new_rejected(
                                    "All futures rejected".to_string(),
                                    PrimitiveType::any(),
                                ))
                            }
                            _ => RuntimeValue::Future(FutureValue::new_rejected(
                                "Future.any requires an array".to_string(),
                                PrimitiveType::any(),
                            )),
                        }
                    },
                    crate::fn_type!(variadic, PrimitiveType::any()),
                );
                static_methods.insert("any".to_string(), Box::new(any_fn));

                let static_properties = HashMap::new();

                // Create a Future type: Future<any>
                use crate::ast::types::{FutureType, Type};
                let any_type = PrimitiveType::any();
                let future_type = Type::Future(Box::new(FutureType {
                    inner_type: any_type,
                }));

                Some(RuntimeValue::PrimitiveTypeObject(PrimitiveTypeObject::new(
                    "Future".to_string(),
                    static_methods,
                    static_properties,
                    future_type,
                )))
            }
            _ => None,
        }
    }
}
