use crate::ast::nodes::*;
use crate::ast::types::Type;
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use std::collections::HashMap;

use super::instruction::{
    Instruction, IRProgram, MatchArm as IRMatchArm, MatchPattern as IRMatchPattern, Register,
    TemplatePart,
};

/// Compiler from AST to IR
pub struct IRCompiler {
    program: IRProgram,
    temp_counter: usize,
    label_counter: usize,
    scope_depth: usize,
}

impl IRCompiler {
    pub fn new() -> Self {
        Self {
            program: IRProgram::new(),
            temp_counter: 0,
            label_counter: 0,
            scope_depth: 0,
        }
    }

    /// Compile a program to IR
    pub fn compile(mut self, program: &Program) -> Result<IRProgram, RaccoonError> {
        for stmt in &program.stmts {
            self.compile_stmt(stmt)?;
        }
        Ok(self.program)
    }

    /// Generate a new temporary register
    fn next_temp(&mut self) -> Register {
        let reg = Register::Temp(self.temp_counter);
        self.temp_counter += 1;
        reg
    }

    /// Generate a new label name
    fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Compile a statement
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), RaccoonError> {
        match stmt {
            Stmt::Program(program) => {
                for stmt in &program.stmts {
                    self.compile_stmt(stmt)?;
                }
                Ok(())
            }
            Stmt::VarDecl(decl) => self.compile_var_decl(decl),
            Stmt::FnDecl(decl) => self.compile_fn_decl(decl),
            Stmt::ClassDecl(decl) => self.compile_class_decl(decl),
            Stmt::InterfaceDecl(_) => Ok(()), // Interfaces are type-level only
            Stmt::EnumDecl(decl) => self.compile_enum_decl(decl),
            Stmt::TypeAliasDecl(_) => Ok(()), // Type aliases are type-level only
            Stmt::ImportDecl(_) => Ok(()),    // Handled separately
            Stmt::ExportDecl(_) => Ok(()),    // Handled separately
            Stmt::Block(block) => self.compile_block(block),
            Stmt::IfStmt(if_stmt) => self.compile_if_stmt(if_stmt),
            Stmt::WhileStmt(while_stmt) => self.compile_while_stmt(while_stmt),
            Stmt::DoWhileStmt(do_while) => self.compile_do_while_stmt(do_while),
            Stmt::ForStmt(for_stmt) => self.compile_for_stmt(for_stmt),
            Stmt::ForInStmt(for_in) => self.compile_for_in_stmt(for_in),
            Stmt::ForOfStmt(for_of) => self.compile_for_of_stmt(for_of),
            Stmt::SwitchStmt(switch_stmt) => self.compile_switch_stmt(switch_stmt),
            Stmt::ReturnStmt(ret) => self.compile_return_stmt(ret),
            Stmt::BreakStmt(_) => {
                self.program.emit(Instruction::Jump {
                    label: "break".to_string(),
                });
                Ok(())
            }
            Stmt::ContinueStmt(_) => {
                self.program.emit(Instruction::Jump {
                    label: "continue".to_string(),
                });
                Ok(())
            }
            Stmt::ExprStmt(expr_stmt) => {
                self.compile_expr(&expr_stmt.expression)?;
                Ok(())
            }
            Stmt::TryStmt(try_stmt) => self.compile_try_stmt(try_stmt),
            Stmt::ThrowStmt(throw) => self.compile_throw_stmt(throw),
        }
    }

    /// Compile a variable declaration
    fn compile_var_decl(&mut self, decl: &VarDecl) -> Result<(), RaccoonError> {
        match &decl.pattern {
            VarPattern::Identifier(name) => {
                // Declare variable
                self.program.emit(Instruction::Declare {
                    name: name.clone(),
                    is_const: decl.is_constant,
                });

                // Initialize if there's an initializer
                if let Some(init) = &decl.initializer {
                    let value_reg = self.compile_expr(init)?;
                    self.program.emit(Instruction::Store {
                        name: name.clone(),
                        src: value_reg,
                    });
                }
            }
            VarPattern::Destructuring(pattern) => {
                if let Some(init) = &decl.initializer {
                    let value_reg = self.compile_expr(init)?;
                    self.compile_destructuring_pattern(pattern, value_reg)?;
                }
            }
        }
        Ok(())
    }

    /// Compile destructuring pattern
    fn compile_destructuring_pattern(
        &mut self,
        pattern: &DestructuringPattern,
        src: Register,
    ) -> Result<(), RaccoonError> {
        match pattern {
            DestructuringPattern::Array(array_pattern) => {
                let mut dests = Vec::new();
                for element in &array_pattern.elements {
                    if let Some(elem) = element {
                        let dest = match elem {
                            ArrayPatternElement::Identifier(ident) => {
                                Register::Local(ident.name.clone())
                            }
                            _ => self.next_temp(),
                        };
                        dests.push(dest);
                    }
                }

                let rest_dest = array_pattern
                    .rest
                    .as_ref()
                    .map(|rest| Register::Local(rest.argument.name.clone()));

                self.program.emit(Instruction::DestructureArray {
                    dests,
                    src,
                    has_rest: array_pattern.rest.is_some(),
                    rest_dest,
                });
            }
            DestructuringPattern::Object(object_pattern) => {
                let mut mappings = Vec::new();
                for prop in &object_pattern.properties {
                    let dest = match &prop.value {
                        ObjectPatternValue::Identifier(ident) => {
                            Register::Local(ident.name.clone())
                        }
                        _ => self.next_temp(),
                    };
                    mappings.push((prop.key.clone(), dest));
                }

                let rest_dest = object_pattern
                    .rest
                    .as_ref()
                    .map(|rest| Register::Local(rest.argument.name.clone()));

                self.program.emit(Instruction::DestructureObject {
                    mappings,
                    src,
                    rest_dest,
                });
            }
        }
        Ok(())
    }

    /// Compile a function declaration
    fn compile_fn_decl(&mut self, decl: &FnDecl) -> Result<(), RaccoonError> {
        let mut params = Vec::new();
        for param in &decl.parameters {
            if let VarPattern::Identifier(name) = &param.pattern {
                params.push(name.clone());
            }
        }

        // Compile function body in a new compiler context
        let mut body_compiler = IRCompiler::new();
        for stmt in &decl.body {
            body_compiler.compile_stmt(stmt)?;
        }

        let dest = Register::Global(decl.name.clone());
        self.program.emit(Instruction::CreateFunction {
            dest,
            name: decl.name.clone(),
            params,
            body: body_compiler.program.instructions,
            is_async: decl.is_async,
        });

        Ok(())
    }

    /// Compile a class declaration
    fn compile_class_decl(&mut self, _decl: &ClassDecl) -> Result<(), RaccoonError> {
        // TODO: Implement class compilation
        // For now, this is a placeholder
        Ok(())
    }

    /// Compile an enum declaration
    fn compile_enum_decl(&mut self, decl: &EnumDecl) -> Result<(), RaccoonError> {
        // Create an object with enum members
        let mut properties = Vec::new();
        let mut value = 0i64;

        for member in &decl.members {
            let member_value = if let Some(val_expr) = &member.value {
                self.compile_expr(val_expr)?
            } else {
                let reg = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: reg.clone(),
                    value: RuntimeValue::Int(crate::runtime::IntValue::new(value)),
                });
                value += 1;
                reg
            };
            properties.push((member.name.clone(), member_value));
        }

        let enum_obj = self.next_temp();
        self.program.emit(Instruction::CreateObject {
            dest: enum_obj.clone(),
            properties,
        });

        self.program.emit(Instruction::Store {
            name: decl.name.clone(),
            src: enum_obj,
        });

        Ok(())
    }

    /// Compile a block statement
    fn compile_block(&mut self, block: &Block) -> Result<(), RaccoonError> {
        self.program.emit(Instruction::PushScope);
        self.scope_depth += 1;

        for stmt in &block.statements {
            self.compile_stmt(stmt)?;
        }

        self.scope_depth -= 1;
        self.program.emit(Instruction::PopScope);
        Ok(())
    }

    /// Compile an if statement
    fn compile_if_stmt(&mut self, if_stmt: &IfStmt) -> Result<(), RaccoonError> {
        let condition_reg = self.compile_expr(&if_stmt.condition)?;

        let else_label = self.next_label("else");
        let end_label = self.next_label("endif");

        // Jump to else if condition is false
        self.program.emit(Instruction::JumpIfFalse {
            condition: condition_reg,
            label: else_label.clone(),
        });

        // Then branch
        self.compile_stmt(&if_stmt.then_branch)?;
        self.program.emit(Instruction::Jump {
            label: end_label.clone(),
        });

        // Else branch
        self.program.emit_label(else_label);
        if let Some(else_branch) = &if_stmt.else_branch {
            self.compile_stmt(else_branch)?;
        }

        self.program.emit_label(end_label);
        Ok(())
    }

    /// Compile a while statement
    fn compile_while_stmt(&mut self, while_stmt: &WhileStmt) -> Result<(), RaccoonError> {
        let start_label = self.next_label("while_start");
        let end_label = self.next_label("while_end");

        self.program.emit_label(start_label.clone());

        let condition_reg = self.compile_expr(&while_stmt.condition)?;
        self.program.emit(Instruction::JumpIfFalse {
            condition: condition_reg,
            label: end_label.clone(),
        });

        self.compile_stmt(&while_stmt.body)?;

        self.program.emit(Instruction::Jump {
            label: start_label,
        });

        self.program.emit_label(end_label);
        Ok(())
    }

    /// Compile a do-while statement
    fn compile_do_while_stmt(&mut self, do_while: &DoWhileStmt) -> Result<(), RaccoonError> {
        let start_label = self.next_label("do_start");

        self.program.emit_label(start_label.clone());

        self.compile_stmt(&do_while.body)?;

        let condition_reg = self.compile_expr(&do_while.condition)?;
        self.program.emit(Instruction::JumpIfTrue {
            condition: condition_reg,
            label: start_label,
        });

        Ok(())
    }

    /// Compile a for statement
    fn compile_for_stmt(&mut self, for_stmt: &ForStmt) -> Result<(), RaccoonError> {
        self.program.emit(Instruction::PushScope);

        // Initialize
        if let Some(init) = &for_stmt.initializer {
            self.compile_stmt(init)?;
        }

        let start_label = self.next_label("for_start");
        let end_label = self.next_label("for_end");
        let continue_label = self.next_label("for_continue");

        self.program.emit_label(start_label.clone());

        // Condition
        if let Some(condition) = &for_stmt.condition {
            let condition_reg = self.compile_expr(condition)?;
            self.program.emit(Instruction::JumpIfFalse {
                condition: condition_reg,
                label: end_label.clone(),
            });
        }

        // Body
        self.compile_stmt(&for_stmt.body)?;

        // Continue point
        self.program.emit_label(continue_label);

        // Increment
        if let Some(increment) = &for_stmt.increment {
            self.compile_expr(increment)?;
        }

        self.program.emit(Instruction::Jump {
            label: start_label,
        });

        self.program.emit_label(end_label);
        self.program.emit(Instruction::PopScope);
        Ok(())
    }

    /// Compile a for-in statement
    fn compile_for_in_stmt(&mut self, _for_in: &ForInStmt) -> Result<(), RaccoonError> {
        // TODO: Implement for-in compilation
        Ok(())
    }

    /// Compile a for-of statement
    fn compile_for_of_stmt(&mut self, _for_of: &ForOfStmt) -> Result<(), RaccoonError> {
        // TODO: Implement for-of compilation
        Ok(())
    }

    /// Compile a switch statement
    fn compile_switch_stmt(&mut self, switch_stmt: &SwitchStmt) -> Result<(), RaccoonError> {
        let discriminant_reg = self.compile_expr(&switch_stmt.discriminant)?;
        let end_label = self.next_label("switch_end");

        for (i, case) in switch_stmt.cases.iter().enumerate() {
            if let Some(test) = &case.test {
                let test_reg = self.compile_expr(test)?;
                let temp = self.next_temp();

                // Compare discriminant with test
                self.program.emit(Instruction::BinaryOp {
                    dest: temp.clone(),
                    left: discriminant_reg.clone(),
                    right: test_reg,
                    op: crate::tokens::BinaryOperator::Equal,
                });

                let next_case_label = self.next_label(&format!("case_{}", i + 1));
                self.program.emit(Instruction::JumpIfFalse {
                    condition: temp,
                    label: next_case_label.clone(),
                });

                // Case consequent
                for stmt in &case.consequent {
                    self.compile_stmt(stmt)?;
                }

                self.program.emit(Instruction::Jump {
                    label: end_label.clone(),
                });

                self.program.emit_label(next_case_label);
            } else {
                // Default case
                for stmt in &case.consequent {
                    self.compile_stmt(stmt)?;
                }
            }
        }

        self.program.emit_label(end_label);
        Ok(())
    }

    /// Compile a return statement
    fn compile_return_stmt(&mut self, ret: &ReturnStmt) -> Result<(), RaccoonError> {
        let value = if let Some(val_expr) = &ret.value {
            Some(self.compile_expr(val_expr)?)
        } else {
            None
        };

        self.program.emit(Instruction::Return { value });
        Ok(())
    }

    /// Compile a try statement
    fn compile_try_stmt(&mut self, _try_stmt: &TryStmt) -> Result<(), RaccoonError> {
        // TODO: Implement try-catch compilation
        // This requires exception handling support in the VM
        Ok(())
    }

    /// Compile a throw statement
    fn compile_throw_stmt(&mut self, throw: &ThrowStmt) -> Result<(), RaccoonError> {
        let value_reg = self.compile_expr(&throw.value)?;
        self.program.emit(Instruction::Throw { value: value_reg });
        Ok(())
    }

    /// Compile an expression and return the register containing the result
    fn compile_expr(&mut self, expr: &Expr) -> Result<Register, RaccoonError> {
        match expr {
            Expr::IntLiteral(lit) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: dest.clone(),
                    value: RuntimeValue::Int(crate::runtime::IntValue::new(lit.value)),
                });
                Ok(dest)
            }
            Expr::FloatLiteral(lit) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: dest.clone(),
                    value: RuntimeValue::Float(crate::runtime::FloatValue::new(lit.value)),
                });
                Ok(dest)
            }
            Expr::StrLiteral(lit) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: dest.clone(),
                    value: RuntimeValue::Str(crate::runtime::StrValue::new(lit.value.clone())),
                });
                Ok(dest)
            }
            Expr::BoolLiteral(lit) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: dest.clone(),
                    value: RuntimeValue::Bool(crate::runtime::BoolValue::new(lit.value)),
                });
                Ok(dest)
            }
            Expr::NullLiteral(_) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::LoadConst {
                    dest: dest.clone(),
                    value: RuntimeValue::Null(crate::runtime::NullValue::new()),
                });
                Ok(dest)
            }
            Expr::Identifier(ident) => {
                let dest = self.next_temp();
                self.program.emit(Instruction::Load {
                    dest: dest.clone(),
                    name: ident.name.clone(),
                });
                Ok(dest)
            }
            Expr::Binary(binary) => self.compile_binary_expr(binary),
            Expr::Unary(unary) => self.compile_unary_expr(unary),
            Expr::Assignment(assign) => self.compile_assignment(assign),
            Expr::Call(call) => self.compile_call_expr(call),
            Expr::ArrayLiteral(array) => self.compile_array_literal(array),
            Expr::ObjectLiteral(obj) => self.compile_object_literal(obj),
            Expr::Member(member) => self.compile_member_expr(member),
            Expr::Index(index) => self.compile_index_expr(index),
            Expr::Conditional(cond) => self.compile_conditional_expr(cond),
            Expr::UnaryUpdate(update) => self.compile_unary_update(update),
            Expr::TemplateStr(template) => self.compile_template_str(template),
            Expr::ArrowFn(arrow) => self.compile_arrow_fn(arrow),
            Expr::TypeOf(typeof_expr) => self.compile_typeof_expr(typeof_expr),
            Expr::InstanceOf(instanceof) => self.compile_instanceof_expr(instanceof),
            Expr::OptionalChaining(opt_chain) => self.compile_optional_chaining(opt_chain),
            Expr::MethodCall(method_call) => self.compile_method_call(method_call),
            Expr::New(new_expr) => self.compile_new_expr(new_expr),
            Expr::Range(range) => self.compile_range_expr(range),
            Expr::NullCoalescing(null_coal) => self.compile_null_coalescing(null_coal),
            Expr::Await(await_expr) => self.compile_await_expr(await_expr),
            Expr::Match(match_expr) => self.compile_match_expr(match_expr),
            _ => {
                // For unimplemented expressions, return a placeholder
                let dest = self.next_temp();
                self.program.emit(Instruction::Comment {
                    text: format!("Unimplemented expression: {:?}", expr),
                });
                Ok(dest)
            }
        }
    }

    /// Compile a binary expression
    fn compile_binary_expr(&mut self, binary: &BinaryExpr) -> Result<Register, RaccoonError> {
        let left = self.compile_expr(&binary.left)?;
        let right = self.compile_expr(&binary.right)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::BinaryOp {
            dest: dest.clone(),
            left,
            right,
            op: binary.operator.clone(),
        });

        Ok(dest)
    }

    /// Compile a unary expression
    fn compile_unary_expr(&mut self, unary: &UnaryExpr) -> Result<Register, RaccoonError> {
        let operand = self.compile_expr(&unary.operand)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::UnaryOp {
            dest: dest.clone(),
            operand,
            op: unary.operator.clone(),
        });

        Ok(dest)
    }

    /// Compile an assignment
    fn compile_assignment(&mut self, assign: &Assignment) -> Result<Register, RaccoonError> {
        let value_reg = self.compile_expr(&assign.value)?;

        match assign.target.as_ref() {
            Expr::Identifier(ident) => {
                self.program.emit(Instruction::Store {
                    name: ident.name.clone(),
                    src: value_reg.clone(),
                });
            }
            Expr::Member(member) => {
                let object_reg = self.compile_expr(&member.object)?;
                self.program.emit(Instruction::StoreProperty {
                    object: object_reg,
                    property: member.property.clone(),
                    value: value_reg.clone(),
                });
            }
            Expr::Index(index) => {
                let array_reg = self.compile_expr(&index.object)?;
                let index_reg = self.compile_expr(&index.index)?;
                self.program.emit(Instruction::StoreIndex {
                    array: array_reg,
                    index: index_reg,
                    value: value_reg.clone(),
                });
            }
            _ => {
                return Err(RaccoonError::new(
                    "Invalid assignment target",
                    assign.position,
                    None::<String>,
                ))
            }
        }

        Ok(value_reg)
    }

    /// Compile a call expression
    fn compile_call_expr(&mut self, call: &CallExpr) -> Result<Register, RaccoonError> {
        let callee = self.compile_expr(&call.callee)?;

        let mut args = Vec::new();
        for arg in &call.args {
            args.push(self.compile_expr(arg)?);
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::Call {
            dest: dest.clone(),
            callee,
            args,
        });

        Ok(dest)
    }

    /// Compile an array literal
    fn compile_array_literal(&mut self, array: &ArrayLiteral) -> Result<Register, RaccoonError> {
        let mut elements = Vec::new();
        for elem in &array.elements {
            elements.push(self.compile_expr(elem)?);
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::CreateArray {
            dest: dest.clone(),
            elements,
        });

        Ok(dest)
    }

    /// Compile an object literal
    fn compile_object_literal(
        &mut self,
        obj: &ObjectLiteral,
    ) -> Result<Register, RaccoonError> {
        let mut properties = Vec::new();

        for prop in &obj.properties {
            match prop {
                ObjectLiteralProperty::KeyValue { key, value } => {
                    let value_reg = self.compile_expr(value)?;
                    properties.push((key.clone(), value_reg));
                }
                ObjectLiteralProperty::Spread(_) => {
                    // TODO: Handle spread in objects
                }
            }
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::CreateObject {
            dest: dest.clone(),
            properties,
        });

        Ok(dest)
    }

    /// Compile a member expression
    fn compile_member_expr(&mut self, member: &MemberExpr) -> Result<Register, RaccoonError> {
        let object = self.compile_expr(&member.object)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::LoadProperty {
            dest: dest.clone(),
            object,
            property: member.property.clone(),
        });

        Ok(dest)
    }

    /// Compile an index expression
    fn compile_index_expr(&mut self, index: &IndexExpr) -> Result<Register, RaccoonError> {
        let array = self.compile_expr(&index.object)?;
        let index_reg = self.compile_expr(&index.index)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::LoadIndex {
            dest: dest.clone(),
            array,
            index: index_reg,
        });

        Ok(dest)
    }

    /// Compile a conditional expression
    fn compile_conditional_expr(
        &mut self,
        cond: &ConditionalExpr,
    ) -> Result<Register, RaccoonError> {
        let condition = self.compile_expr(&cond.condition)?;
        let then_val = self.compile_expr(&cond.then_expr)?;
        let else_val = self.compile_expr(&cond.else_expr)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::Conditional {
            dest: dest.clone(),
            condition,
            then_val,
            else_val,
        });

        Ok(dest)
    }

    /// Compile a unary update expression (++/--)
    fn compile_unary_update(
        &mut self,
        update: &UnaryUpdateExpr,
    ) -> Result<Register, RaccoonError> {
        let operand = self.compile_expr(&update.operand)?;
        let dest = self.next_temp();

        match update.operator {
            crate::ast::nodes::UpdateOperator::Increment => {
                self.program.emit(Instruction::Increment {
                    dest: dest.clone(),
                    operand,
                    is_prefix: update.is_prefix,
                });
            }
            crate::ast::nodes::UpdateOperator::Decrement => {
                self.program.emit(Instruction::Decrement {
                    dest: dest.clone(),
                    operand,
                    is_prefix: update.is_prefix,
                });
            }
        }

        Ok(dest)
    }

    /// Compile a template string
    fn compile_template_str(
        &mut self,
        template: &TemplateStrExpr,
    ) -> Result<Register, RaccoonError> {
        let mut parts = Vec::new();

        for part in &template.parts {
            match part {
                crate::ast::nodes::TemplateStrPart::String(s) => {
                    parts.push(TemplatePart::String(s.value.clone()));
                }
                crate::ast::nodes::TemplateStrPart::Expr(expr) => {
                    let reg = self.compile_expr(expr)?;
                    parts.push(TemplatePart::Expr(reg));
                }
            }
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::CreateTemplate {
            dest: dest.clone(),
            parts,
        });

        Ok(dest)
    }

    /// Compile an arrow function
    fn compile_arrow_fn(&mut self, arrow: &ArrowFnExpr) -> Result<Register, RaccoonError> {
        let mut params = Vec::new();
        for param in &arrow.parameters {
            if let VarPattern::Identifier(name) = &param.pattern {
                params.push(name.clone());
            }
        }

        let body_instructions = match &arrow.body {
            ArrowFnBody::Expr(expr) => {
                let mut body_compiler = IRCompiler::new();
                let result = body_compiler.compile_expr(expr)?;
                body_compiler.program.emit(Instruction::Return {
                    value: Some(result),
                });
                body_compiler.program.instructions
            }
            ArrowFnBody::Block(stmts) => {
                let mut body_compiler = IRCompiler::new();
                for stmt in stmts {
                    body_compiler.compile_stmt(stmt)?;
                }
                body_compiler.program.instructions
            }
        };

        let dest = self.next_temp();
        self.program.emit(Instruction::CreateFunction {
            dest: dest.clone(),
            name: "<arrow>".to_string(),
            params,
            body: body_instructions,
            is_async: arrow.is_async,
        });

        Ok(dest)
    }

    /// Compile a typeof expression
    fn compile_typeof_expr(&mut self, typeof_expr: &TypeOfExpr) -> Result<Register, RaccoonError> {
        let operand = self.compile_expr(&typeof_expr.operand)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::TypeOf {
            dest: dest.clone(),
            operand,
        });

        Ok(dest)
    }

    /// Compile an instanceof expression
    fn compile_instanceof_expr(
        &mut self,
        instanceof: &InstanceOfExpr,
    ) -> Result<Register, RaccoonError> {
        let operand = self.compile_expr(&instanceof.operand)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::InstanceOf {
            dest: dest.clone(),
            operand,
            type_name: instanceof.type_name.clone(),
        });

        Ok(dest)
    }

    /// Compile optional chaining
    fn compile_optional_chaining(
        &mut self,
        opt_chain: &OptionalChainingExpr,
    ) -> Result<Register, RaccoonError> {
        let object = self.compile_expr(&opt_chain.object)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::OptionalChain {
            dest: dest.clone(),
            object,
            property: opt_chain.property.clone(),
        });

        Ok(dest)
    }

    /// Compile a method call
    fn compile_method_call(
        &mut self,
        method_call: &MethodCallExpr,
    ) -> Result<Register, RaccoonError> {
        let object = self.compile_expr(&method_call.object)?;

        let mut args = Vec::new();
        for arg in &method_call.args {
            args.push(self.compile_expr(arg)?);
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::MethodCall {
            dest: dest.clone(),
            object,
            method: method_call.method.clone(),
            args,
        });

        Ok(dest)
    }

    /// Compile a new expression
    fn compile_new_expr(&mut self, new_expr: &NewExpr) -> Result<Register, RaccoonError> {
        let mut args = Vec::new();
        for arg in &new_expr.args {
            args.push(self.compile_expr(arg)?);
        }

        let dest = self.next_temp();
        self.program.emit(Instruction::NewInstance {
            dest: dest.clone(),
            class_name: new_expr.class_name.clone(),
            args,
        });

        Ok(dest)
    }

    /// Compile a range expression
    fn compile_range_expr(&mut self, range: &RangeExpr) -> Result<Register, RaccoonError> {
        let start = self.compile_expr(&range.start)?;
        let end = self.compile_expr(&range.end)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::CreateRange {
            dest: dest.clone(),
            start,
            end,
        });

        Ok(dest)
    }

    /// Compile null coalescing
    fn compile_null_coalescing(
        &mut self,
        null_coal: &NullCoalescingExpr,
    ) -> Result<Register, RaccoonError> {
        let left = self.compile_expr(&null_coal.left)?;
        let right = self.compile_expr(&null_coal.right)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::NullCoalesce {
            dest: dest.clone(),
            left,
            right,
        });

        Ok(dest)
    }

    /// Compile an await expression
    fn compile_await_expr(&mut self, await_expr: &AwaitExpr) -> Result<Register, RaccoonError> {
        let future = self.compile_expr(&await_expr.expression)?;
        let dest = self.next_temp();

        self.program.emit(Instruction::Await {
            dest: dest.clone(),
            future,
        });

        Ok(dest)
    }

    /// Compile a match expression
    fn compile_match_expr(&mut self, match_expr: &MatchExpr) -> Result<Register, RaccoonError> {
        let scrutinee = self.compile_expr(&match_expr.scrutinee)?;
        let dest = self.next_temp();

        let mut arms = Vec::new();
        for arm in &match_expr.arms {
            let pattern = self.compile_pattern(&arm.pattern)?;
            let guard = if let Some(guard_expr) = &arm.guard {
                Some(self.compile_expr(guard_expr)?)
            } else {
                None
            };

            let mut body_compiler = IRCompiler::new();
            let body_reg = body_compiler.compile_expr(&arm.body)?;
            body_compiler.program.emit(Instruction::Return {
                value: Some(body_reg),
            });

            arms.push(IRMatchArm {
                pattern,
                guard,
                body: body_compiler.program.instructions,
            });
        }

        self.program.emit(Instruction::Match {
            dest: dest.clone(),
            scrutinee,
            arms,
        });

        Ok(dest)
    }

    /// Compile a pattern for matching
    fn compile_pattern(&mut self, pattern: &Pattern) -> Result<IRMatchPattern, RaccoonError> {
        match pattern {
            Pattern::Wildcard(_) => Ok(IRMatchPattern::Wildcard),
            Pattern::Literal(_expr) => {
                // For literals, we need to extract the runtime value
                // This is a simplification; in reality, we'd need to evaluate constant expressions
                Ok(IRMatchPattern::Literal(RuntimeValue::Null(
                    crate::runtime::NullValue::new(),
                )))
            }
            Pattern::Range(_start, _end) => {
                // Similar simplification for ranges
                Ok(IRMatchPattern::Range(
                    RuntimeValue::Null(crate::runtime::NullValue::new()),
                    RuntimeValue::Null(crate::runtime::NullValue::new()),
                ))
            }
            Pattern::Variable(name) => Ok(IRMatchPattern::Variable(name.clone())),
            Pattern::Array(patterns) => {
                let mut compiled_patterns = Vec::new();
                for p in patterns {
                    compiled_patterns.push(self.compile_pattern(p)?);
                }
                Ok(IRMatchPattern::Array(compiled_patterns))
            }
            Pattern::Object(props) => {
                let mut compiled_props = Vec::new();
                for (key, pattern) in props {
                    compiled_props.push((key.clone(), self.compile_pattern(pattern)?));
                }
                Ok(IRMatchPattern::Object(compiled_props))
            }
            Pattern::Or(patterns) => {
                let mut compiled_patterns = Vec::new();
                for p in patterns {
                    compiled_patterns.push(self.compile_pattern(p)?);
                }
                Ok(IRMatchPattern::Or(compiled_patterns))
            }
            _ => Ok(IRMatchPattern::Wildcard),
        }
    }
}

impl Default for IRCompiler {
    fn default() -> Self {
        Self::new()
    }
}
