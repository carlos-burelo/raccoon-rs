mod operators;
pub mod declarations;
pub mod control_flow;
pub mod expressions;
pub mod helpers;
pub mod module_loader;
pub mod builtins;

use crate::ast::nodes::*;
use crate::error::RaccoonError;
use crate::runtime::{
    DecoratorRegistry, Environment, NullValue, RuntimeValue,
    TypeRegistry, setup_builtins,
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
    pub type_registry: TypeRegistry,
    pub stdlib_loader: std::sync::Arc<crate::runtime::StdLibLoader>,
    pub recursion_depth: usize,
    pub max_recursion_depth: usize,
    pub decorator_registry: DecoratorRegistry,
}

impl Interpreter {
    pub fn new(file: Option<String>) -> Self {
        let mut env = Environment::new(file.clone());
        let type_registry = TypeRegistry::new();
        setup_builtins(&mut env);

        let stdlib_loader = std::sync::Arc::new(crate::runtime::StdLibLoader::with_default_path());
        let decorator_registry = DecoratorRegistry::new();

        Self {
            environment: env,
            file,
            type_registry,
            stdlib_loader,
            recursion_depth: 0,
            max_recursion_depth: 500,
            decorator_registry,
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
            Stmt::Block(block) => control_flow::ControlFlow::execute_block_internal(self, block).await,
            Stmt::IfStmt(if_stmt) => control_flow::ControlFlow::execute_if_stmt_internal(self, if_stmt).await,
            Stmt::WhileStmt(while_stmt) => control_flow::ControlFlow::execute_while_stmt(self, while_stmt).await,
            Stmt::DoWhileStmt(do_while_stmt) => control_flow::ControlFlow::execute_do_while_stmt(self, do_while_stmt).await,
            Stmt::ForStmt(for_stmt) => control_flow::ControlFlow::execute_for_stmt(self, for_stmt).await,
            Stmt::ForInStmt(for_in) => control_flow::ControlFlow::execute_for_in_stmt(self, for_in).await,
            Stmt::ForOfStmt(for_of) => control_flow::ControlFlow::execute_for_of_stmt(self, for_of).await,
            Stmt::SwitchStmt(switch_stmt) => control_flow::ControlFlow::execute_switch_stmt(self, switch_stmt).await,
            Stmt::ReturnStmt(ret) => control_flow::ControlFlow::execute_return_stmt(self, ret).await,
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
            Stmt::EnumDecl(enum_decl) => declarations::Declarations::execute_enum_decl(self, enum_decl).await,
            Stmt::TypeAliasDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::ImportDecl(import_decl) => module_loader::ModuleLoader::execute_import_decl(self, import_decl).await,
            Stmt::ExportDecl(_) => Ok(InterpreterResult::Value(RuntimeValue::Null(
                NullValue::new(),
            ))),
            Stmt::TryStmt(try_stmt) => control_flow::ControlFlow::execute_try_stmt(self, try_stmt).await,
            Stmt::ThrowStmt(throw) => declarations::Declarations::execute_throw_stmt(self, throw).await,
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
        operators::apply_binary_op(left, right, operator, position, &self.file).await
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
}
