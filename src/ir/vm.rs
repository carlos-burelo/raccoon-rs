use crate::error::RaccoonError;
use crate::runtime::{Environment, RuntimeValue};
use async_recursion::async_recursion;
use std::collections::HashMap;

use super::instruction::{IRProgram, Instruction, MatchPattern, Register, TemplatePart};

/// Virtual Machine for executing IR bytecode
pub struct VM {
    /// Register file - maps registers to runtime values
    registers: HashMap<String, RuntimeValue>,
    /// Environment for variable storage
    environment: Environment,
    /// Program counter
    pc: usize,
    /// Current executing program
    program: Option<IRProgram>,
}

impl VM {
    pub fn new(environment: Environment) -> Self {
        Self {
            registers: HashMap::new(),
            environment,
            pc: 0,
            program: None,
        }
    }

    /// Execute an IR program
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

    /// Execute a single instruction
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

                // Use the existing binary operator logic from the interpreter
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

                // Apply unary operator
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
                    crate::tokens::UnaryOperator::BitwiseNot => {
                        // Simplified bitwise not
                        match operand_val {
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
                        }
                    }
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

            Instruction::Label { .. } => {
                // Labels are just markers, no execution needed
                Ok(ExecutionResult::Continue)
            }

            Instruction::Call { dest, callee, args } => {
                let callee_val = self.get_register(callee)?;

                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_register(arg)?);
                }

                // Execute function call
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
                name: _,
                params: _,
                body: _,
                is_async: _,
            } => {
                // Create a closure/function value (simplified - storing as a native function for now)
                // In a full implementation, we would store the IR body and execute it later
                let function = RuntimeValue::Null(crate::runtime::NullValue::new());
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
                            }
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
                        "length" => RuntimeValue::Int(crate::runtime::IntValue::new(arr.elements.len() as i64)),
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
                        "length" => RuntimeValue::Int(crate::runtime::IntValue::new(s.value.len() as i64)),
                        "isEmpty" => RuntimeValue::Bool(crate::runtime::BoolValue::new(s.value.is_empty())),
                        _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
                    },
                    RuntimeValue::ClassInstance(instance) => {
                        // Try to get property from instance
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

                // Call method on object
                let result = self.call_method(object_val, method, arg_values).await?;
                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::NewInstance {
                dest,
                class_name: _,
                args,
            } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_register(arg)?);
                }

                // Create new instance (simplified)
                let instance = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    HashMap::new(),
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, instance);
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
                // Simplified instanceof check
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

                let result = match operand_val {
                    RuntimeValue::Int(i) => {
                        let new_val = i.value + 1;
                        if *is_prefix {
                            RuntimeValue::Int(crate::runtime::IntValue::new(new_val))
                        } else {
                            // Post-increment returns old value
                            self.set_register(
                                operand,
                                RuntimeValue::Int(crate::runtime::IntValue::new(new_val)),
                            );
                            RuntimeValue::Int(crate::runtime::IntValue::new(i.value))
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

                self.set_register(dest, result);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Decrement {
                dest,
                operand,
                is_prefix,
            } => {
                let operand_val = self.get_register(operand)?;

                let result = match operand_val {
                    RuntimeValue::Int(i) => {
                        let new_val = i.value - 1;
                        if *is_prefix {
                            RuntimeValue::Int(crate::runtime::IntValue::new(new_val))
                        } else {
                            // Post-decrement returns old value
                            self.set_register(
                                operand,
                                RuntimeValue::Int(crate::runtime::IntValue::new(new_val)),
                            );
                            RuntimeValue::Int(crate::runtime::IntValue::new(i.value))
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

                // Try each arm in order
                for arm in arms {
                    if self.matches_pattern(&scrutinee_val, &arm.pattern)? {
                        // Check guard if present
                        if let Some(guard_reg) = &arm.guard {
                            let guard_val = self.get_register(guard_reg)?;
                            if !crate::interpreter::operators::is_truthy(&guard_val) {
                                continue;
                            }
                        }

                        // Execute arm body
                        let mut arm_vm = VM::new(self.environment.clone());
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

                // No match found
                return Err(RaccoonError::new(
                    "No matching pattern in match expression",
                    (0, 0),
                    None::<String>,
                ));
            }

            Instruction::CreateRange { dest, start, end } => {
                let start_val = self.get_register(start)?;
                let end_val = self.get_register(end)?;

                // Create a range object (simplified)
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

            Instruction::Break => {
                // Break is handled by loop instructions
                Ok(ExecutionResult::Continue)
            }

            Instruction::Continue => {
                // Continue is handled by loop instructions
                Ok(ExecutionResult::Continue)
            }

            Instruction::ForIn {
                variable,
                object,
                body,
            } => {
                let obj_val = self.get_register(object)?;

                match obj_val {
                    RuntimeValue::Object(obj) => {
                        for key in obj.properties.keys() {
                            // Create a new scope for each iteration
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(
                                variable.clone(),
                                RuntimeValue::Str(crate::runtime::StrValue::new(key.clone())),
                            )?;

                            let mut loop_vm = VM::new(loop_env);
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
                            // Create a new scope for each iteration
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(
                                variable.clone(),
                                RuntimeValue::Int(crate::runtime::IntValue::new(i as i64)),
                            )?;

                            let mut loop_vm = VM::new(loop_env);
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
                            // Create a new scope for each iteration
                            let mut loop_env = self.environment.clone();
                            loop_env.push_scope();
                            loop_env.declare(variable.clone(), elem)?;

                            let mut loop_vm = VM::new(loop_env);
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
                let mut try_vm = VM::new(self.environment.clone());
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

                        let mut catch_vm = VM::new(self.environment.clone());
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
                    let mut finally_vm = VM::new(self.environment.clone());
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
                // Simplified: load 'this' (in a full implementation, track current 'this')
                let this_val = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    HashMap::new(),
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, this_val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::CallSuper {
                dest,
                method: _,
                args: _,
            } => {
                // Simplified: super call (full implementation would require inheritance tracking)
                self.set_register(dest, RuntimeValue::Null(crate::runtime::NullValue::new()));
                Ok(ExecutionResult::Continue)
            }

            Instruction::CreateClass {
                name,
                constructor: _,
                methods: _,
                properties: _,
            } => {
                // Create a class value
                let class = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    HashMap::new(),
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.environment.declare(name.clone(), class)?;
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadArray { dest, operand } => {
                let val = self.get_register(operand)?;
                // For now, just pass through the value
                // In a full implementation, this would be handled by CreateArray
                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadObject { dest, operand } => {
                let val = self.get_register(operand)?;
                // For now, just pass through the value
                // In a full implementation, this would be handled by CreateObject
                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::SpreadCall { dest, operand } => {
                let val = self.get_register(operand)?;
                // For now, just pass through the value
                // In a full implementation, this would expand arguments in a call
                self.set_register(dest, val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Import {
                dest,
                path: _,
                items: _,
            } => {
                // Simplified: return a module object
                let module = RuntimeValue::Object(crate::runtime::ObjectValue::new(
                    HashMap::new(),
                    crate::ast::types::PrimitiveType::any(),
                ));
                self.set_register(dest, module);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Export { name: _, value: _ } => {
                // Export is handled at compile time mostly
                Ok(ExecutionResult::Continue)
            }

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
                // Simplified: return the iterable itself
                self.set_register(dest, iter_val);
                Ok(ExecutionResult::Continue)
            }

            Instruction::IteratorNext { dest, iterator } => {
                let _iter = self.get_register(iterator)?;
                // Simplified: return done=true
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
                // Simplified: create a generator function as null for now
                let gen = RuntimeValue::Null(crate::runtime::NullValue::new());
                self.set_register(dest, gen);
                Ok(ExecutionResult::Continue)
            }

            Instruction::Yield { value } => {
                // Simplified: just return the value
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
            } => {
                // Promise catch - simplified implementation
                Ok(ExecutionResult::Continue)
            }

            Instruction::Finally { block: _ } => {
                // Finally block - already handled in TryCatch
                Ok(ExecutionResult::Continue)
            }

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

                // Call the tag function with strings and values
                // Simplified: just concatenate for now
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
                // Check if null and throw if so
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

    /// Get the type name of a runtime value
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

    /// Check if a value matches a pattern
    fn matches_pattern(
        &self,
        value: &RuntimeValue,
        pattern: &MatchPattern,
    ) -> Result<bool, RaccoonError> {
        match pattern {
            MatchPattern::Wildcard => Ok(true),
            MatchPattern::Literal(lit) => {
                // Simplified comparison
                Ok(value.to_string() == lit.to_string())
            }
            MatchPattern::Variable(_) => Ok(true), // Variable patterns always match
            MatchPattern::Array(_) => {
                // Simplified array pattern matching
                Ok(matches!(value, RuntimeValue::Array(_)))
            }
            MatchPattern::Object(_) => {
                // Simplified object pattern matching
                Ok(matches!(value, RuntimeValue::Object(_)))
            }
            MatchPattern::Or(patterns) => {
                for p in patterns {
                    if self.matches_pattern(value, p)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    /// Call a function
    async fn call_function(
        &mut self,
        callee: RuntimeValue,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match callee {
            RuntimeValue::NativeFunction(func) => {
                Ok((func.implementation)(args))
            }
            RuntimeValue::NativeAsyncFunction(func) => {
                Ok((func.implementation)(args).await)
            }
            RuntimeValue::Function(_) => {
                // TODO: Implement user-defined function calls
                // This would require:
                // 1. Setting up a new scope
                // 2. Binding parameters
                // 3. Executing the function body
                // 4. Managing the call stack
                Ok(RuntimeValue::Null(crate::runtime::NullValue::new()))
            }
            _ => {
                Err(RaccoonError::new(
                    "Cannot call non-function value",
                    (0, 0),
                    None::<String>,
                ))
            }
        }
    }

    /// Call a method on an object
    async fn call_method(
        &mut self,
        object: RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match object {
            RuntimeValue::Object(obj) => {
                // Try to get the method from the object
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
            RuntimeValue::ClassInstance(_) => {
                // For class instances, call the method from the class
                // In a full implementation, would look up the method in the class definition
                Ok(RuntimeValue::Null(crate::runtime::NullValue::new()))
            }
            _ => {
                // For other types, try to use the type handler
                // This would use the runtime type system to find the method
                Ok(RuntimeValue::Null(crate::runtime::NullValue::new()))
            }
        }
    }

    /// Get a register value
    fn get_register(&self, reg: &Register) -> Result<RuntimeValue, RaccoonError> {
        let key = reg.to_string();
        self.registers.get(&key).cloned().ok_or_else(|| {
            RaccoonError::new(
                &format!("Register not found: {}", key),
                (0, 0),
                None::<String>,
            )
        })
    }

    /// Set a register value
    fn set_register(&mut self, reg: &Register, value: RuntimeValue) {
        let key = reg.to_string();
        self.registers.insert(key, value);
    }

    /// Resolve a label to an instruction position
    fn resolve_label(&self, label: &str) -> Option<usize> {
        self.program
            .as_ref()
            .and_then(|p| p.labels.get(label).copied())
    }

    /// Get the current program length
    fn get_program_len(&self) -> usize {
        self.program
            .as_ref()
            .map(|p| p.instructions.len())
            .unwrap_or(0)
    }

    /// Get an instruction at a specific position
    fn get_instruction(&self, pos: usize) -> &Instruction {
        &self.program.as_ref().unwrap().instructions[pos]
    }
}

/// Result of executing an instruction
enum ExecutionResult {
    Continue,
    Return(RuntimeValue),
    Jump(String),
}
