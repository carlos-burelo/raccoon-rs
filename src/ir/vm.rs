use crate::error::RaccoonError;
use crate::runtime::{Environment, RuntimeValue, TypeRegistry};
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::instruction::{IRProgram, Instruction, MatchPattern, Register, TemplatePart};

pub struct VM {
    registers: HashMap<String, RuntimeValue>,
    environment: Environment,
    pc: usize,
    program: Option<IRProgram>,
    type_registry: std::sync::Arc<TypeRegistry>,
}

impl VM {
    pub fn new(environment: Environment, type_registry: std::sync::Arc<TypeRegistry>) -> Self {
        Self {
            registers: HashMap::new(),
            environment,
            pc: 0,
            program: None,
            type_registry,
        }
    }

    #[async_recursion(?Send)]
    pub async fn execute(&mut self, program: IRProgram) -> Result<RuntimeValue, RaccoonError> {
        self.program = Some(program);
        self.pc = 0;

        let mut last_value = RuntimeValue::Null(crate::runtime::NullValue::new());

        while self.pc < self.get_program_len() {
            let instruction = self.get_instruction(self.pc).clone();
            self.pc += 1;

            match self.execute_instruction(&instruction).await? {
                ExecutionResult::Continue => {}
                ExecutionResult::Return(value) => {
                    last_value = value;
                    break;
                }
                ExecutionResult::Jump(label) => {
                    if let Some(pos) = self.resolve_label(&label) {
                        self.pc = pos;
                    } else {
                        return Err(RaccoonError::new(
                            &format!("Unknown label: {}", label),
                            (0, 0),
                            None::<String>,
                        ));
                    }
                }
            }
        }

        Ok(last_value)
    }

    #[async_recursion(?Send)]
    async fn execute_instruction(
        &mut self,
        instruction: &Instruction,
    ) -> Result<ExecutionResult, RaccoonError> {
        match instruction {
            Instruction::LoadConst { dest, value } => {
                self.set_register(dest, value.clone());
                Ok(ExecutionResult::Continue)
            }

            Instruction::Move { dest, src } => {
                let value = self.get_register(src)?;
                self.set_register(dest, value);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Declare { name, is_const: _ } => {
                self.environment.declare(
                    name.clone(),
                    RuntimeValue::Null(crate::runtime::NullValue::new()),
                )?;
                Ok(ExecutionResult::Continue)
            }

            Instruction::Store { name, src } => {
                let value = self.get_register(src)?;
                self.environment.assign(name, value, (0, 0))?;
                Ok(ExecutionResult::Continue)
            }

            Instruction::Load { dest, name } => {
                let value = self.environment.get(name, (0, 0))?;
                self.set_register(dest, value);
                Ok(ExecutionResult::Continue)
            }

            Instruction::BinaryOp {
                dest,
                left,
                right,
                op,
            } => {
                let left_val = self.get_register(left)?;
                let right_val = self.get_register(right)?;

                let result = crate::interpreter::operators::apply_binary_op(
                    left_val,
                    right_val,
                    op.clone(),
                    (0, 0),
                    &None,
                    &crate::runtime::CallStack::new(),
                )
                .await?;

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::UnaryOp { dest, operand, op } => {
                let operand_val = self.get_register(operand)?;

                let result = match op {
                    crate::tokens::UnaryOperator::Not => {
                        let is_truthy = crate::interpreter::operators::is_truthy(&operand_val);
                        RuntimeValue::Bool(crate::runtime::BoolValue::new(!is_truthy))
                    }
                    crate::tokens::UnaryOperator::Negate => match operand_val {
                        RuntimeValue::Int(i) => {
                            RuntimeValue::Int(crate::runtime::IntValue::new(-i.value))
                        }
                        RuntimeValue::Float(f) => {
                            RuntimeValue::Float(crate::runtime::FloatValue::new(-f.value))
                        }
                        _ => {
                            return Err(RaccoonError::new(
                                "Cannot negate non-numeric value",
                                (0, 0),
                                None::<String>,
                            ))
                        }
                    },
                    crate::tokens::UnaryOperator::BitwiseNot => match operand_val {
                        RuntimeValue::Int(i) => {
                            RuntimeValue::Int(crate::runtime::IntValue::new(!i.value))
                        }
                        _ => {
                            return Err(RaccoonError::new(
                                "Cannot apply bitwise not to non-integer value",
                                (0, 0),
                                None::<String>,
                            ))
                        }
                    },
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Jump { label } => Ok(ExecutionResult::Jump(label.clone())),

            Instruction::JumpIfFalse { condition, label } => {
                let condition_val = self.get_register(condition)?;
                let is_truthy = crate::interpreter::operators::is_truthy(&condition_val);

                if !is_truthy {
                    Ok(ExecutionResult::Jump(label.clone()))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }

            Instruction::JumpIfTrue { condition, label } => {
                let condition_val = self.get_register(condition)?;
                let is_truthy = crate::interpreter::operators::is_truthy(&condition_val);

                if is_truthy {
                    Ok(ExecutionResult::Jump(label.clone()))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }

            Instruction::Label { .. } => Ok(ExecutionResult::Continue),

            Instruction::Call { dest, callee, args } => {
                let callee_val = self.get_register(callee)?;

                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_register(arg)?);
                }

                let result = self.call_function(callee_val, arg_values).await?;
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Return { value } => {
                let return_value = if let Some(val_reg) = value {
                    self.get_register(val_reg)?
                } else {
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                };
                Ok(ExecutionResult::Return(return_value))
            }

            Instruction::CreateFunction {
                dest,
                name,
                params,
                body,
                labels,
                is_async,
            } => {
                let ir_func = crate::ir::IRFunctionValue::new(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    labels.clone(),
                    *is_async,
                );
                let function = RuntimeValue::Dynamic(Box::new(ir_func));
                self.set_register(dest, function);
                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateArray { dest, elements } => {
                let mut element_values = Vec::new();
                for elem in elements {
                    element_values.push(self.get_register(elem)?);
                }

                let array = RuntimeValue::Array(crate::runtime::ArrayValue::new(
                    element_values,
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, array);
                Ok(ExecutionResult::Continue)
            }

            Instruction::LoadIndex { dest, array, index } => {
                let array_val = self.get_register(array)?;
                let index_val = self.get_register(index)?;

                let result = match array_val {
                    RuntimeValue::Array(arr) => {
                        if let RuntimeValue::Int(i) = index_val {
                            let idx = i.value as usize;
                            if idx < arr.elements.len() {
                                arr.elements[idx].clone()
                            } else {
                                RuntimeValue::Null(crate::runtime::NullValue::new())
                            }
                        } else {
                            return Err(RaccoonError::new(
                                "Array index must be an integer",
                                (0, 0),
                                None::<String>,
                            ));
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot index non-array value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::StoreIndex {
                array,
                index,
                value,
            } => {
                let array_val = self.get_register(array)?;
                let index_val = self.get_register(index)?;
                let value_val = self.get_register(value)?;

                match array_val {
                    RuntimeValue::Array(mut arr) => {
                        if let RuntimeValue::Int(i) = index_val {
                            let idx = i.value as usize;
                            if idx < arr.elements.len() {
                                arr.elements[idx] = value_val;
                                self.set_register(array, RuntimeValue::Array(arr));
                            } else {
                                return Err(RaccoonError::new(
                                    "Array index out of bounds",
                                    (0, 0),
                                    None::<String>,
                                ));
                            }
                        } else {
                            return Err(RaccoonError::new(
                                "Array index must be an integer",
                                (0, 0),
                                None::<String>,
                            ));
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot index non-array value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateObject { dest, properties } => {
                let mut prop_map = HashMap::new();
                for (key, val_reg) in properties {
                    let value = self.get_register(val_reg)?;
                    prop_map.insert(key.clone(), value);
                }

                let object = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    prop_map,
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, object);
                Ok(ExecutionResult::Continue)
            }

            Instruction::LoadProperty {
                dest,
                object,
                property,
            } => {
                let object_val = self.get_register(object)?;

                let result = match object_val {
                    RuntimeValue::Object(obj) => obj
                        .properties
                        .get(property)
                        .cloned()
                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new())),
                    RuntimeValue::Array(arr) => match property.as_str() {
                        "length" => RuntimeValue::Int(crate::runtime::IntValue::new(
                            arr.elements.len() as i64,
                        )),
                        "first" => {
                            if arr.elements.is_empty() {
                                RuntimeValue::Null(crate::runtime::NullValue::new())
                            } else {
                                arr.elements[0].clone()
                            }
                        }
                        _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
                    },
                    RuntimeValue::Str(s) => match property.as_str() {
                        "length" => {
                            RuntimeValue::Int(crate::runtime::IntValue::new(s.value.len() as i64))
                        }
                        "isEmpty" => {
                            RuntimeValue::Bool(crate::runtime::BoolValue::new(s.value.is_empty()))
                        }
                        _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
                    },
                    RuntimeValue::ClassInstance(instance) => {
                        if let Some(value) = instance.properties.read().unwrap().get(property) {
                            value.clone()
                        } else {
                            RuntimeValue::Null(crate::runtime::NullValue::new())
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot access property of non-object",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::StoreProperty {
                object,
                property,
                value,
            } => {
                let object_val = self.get_register(object)?;
                let value_val = self.get_register(value)?;

                match object_val {
                    RuntimeValue::Object(mut obj) => {
                        obj.properties.insert(property.clone(), value_val);
                        self.set_register(object, RuntimeValue::Object(obj));
                    }
                    RuntimeValue::ClassInstance(instance) => {
                        instance
                            .properties
                            .write()
                            .unwrap()
                            .insert(property.clone(), value_val);
                        self.set_register(object, RuntimeValue::ClassInstance(instance));
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot set property of non-object",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::MethodCall {
                dest,
                object,
                method,
                args,
            } => {
                let object_val = self.get_register(object)?;

                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_register(arg)?);
                }

                let result = self.call_method(object_val, method, arg_values).await?;
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::NewInstance {
                dest,
                class_name,
                args,
            } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_register(arg)?);
                }

                let class_val = self.environment.get(class_name, (0, 0))?;

                match class_val {
                    RuntimeValue::Dynamic(dyn_val) if dyn_val.type_name() == "IRClass" => {
                        let ir_class: &crate::ir::IRClassValue = unsafe {
                            &*(dyn_val.as_ref() as *const dyn crate::runtime::DynamicValue
                                as *const crate::ir::IRClassValue)
                        };

                        let mut instance_props = ir_class.properties.clone();

                        if let Some((params, body)) = &ir_class.constructor {
                            let mut ctor_env = self.environment.clone();
                            ctor_env.push_scope();

                            let temp_instance = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                                instance_props.clone(),
                                crate::ast::types::PrimitiveType::any(),
                            ));
                            ctor_env.declare("this".to_string(), temp_instance)?;

                            for (i, param) in params.iter().enumerate() {
                                let arg_value = arg_values
                                    .get(i)
                                    .cloned()
                                    .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));
                                ctor_env.declare(param.clone(), arg_value)?;
                            }

                            let mut ctor_vm = VM::new(ctor_env, self.type_registry.clone());
                            let ctor_program = IRProgram {
                                instructions: body.clone(),
                                constant_pool: Vec::new(),
                                labels: HashMap::new(),
                            };
                            ctor_vm.execute(ctor_program).await?;

                            if let Ok(updated_this) = ctor_vm.environment.get("this", (0, 0)) {
                                if let RuntimeValue::Object(obj) = updated_this {
                                    instance_props = obj.properties.clone();
                                }
                            }
                        }

                        // Convert IR methods to FunctionValue
                        let mut method_map = HashMap::new();
                        for (method_name, ir_method) in &ir_class.methods {
                            // Convert IR method to FunctionValue
                            let params: Vec<crate::ast::nodes::FnParam> = ir_method
                                .params
                                .iter()
                                .map(|p| crate::ast::nodes::FnParam {
                                    pattern: crate::ast::nodes::VarPattern::Identifier(p.clone()),
                                    param_type: crate::ast::types::PrimitiveType::any(),
                                    default_value: None,
                                    is_optional: false,
                                    is_rest: false,
                                })
                                .collect();

                            // We need to convert IR instructions back to AST statements
                            // For now, we'll store a special marker function that the IR VM will handle
                            let func_value = crate::runtime::FunctionValue::new(
                                params,
                                vec![], // Empty body - we'll execute IR instructions directly
                                ir_method.is_async,
                                crate::ast::types::PrimitiveType::any(),
                            );

                            method_map.insert(method_name.clone(), func_value);
                        }

                        // Create ClassInstance with methods
                        let instance = crate::runtime::ClassInstance::new(
                            ir_class.name.clone(),
                            instance_props.clone(),
                            method_map,
                            vec![],
                            crate::ast::types::PrimitiveType::any(),
                        );

                        self.set_register(dest, RuntimeValue::ClassInstance(instance));
                    }
                    _ => {
                        let instance = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                            HashMap::new(),
                            crate::ast::types::PrimitiveType::any(),
                        ));
                        self.set_register(dest, instance);
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::Await { dest, future } => {
                let future_val = self.get_register(future)?;

                let result = match future_val {
                    RuntimeValue::Future(fut) => fut.wait_for_completion().await.map_err(|e| {
                        RaccoonError::new(&format!("Await error: {}", e), (0, 0), None::<String>)
                    })?,
                    _ => future_val,
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::TypeOf { dest, operand } => {
                let operand_val = self.get_register(operand)?;
                let type_str = Self::get_type_name(&operand_val);
                let result = RuntimeValue::Str(crate::runtime::StrValue::new(type_str));
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::InstanceOf {
                dest,
                operand,
                type_name,
            } => {
                let operand_val = self.get_register(operand)?;

                let is_instance = Self::get_type_name(&operand_val) == *type_name;
                let result = RuntimeValue::Bool(crate::runtime::BoolValue::new(is_instance));
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Throw { value } => {
                let value_val = self.get_register(value)?;
                Err(RaccoonError::new(
                    &format!("Thrown: {}", value_val.to_string()),
                    (0, 0),
                    None::<String>,
                ))
            }

            Instruction::Nop => Ok(ExecutionResult::Continue),

            Instruction::Comment { .. } => Ok(ExecutionResult::Continue),

            Instruction::DestructureArray {
                dests,
                src,
                has_rest,
                rest_dest,
            } => {
                let src_val = self.get_register(src)?;

                match src_val {
                    RuntimeValue::Array(arr) => {
                        for (i, dest) in dests.iter().enumerate() {
                            let value = if i < arr.elements.len() {
                                arr.elements[i].clone()
                            } else {
                                RuntimeValue::Null(crate::runtime::NullValue::new())
                            };
                            self.set_register(dest, value);
                        }

                        if *has_rest {
                            if let Some(rest) = rest_dest {
                                let rest_elements = arr.elements[dests.len()..].to_vec();
                                let rest_array =
                                    RuntimeValue::Array(crate::runtime::ArrayValue::new(
                                        rest_elements,
                                        crate::ast::types::PrimitiveType::any(),
                                    ));
                                self.set_register(rest, rest_array);
                            }
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot destructure non-array value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::DestructureObject {
                mappings,
                src,
                rest_dest: _,
            } => {
                let src_val = self.get_register(src)?;

                match src_val {
                    RuntimeValue::Object(obj) => {
                        for (key, dest) in mappings {
                            let value = obj
                                .properties
                                .get(key)
                                .cloned()
                                .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));
                            self.set_register(dest, value);
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot destructure non-object value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::Increment {
                dest,
                operand,
                is_prefix,
            } => {
                let operand_val = self.get_register(operand)?;

                let (result, new_val) = match operand_val {
                    RuntimeValue::Int(i) => {
                        let new_value = RuntimeValue::Int(crate::runtime::IntValue::new(i.value + 1));
                        let old_value = RuntimeValue::Int(crate::runtime::IntValue::new(i.value));

                        if *is_prefix {
                            (new_value.clone(), new_value)
                        } else {
                            (old_value, new_value)
                        }
                    }
                    RuntimeValue::Float(f) => {
                        let new_value = RuntimeValue::Float(crate::runtime::FloatValue::new(f.value + 1.0));
                        let old_value = RuntimeValue::Float(crate::runtime::FloatValue::new(f.value));

                        if *is_prefix {
                            (new_value.clone(), new_value)
                        } else {
                            (old_value, new_value)
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot increment non-numeric value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                };

                self.set_register(operand, new_val);
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Decrement {
                dest,
                operand,
                is_prefix,
            } => {
                let operand_val = self.get_register(operand)?;

                let (result, new_val) = match operand_val {
                    RuntimeValue::Int(i) => {
                        let new_value = RuntimeValue::Int(crate::runtime::IntValue::new(i.value - 1));
                        let old_value = RuntimeValue::Int(crate::runtime::IntValue::new(i.value));

                        if *is_prefix {
                            (new_value.clone(), new_value)
                        } else {
                            (old_value, new_value)
                        }
                    }
                    RuntimeValue::Float(f) => {
                        let new_value = RuntimeValue::Float(crate::runtime::FloatValue::new(f.value - 1.0));
                        let old_value = RuntimeValue::Float(crate::runtime::FloatValue::new(f.value));

                        if *is_prefix {
                            (new_value.clone(), new_value)
                        } else {
                            (old_value, new_value)
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "Cannot decrement non-numeric value",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                };

                self.set_register(operand, new_val);
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateTemplate { dest, parts } => {
                let mut result = String::new();

                for part in parts {
                    match part {
                        TemplatePart::String(s) => result.push_str(s),
                        TemplatePart::Expr(reg) => {
                            let value = self.get_register(reg)?;
                            result.push_str(&value.to_string());
                        }
                    }
                }

                let template_str = RuntimeValue::Str(crate::runtime::StrValue::new(result));
                self.set_register(dest, template_str);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Match {
                dest,
                scrutinee,
                arms,
            } => {
                let scrutinee_val = self.get_register(scrutinee)?;

                for arm in arms {
                    if self.matches_pattern(&scrutinee_val, &arm.pattern)? {
                        if let Some(guard_reg) = &arm.guard {
                            let guard_val = self.get_register(guard_reg)?;
                            if !crate::interpreter::operators::is_truthy(&guard_val) {
                                continue;
                            }
                        }

                        let mut arm_vm = VM::new(self.environment.clone(), self.type_registry.clone());
                        let arm_program = IRProgram {
                            instructions: arm.body.clone(),
                            constant_pool: Vec::new(),
                            labels: HashMap::new(),
                        };
                        let result = arm_vm.execute(arm_program).await?;
                        self.set_register(dest, result);
                        return Ok(ExecutionResult::Continue);
                    }
                }

                return Err(RaccoonError::new(
                    "No matching pattern in match expression",
                    (0, 0),
                    None::<String>,
                ));
            }

            Instruction::CreateRange { dest, start, end } => {
                let start_val = self.get_register(start)?;
                let end_val = self.get_register(end)?;

                let mut props = HashMap::new();
                props.insert("start".to_string(), start_val);
                props.insert("end".to_string(), end_val);

                let range = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    props,
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, range);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Spread { dest, operand } => {
                let operand_val = self.get_register(operand)?;
                self.set_register(dest, operand_val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Conditional {
                dest,
                condition,
                then_val,
                else_val,
            } => {
                let condition_val = self.get_register(condition)?;
                let is_truthy = crate::interpreter::operators::is_truthy(&condition_val);

                let result = if is_truthy {
                    self.get_register(then_val)?
                } else {
                    self.get_register(else_val)?
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::NullCoalesce { dest, left, right } => {
                let left_val = self.get_register(left)?;

                let result = match left_val {
                    RuntimeValue::Null(_) => self.get_register(right)?,
                    _ => left_val,
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::OptionalChain {
                dest,
                object,
                property,
            } => {
                let object_val = self.get_register(object)?;

                let result = match object_val {
                    RuntimeValue::Null(_) => RuntimeValue::Null(crate::runtime::NullValue::new()),
                    RuntimeValue::Object(obj) => obj
                        .properties
                        .get(property)
                        .cloned()
                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new())),
                    _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
                };

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::PushScope => {
                self.environment.push_scope();
                Ok(ExecutionResult::Continue)
            }

            Instruction::PopScope => {
                self.environment.pop_scope();
                Ok(ExecutionResult::Continue)
            }

            Instruction::Break => Ok(ExecutionResult::Continue),

            Instruction::Continue => Ok(ExecutionResult::Continue),

            Instruction::ForIn {
                variable,
                object,
                body,
            } => {
                let obj_val = self.get_register(object)?;

                match obj_val {
                    RuntimeValue::Object(obj) => {
                        for key in obj.properties.keys() {
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(
                                variable.clone(),
                                RuntimeValue::Str(crate::runtime::StrValue::new(key.clone())),
                            )?;

                            let mut loop_vm = VM::new(loop_env, self.type_registry.clone());
                            let loop_program = IRProgram {
                                instructions: body.clone(),
                                constant_pool: Vec::new(),
                                labels: HashMap::new(),
                            };
                            loop_vm.execute(loop_program).await?;
                        }
                    }
                    RuntimeValue::Array(arr) => {
                        for (i, _) in arr.elements.iter().enumerate() {
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(
                                variable.clone(),
                                RuntimeValue::Int(crate::runtime::IntValue::new(i as i64)),
                            )?;

                            let mut loop_vm = VM::new(loop_env, self.type_registry.clone());
                            let loop_program = IRProgram {
                                instructions: body.clone(),
                                constant_pool: Vec::new(),
                                labels: HashMap::new(),
                            };
                            loop_vm.execute(loop_program).await?;
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "for-in requires object or array",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::ForOf {
                variable,
                iterable,
                body,
            } => {
                let iter_val = self.get_register(iterable)?;

                match iter_val {
                    RuntimeValue::Array(arr) => {
                        for elem in arr.elements {
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(variable.clone(), elem)?;

                            let mut loop_vm = VM::new(loop_env, self.type_registry.clone());
                            let loop_program = IRProgram {
                                instructions: body.clone(),
                                constant_pool: Vec::new(),
                                labels: HashMap::new(),
                            };
                            loop_vm.execute(loop_program).await?;
                        }
                    }
                    _ => {
                        return Err(RaccoonError::new(
                            "for-of requires iterable",
                            (0, 0),
                            None::<String>,
                        ))
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::TryCatch {
                try_body,
                catch_handler,
                finally_body,
            } => {
                let mut try_vm = VM::new(self.environment.clone(), self.type_registry.clone());
                let try_program = IRProgram {
                    instructions: try_body.clone(),
                    constant_pool: Vec::new(),
                    labels: HashMap::new(),
                };

                let try_result = try_vm.execute(try_program).await;

                match try_result {
                    Err(error) if catch_handler.is_some() => {
                        let (error_var, catch_body) = catch_handler.as_ref().unwrap();
                        self.environment.declare(
                            error_var.clone(),
                            RuntimeValue::Str(crate::runtime::StrValue::new(error.message.clone())),
                        )?;

                        let mut catch_vm = VM::new(self.environment.clone(), self.type_registry.clone());
                        let catch_program = IRProgram {
                            instructions: catch_body.clone(),
                            constant_pool: Vec::new(),
                            labels: HashMap::new(),
                        };
                        catch_vm.execute(catch_program).await?;
                    }
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                if let Some(finally) = finally_body {
                    let mut finally_vm = VM::new(self.environment.clone(), self.type_registry.clone());
                    let finally_program = IRProgram {
                        instructions: finally.clone(),
                        constant_pool: Vec::new(),
                        labels: HashMap::new(),
                    };
                    finally_vm.execute(finally_program).await?;
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::LoadThis { dest } => {
                let this_val = self.environment.get("this", (0, 0)).unwrap_or_else(|_| {
                    RuntimeValue::Object(crate::runtime::ObjectValue::new(
                        HashMap::new(),
                        crate::ast::types::PrimitiveType::any(),
                    ))
                });
                self.set_register(dest, this_val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::CallSuper {
                dest,
                method: _,
                args: _,
            } => {
                self.set_register(dest, RuntimeValue::Null(crate::runtime::NullValue::new()));
                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateClass {
                name,
                constructor,
                methods,
                properties,
            } => {
                let mut prop_values = Vec::new();
                for (prop_name, prop_reg) in properties {
                    let value = self.get_register(prop_reg)?;
                    prop_values.push((prop_name.clone(), value));
                }

                let ir_class = crate::ir::IRClassValue::new(
                    name.clone(),
                    constructor.clone(),
                    methods.clone(),
                    prop_values,
                );

                let class_value = RuntimeValue::Dynamic(Box::new(ir_class));
                self.environment.declare(name.clone(), class_value)?;
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadArray { dest, operand } => {
                let val = self.get_register(operand)?;

                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadObject { dest, operand } => {
                let val = self.get_register(operand)?;

                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadCall { dest, operand } => {
                let val = self.get_register(operand)?;

                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Import {
                dest,
                path: _,
                items: _,
            } => {
                let module = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    HashMap::new(),
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, module);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Export { name: _, value: _ } => Ok(ExecutionResult::Continue),

            Instruction::CompoundAssign { dest, src, op } => {
                let dest_val = self.get_register(dest)?;
                let src_val = self.get_register(src)?;

                let result = crate::interpreter::operators::apply_binary_op(
                    dest_val,
                    src_val,
                    op.clone(),
                    (0, 0),
                    &None,
                    &crate::runtime::CallStack::new(),
                )
                .await?;

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::GetIterator { dest, iterable } => {
                let iter_val = self.get_register(iterable)?;

                self.set_register(dest, iter_val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::IteratorNext { dest, iterator } => {
                let _iter = self.get_register(iterator)?;

                let mut result = HashMap::new();
                result.insert(
                    "done".to_string(),
                    RuntimeValue::Bool(crate::runtime::BoolValue::new(true)),
                );
                let iter_result = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    result,
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, iter_result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateGenerator {
                dest,
                name: _,
                params: _,
                body: _,
            } => {
                let gen = RuntimeValue::Null(crate::runtime::NullValue::new());
                self.set_register(dest, gen);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Yield { value } => {
                let val = if let Some(v) = value {
                    self.get_register(v)?
                } else {
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                };
                Ok(ExecutionResult::Return(val))
            }

            Instruction::Catch {
                dest: _,
                promise: _,
                handler: _,
            } => Ok(ExecutionResult::Continue),

            Instruction::Finally { block: _ } => Ok(ExecutionResult::Continue),

            Instruction::TaggedTemplate {
                dest,
                tag,
                parts,
                expressions,
            } => {
                let _tag_fn = self.get_register(tag)?;

                let strings = parts.clone();
                let mut values = Vec::new();

                for expr_reg in expressions {
                    values.push(self.get_register(expr_reg)?);
                }

                let mut result = String::new();
                for (i, part) in strings.iter().enumerate() {
                    result.push_str(part);
                    if i < values.len() {
                        result.push_str(&values[i].to_string());
                    }
                }

                let template_result = RuntimeValue::Str(crate::runtime::StrValue::new(result));
                self.set_register(dest, template_result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::NullAssert { dest, value } => {
                let val = self.get_register(value)?;

                if matches!(val, RuntimeValue::Null(_)) {
                    return Err(RaccoonError::new(
                        "Null assertion failed",
                        (0, 0),
                        None::<String>,
                    ));
                }
                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::DeleteProperty {
                dest,
                object,
                property,
            } => {
                let obj_val = self.get_register(object)?;

                match obj_val {
                    RuntimeValue::Object(mut obj) => {
                        obj.properties.remove(property);
                        let result = RuntimeValue::Bool(crate::runtime::BoolValue::new(true));
                        self.set_register(dest, result);
                    }
                    _ => {
                        let result = RuntimeValue::Bool(crate::runtime::BoolValue::new(false));
                        self.set_register(dest, result);
                    }
                }

                Ok(ExecutionResult::Continue)
            }

            Instruction::In {
                dest,
                property,
                object,
            } => {
                let obj_val = self.get_register(object)?;

                let exists = match obj_val {
                    RuntimeValue::Object(obj) => obj.properties.contains_key(property),
                    _ => false,
                };

                self.set_register(
                    dest,
                    RuntimeValue::Bool(crate::runtime::BoolValue::new(exists)),
                );
                Ok(ExecutionResult::Continue)
            }
        }
    }

    fn get_type_name(value: &RuntimeValue) -> String {
        match value {
            RuntimeValue::Int(_) => "int".to_string(),
            RuntimeValue::BigInt(_) => "bigint".to_string(),
            RuntimeValue::Float(_) => "float".to_string(),
            RuntimeValue::Decimal(_) => "decimal".to_string(),
            RuntimeValue::Str(_) => "str".to_string(),
            RuntimeValue::Bool(_) => "bool".to_string(),
            RuntimeValue::Null(_) => "null".to_string(),
            RuntimeValue::Array(_) => "array".to_string(),
            RuntimeValue::Map(_) => "map".to_string(),
            RuntimeValue::Object(_) => "object".to_string(),
            RuntimeValue::Class(c) => format!("class {}", c.class_name),
            RuntimeValue::ClassInstance(c) => c.class_name.clone(),
            RuntimeValue::Function(_) => "function".to_string(),
            RuntimeValue::NativeFunction(_) => "function".to_string(),
            RuntimeValue::NativeAsyncFunction(_) => "function".to_string(),
            RuntimeValue::Future(_) => "future".to_string(),
            RuntimeValue::Enum(e) => e.enum_name.clone(),
            RuntimeValue::EnumObject(e) => format!("enum {}", e.enum_name),
            RuntimeValue::PrimitiveTypeObject(p) => format!("type {}", p.type_name),
            RuntimeValue::Type(t) => format!("type {}", t.name()),
            RuntimeValue::Dynamic(d) => d.type_name().to_string(),
        }
    }

    fn matches_pattern(
        &self,
        value: &RuntimeValue,
        pattern: &MatchPattern,
    ) -> Result<bool, RaccoonError> {
        match pattern {
            MatchPattern::Wildcard => Ok(true),
            MatchPattern::Literal(lit) => {
                match (value, lit) {
                    (RuntimeValue::Int(v), RuntimeValue::Int(l)) => Ok(v.value == l.value),
                    (RuntimeValue::Float(v), RuntimeValue::Float(l)) => Ok((v.value - l.value).abs() < f64::EPSILON),
                    (RuntimeValue::Str(v), RuntimeValue::Str(l)) => Ok(v.value == l.value),
                    (RuntimeValue::Bool(v), RuntimeValue::Bool(l)) => Ok(v.value == l.value),
                    (RuntimeValue::Null(_), RuntimeValue::Null(_)) => Ok(true),
                    _ => Ok(false),
                }
            }
            MatchPattern::Range(start, end) => {
                match (value, start, end) {
                    (RuntimeValue::Int(v), RuntimeValue::Int(s), RuntimeValue::Int(e)) => {
                        Ok(v.value >= s.value && v.value <= e.value)
                    }
                    (RuntimeValue::Float(v), RuntimeValue::Float(s), RuntimeValue::Float(e)) => {
                        Ok(v.value >= s.value && v.value <= e.value)
                    }
                    _ => Ok(false),
                }
            }
            MatchPattern::Variable(_) => Ok(true),
            MatchPattern::Array(patterns) => {
                match value {
                    RuntimeValue::Array(arr) => {
                        if patterns.len() != arr.elements.len() {
                            return Ok(false);
                        }
                        for (i, p) in patterns.iter().enumerate() {
                            if !self.matches_pattern(&arr.elements[i], p)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
            MatchPattern::Object(props) => {
                match value {
                    RuntimeValue::Object(obj) => {
                        for (key, pattern) in props {
                            if let Some(prop_value) = obj.properties.get(key) {
                                if !self.matches_pattern(prop_value, pattern)? {
                                    return Ok(false);
                                }
                            } else {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
            MatchPattern::Or(patterns) => {
                for p in patterns {
                    if self.matches_pattern(value, p)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }

    async fn call_function(
        &mut self,
        callee: RuntimeValue,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match callee {
            RuntimeValue::NativeFunction(func) => Ok((func.implementation)(args)),
            RuntimeValue::NativeAsyncFunction(func) => Ok((func.implementation)(args).await),
            RuntimeValue::Function(func) => {
                // Execute normal Function using the interpreter
                let mut func_env = self.environment.clone();
                func_env.push_scope();

                // Bind parameters
                for (i, param) in func.parameters.iter().enumerate() {
                    let arg_value = args
                        .get(i)
                        .cloned()
                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));

                    match &param.pattern {
                        crate::ast::nodes::VarPattern::Identifier(name) => {
                            func_env.declare(name.clone(), arg_value)?;
                        }
                        crate::ast::nodes::VarPattern::Destructuring(_) => {
                            // For now, skip destructuring in parameters
                        }
                    }
                }

                // Execute function body using the interpreter
                let mut interpreter = crate::interpreter::Interpreter::new(None);
                interpreter.environment = func_env;
                let mut result = RuntimeValue::Null(crate::runtime::NullValue::new());

                for stmt in &func.body {
                    match interpreter.execute_stmt_internal(stmt).await? {
                        crate::interpreter::InterpreterResult::Value(v) => result = v,
                        crate::interpreter::InterpreterResult::Return(v) => {
                            return Ok(v);
                        }
                        _ => {}
                    }
                }

                Ok(result)
            }
            RuntimeValue::Dynamic(dyn_val) => {
                if dyn_val.type_name() == "IRFunction" {
                    let ir_func: &crate::ir::IRFunctionValue =
                        unsafe { &*(dyn_val.as_ref() as *const dyn crate::runtime::DynamicValue as *const crate::ir::IRFunctionValue) };

                    let mut func_env = self.environment.clone();
                    func_env.push_scope();

                    for (i, param) in ir_func.params.iter().enumerate() {
                        let arg_value = args
                            .get(i)
                            .cloned()
                            .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));
                        func_env.declare(param.clone(), arg_value)?;
                    }

                    let mut func_vm = VM::new(func_env, self.type_registry.clone());
                    let func_program = IRProgram {
                        instructions: ir_func.body.clone(),
                        constant_pool: Vec::new(),
                        labels: ir_func.labels.clone(),
                    };

                    func_vm.execute(func_program).await
                } else {
                    Err(RaccoonError::new(
                        "Cannot call non-function dynamic value",
                        (0, 0),
                        None::<String>,
                    ))
                }
            }
            _ => Err(RaccoonError::new(
                "Cannot call non-function value",
                (0, 0),
                None::<String>,
            )),
        }
    }

    async fn call_method(
        &mut self,
        object: RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match object {
            RuntimeValue::Object(obj) => {
                if let Some(method_val) = obj.properties.get(method) {
                    self.call_function(method_val.clone(), args).await
                } else {
                    Err(RaccoonError::new(
                        &format!("Method '{}' not found", method),
                        (0, 0),
                        None::<String>,
                    ))
                }
            }
            RuntimeValue::ClassInstance(instance) => {
                // First, try to get the IR class from the environment to access IR methods
                if let Ok(class_val) = self.environment.get(&instance.class_name, (0, 0)) {
                    if let RuntimeValue::Dynamic(dyn_val) = class_val {
                        if dyn_val.type_name() == "IRClass" {
                            let ir_class: &crate::ir::IRClassValue = unsafe {
                                &*(dyn_val.as_ref() as *const dyn crate::runtime::DynamicValue
                                    as *const crate::ir::IRClassValue)
                            };

                            // Check if the method exists in IR methods
                            if let Some(ir_method) = ir_class.methods.get(method) {
                                // Execute IR method
                                let mut method_env = self.environment.clone();
                                method_env.push_scope();
                                method_env.declare("this".to_string(), RuntimeValue::ClassInstance(instance.clone()))?;

                                // Bind parameters
                                for (i, param) in ir_method.params.iter().enumerate() {
                                    let arg_value = args
                                        .get(i)
                                        .cloned()
                                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));
                                    method_env.declare(param.clone(), arg_value)?;
                                }

                                // Execute IR instructions
                                let mut method_vm = VM::new(method_env, self.type_registry.clone());
                                let method_program = IRProgram {
                                    instructions: ir_method.body.clone(),
                                    constant_pool: Vec::new(),
                                    labels: ir_method.labels.clone(),
                                };

                                return method_vm.execute(method_program).await;
                            }
                        }
                    }
                }

                // Fallback: check if method exists in instance.methods (for non-IR classes)
                if let Some(method_func) = instance.methods.get(method) {
                    // Only execute if the method has a non-empty body
                    if !method_func.body.is_empty() {
                        let mut method_env = self.environment.clone();
                        method_env.push_scope();
                        method_env.declare("this".to_string(), RuntimeValue::ClassInstance(instance.clone()))?;

                        for (i, param) in method_func.parameters.iter().enumerate() {
                            let arg_value = args
                                .get(i)
                                .cloned()
                                .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()));

                            match &param.pattern {
                                crate::ast::nodes::VarPattern::Identifier(name) => {
                                    method_env.declare(name.clone(), arg_value)?;
                                }
                                crate::ast::nodes::VarPattern::Destructuring(_) => {}
                            }
                        }

                        let mut interpreter = crate::interpreter::Interpreter::new(None);
                        interpreter.environment = method_env;
                        let mut result = RuntimeValue::Null(crate::runtime::NullValue::new());

                        for stmt in &method_func.body {
                            match interpreter.execute_stmt_internal(stmt).await? {
                                crate::interpreter::InterpreterResult::Value(v) => result = v,
                                crate::interpreter::InterpreterResult::Return(v) => {
                                    return Ok(v);
                                }
                                _ => {}
                            }
                        }

                        return Ok(result);
                    }
                }

                Err(RaccoonError::new(
                    &format!("Method '{}' not found", method),
                    (0, 0),
                    None::<String>,
                ))
            }
            RuntimeValue::Str(_)
            | RuntimeValue::Map(_)
            | RuntimeValue::Int(_)
            | RuntimeValue::Float(_)
            | RuntimeValue::Decimal(_)
            | RuntimeValue::BigInt(_)
            | RuntimeValue::Bool(_) => {
                // Use type registry to call instance methods on primitive types
                self.type_registry.call_instance_method(
                    &mut object.clone(),
                    method,
                    args,
                    (0, 0),
                    None::<String>,
                )
            }
            _ => {
                // For other types, return error
                Err(RaccoonError::new(
                    &format!("Cannot call method on non-object type"),
                    (0, 0),
                    None::<String>,
                ))
            }
        }
    }

    fn get_register(&self, reg: &Register) -> Result<RuntimeValue, RaccoonError> {
        match reg {
            Register::Local(name) => {
                self.environment.get(name, (0, 0)).or_else(|_| {
                    let key = reg.to_string();
                    self.registers.get(&key).cloned().ok_or_else(|| {
                        RaccoonError::new(
                            &format!("Register not found: {}", key),
                            (0, 0),
                            None::<String>,
                        )
                    })
                })
            }
            _ => {
                let key = reg.to_string();
                self.registers.get(&key).cloned().ok_or_else(|| {
                    RaccoonError::new(
                        &format!("Register not found: {}", key),
                        (0, 0),
                        None::<String>,
                    )
                })
            }
        }
    }

    fn set_register(&mut self, reg: &Register, value: RuntimeValue) {
        match reg {
            Register::Local(name) => {
                if self.environment.get(name, (0, 0)).is_ok() {
                    let _ = self.environment.assign(name, value.clone(), (0, 0));
                }
                let key = reg.to_string();
                self.registers.insert(key, value);
            }
            _ => {
                let key = reg.to_string();
                self.registers.insert(key, value);
            }
        }
    }

    fn resolve_label(&self, label: &str) -> Option<usize> {
        self.program
            .as_ref()
            .and_then(|p| p.labels.get(label).copied())
    }

    fn get_program_len(&self) -> usize {
        self.program
            .as_ref()
            .map(|p| p.instructions.len())
            .unwrap_or(0)
    }

    fn get_instruction(&self, pos: usize) -> &Instruction {
        &self.program.as_ref().unwrap().instructions[pos]
    }
}

enum ExecutionResult {
    Continue,
    Return(RuntimeValue),
    Jump(String),
}
