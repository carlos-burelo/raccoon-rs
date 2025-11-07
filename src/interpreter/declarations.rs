use crate::ast::nodes::*;
use crate::ast::types::{PrimitiveType, Type};
use crate::error::RaccoonError;
use crate::runtime::{
    DecoratorTarget, FunctionValue,
    NullValue, RuntimeValue,
    TypeKind, TypeObjectBuilder,
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
        let class_type = Type::Class(Box::new(crate::ast::types::ClassType {
            name: decl.name.clone(),
            superclass: decl.superclass.as_ref().map(|_s| {
                Box::new(crate::ast::types::ClassType {
                    name: "Object".to_string(),
                    superclass: None,
                    properties: HashMap::new(),
                    methods: HashMap::new(),
                    constructor: None,
                    type_parameters: vec![],
                })
            }),
            properties: HashMap::new(),
            methods: HashMap::new(),
            constructor: None,
            type_parameters: decl.type_parameters.clone(),
        }));

        // Collect static methods and properties for ClassValue (legacy)
        let mut class_static_methods = HashMap::new();
        let mut class_static_properties = HashMap::new();

        // Collect static methods and properties for TypeObject (new)
        let mut type_static_methods = HashMap::new();
        let mut type_static_properties = HashMap::new();

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

                let function_value = RuntimeValue::Function(FunctionValue::new(
                    method.parameters.clone(),
                    method.body.clone(),
                    method.is_async,
                    fn_type.clone(),
                ));

                // For ClassValue (legacy)
                class_static_methods.insert(
                    method.name.clone(),
                    Box::new(FunctionValue::new(
                        method.parameters.clone(),
                        method.body.clone(),
                        method.is_async,
                        fn_type,
                    ))
                );

                // For TypeObject (new)
                type_static_methods.insert(method.name.clone(), function_value);
            }
        }

        for property in &decl.properties {
            if let Some(initializer) = &property.initializer {
                let value = interpreter.evaluate_expr(initializer).await?;
                class_static_properties.insert(property.name.clone(), value.clone());
                type_static_properties.insert(property.name.clone(), value);
            }
        }

        // Create ClassValue for backward compatibility
        let class_value = RuntimeValue::Class(crate::runtime::ClassValue::with_properties(
            decl.name.clone(),
            class_static_methods,
            class_static_properties,
            class_type.clone(),
            decl.clone(),
        ));

        // Create TypeObject representing the class as a type
        let type_object = TypeObjectBuilder::new(
            class_type,
            TypeKind::Class {
                name: decl.name.clone(),
                superclass: decl.superclass.clone(),
            },
        )
        .static_methods(type_static_methods)
        .static_properties(type_static_properties)
        .constructor(class_value.clone())  // The class value acts as constructor
        .documentation(extract_doc_from_decorators(&decl.decorators))
        .decorators(decl.decorators.iter().map(|d| d.name.clone()).collect())
        .build();

        // Declare the TypeObject in the environment
        interpreter.environment.declare(decl.name.clone(), RuntimeValue::Type(type_object))?;

        Ok(RuntimeValue::Null(NullValue::new()))
    }

    #[async_recursion(?Send)]
    pub async fn execute_enum_decl(
        interpreter: &mut Interpreter,
        decl: &EnumDecl,
    ) -> Result<InterpreterResult, RaccoonError> {
        let mut members = HashMap::new();
        let mut current_value: i64 = 0;
        let mut variant_names = Vec::new();

        for member in &decl.members {
            variant_names.push(member.name.clone());

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

        // Create EnumObject for backward compatibility (legacy)
        let enum_obj = EnumObject::new(decl.name.clone(), members.clone(), enum_type.clone());

        // Create static properties for each enum member
        let mut static_properties = HashMap::new();
        for (member_name, member_data) in &members {
            let member_value = match member_data {
                EnumValueData::Int(i) => RuntimeValue::Int(crate::runtime::IntValue::new(*i)),
                EnumValueData::Str(s) => RuntimeValue::Str(crate::runtime::StrValue::new(s.clone())),
            };
            static_properties.insert(member_name.clone(), member_value);
        }

        // Create TypeObject representing the enum as a type
        let type_object = TypeObjectBuilder::new(
            enum_type,
            TypeKind::Enum {
                name: decl.name.clone(),
                variants: variant_names,
            },
        )
        .static_properties(static_properties)
        .constructor(RuntimeValue::EnumObject(enum_obj))  // EnumObject acts as constructor/namespace
        .documentation(format!("Enum {}", decl.name))
        .build();

        // Declare the TypeObject in the environment
        interpreter.environment.declare(decl.name.clone(), RuntimeValue::Type(type_object))?;

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

/// Extract documentation from decorators (e.g., @doc("..."))
fn extract_doc_from_decorators(decorators: &[DecoratorDecl]) -> String {
    for decorator in decorators {
        if decorator.name == "doc" {
            if let Some(first_arg) = decorator.args.first() {
                if let Expr::StrLiteral(str_lit) = first_arg {
                    return str_lit.value.clone();
                }
            }
        }
    }
    String::new()
}
