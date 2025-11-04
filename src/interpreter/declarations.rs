use crate::ast::nodes::*;
use crate::ast::types::{PrimitiveType, Type};
use crate::error::RaccoonError;
use crate::runtime::{
    DecoratorTarget, FunctionValue,
    NullValue, RuntimeValue,
};
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::{Interpreter, InterpreterResult};
use crate::runtime::values::{EnumObject, EnumValueData};

pub struct Declarations;

impl Declarations {
    #[async_recursion(?Send)]
    pub async fn execute_var_decl(
        interpreter: &mut Interpreter,
        decl: &VarDecl,
    ) -> Result<RuntimeValue, RaccoonError> {
        let value = if let Some(init) = &decl.initializer {
            interpreter.evaluate_expr(init).await?
        } else {
            RuntimeValue::Null(NullValue::new())
        };

        match &decl.pattern {
            VarPattern::Identifier(name) => {
                interpreter.environment.declare(name.clone(), value.clone())?;
            }
            VarPattern::Destructuring(pattern) => {
                super::helpers::Helpers::destructure_pattern(
                    interpreter,
                    pattern,
                    &value,
                    decl.position,
                )
                .await?;
            }
        }

        Ok(value)
    }

    #[async_recursion(?Send)]
    pub async fn execute_fn_decl(
        interpreter: &mut Interpreter,
        decl: &FnDecl,
    ) -> Result<RuntimeValue, RaccoonError> {
        let target = if decl.is_async {
            DecoratorTarget::AsyncFunction
        } else {
            DecoratorTarget::Function
        };

        let decorators = interpreter.decorator_registry.validate(
            &decl.decorators,
            target,
            interpreter.is_in_stdlib(),
            interpreter.file.as_deref(),
        )?;

        let fn_type = Type::Function(Box::new(crate::ast::types::FunctionType {
            params: decl
                .parameters
                .iter()
                .map(|p| p.param_type.clone())
                .collect(),
            return_type: decl
                .return_type
                .clone()
                .unwrap_or_else(|| PrimitiveType::unknown()),
            is_variadic: decl.parameters.iter().any(|p| p.is_rest),
        }));

        let function = RuntimeValue::Function(
            FunctionValue::new(
                decl.parameters.clone(),
                decl.body.clone(),
                decl.is_async,
                fn_type.clone(),
            )
            .with_decorators(decl.decorators.clone()),
        );

        interpreter.environment.declare(decl.name.clone(), function)?;

        for decorator_info in &decorators {
            match decorator_info.spec.name.as_str() {
                "@deprecated" => {
                    let msg = decorator_info
                        .arg_as_string(0)
                        .unwrap_or_else(|| "This function is deprecated".to_string());
                    eprintln!(
                        "⚠️  Warning: Function '{}' is deprecated. {}",
                        decl.name, msg
                    );
                }
                _ => {
                    // Other decorators are registered but not actively implemented yet
                }
            }
        }

        Ok(RuntimeValue::Null(NullValue::new()))
    }

    #[async_recursion(?Send)]
    pub async fn execute_class_decl(
        interpreter: &mut Interpreter,
        decl: &ClassDecl,
    ) -> Result<RuntimeValue, RaccoonError> {
        let class_type = PrimitiveType::any();

        let mut static_methods = HashMap::new();
        let mut static_properties = HashMap::new();

        for method in &decl.methods {
            if method.is_static {
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

                let function = Box::new(FunctionValue::new(
                    method.parameters.clone(),
                    method.body.clone(),
                    method.is_async,
                    fn_type,
                ));

                static_methods.insert(method.name.clone(), function);
            }
        }

        for property in &decl.properties {
            if let Some(initializer) = &property.initializer {
                let value = interpreter.evaluate_expr(initializer).await?;
                static_properties.insert(property.name.clone(), value);
            }
        }

        let class_value = RuntimeValue::Class(crate::runtime::ClassValue::with_properties(
            decl.name.clone(),
            static_methods,
            static_properties,
            class_type,
            decl.clone(),
        ));

        interpreter.environment.declare(decl.name.clone(), class_value)?;

        Ok(RuntimeValue::Null(NullValue::new()))
    }

    #[async_recursion(?Send)]
    pub async fn execute_enum_decl(
        interpreter: &mut Interpreter,
        decl: &EnumDecl,
    ) -> Result<InterpreterResult, RaccoonError> {
        let mut members = HashMap::new();
        let mut current_value: i64 = 0;

        for member in &decl.members {
            if let Some(init_val) = &member.value {
                if let Expr::IntLiteral(int_lit) = init_val {
                    current_value = int_lit.value;
                    members.insert(member.name.clone(), EnumValueData::Int(current_value));
                } else if let Expr::StrLiteral(str_lit) = init_val {
                    members.insert(
                        member.name.clone(),
                        EnumValueData::Str(str_lit.value.clone()),
                    );
                }
                current_value += 1;
            } else {
                members.insert(member.name.clone(), EnumValueData::Int(current_value));
                current_value += 1;
            }
        }

        let enum_type = Type::Enum(Box::new(crate::ast::types::EnumType {
            name: decl.name.clone(),
            members: HashMap::new(),
        }));

        let enum_obj =
            RuntimeValue::EnumObject(EnumObject::new(decl.name.clone(), members, enum_type));

        interpreter.environment.declare(decl.name.clone(), enum_obj)?;

        Ok(InterpreterResult::Value(RuntimeValue::Null(
            NullValue::new(),
        )))
    }

    pub async fn execute_throw_stmt(
        interpreter: &mut Interpreter,
        throw: &ThrowStmt,
    ) -> Result<InterpreterResult, RaccoonError> {
        let value = interpreter.evaluate_expr(&throw.value).await?;
        Err(RaccoonError::new(
            value.to_string(),
            throw.position,
            interpreter.file.clone(),
        ))
    }
}
