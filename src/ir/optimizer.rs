use crate::runtime::RuntimeValue;
use crate::tokens::BinaryOperator;
use std::collections::{HashMap, HashSet};

use super::instruction::{IRProgram, Instruction};

pub struct IROptimizer {
    program: IRProgram,
}

impl IROptimizer {
    pub fn new(program: IRProgram) -> Self {
        Self { program }
    }

    pub fn optimize(mut self) -> IRProgram {
        for _ in 0..3 {
            self.constant_folding();
            self.dead_code_elimination();
            self.remove_nops();
            self.jump_threading();
        }

        self.program
    }

    fn constant_folding(&mut self) {
        let mut constants: HashMap<String, RuntimeValue> = HashMap::new();
        let mut new_instructions = Vec::new();

        for instruction in &self.program.instructions {
            match instruction {
                Instruction::LoadConst { dest, value } => {
                    constants.insert(dest.to_string(), value.clone());
                    new_instructions.push(instruction.clone());
                }

                Instruction::BinaryOp {
                    dest,
                    left,
                    right,
                    op,
                } => {
                    let left_const = constants.get(&left.to_string());
                    let right_const = constants.get(&right.to_string());

                    if let (Some(left_val), Some(right_val)) = (left_const, right_const) {
                        if let Some(result) = Self::fold_binary_op(left_val, right_val, op) {
                            constants.insert(dest.to_string(), result.clone());
                            new_instructions.push(Instruction::LoadConst {
                                dest: dest.clone(),
                                value: result,
                            });
                            continue;
                        }
                    }

                    constants.remove(&dest.to_string());
                    new_instructions.push(instruction.clone());
                }

                Instruction::UnaryOp { dest, operand, op } => {
                    let operand_const = constants.get(&operand.to_string());

                    if let Some(operand_val) = operand_const {
                        if let Some(result) = Self::fold_unary_op(operand_val, op) {
                            constants.insert(dest.to_string(), result.clone());
                            new_instructions.push(Instruction::LoadConst {
                                dest: dest.clone(),
                                value: result,
                            });
                            continue;
                        }
                    }

                    constants.remove(&dest.to_string());
                    new_instructions.push(instruction.clone());
                }

                Instruction::Move { dest, src } => {
                    if let Some(src_val) = constants.get(&src.to_string()) {
                        constants.insert(dest.to_string(), src_val.clone());
                    } else {
                        constants.remove(&dest.to_string());
                    }
                    new_instructions.push(instruction.clone());
                }

                _ => {
                    if let Some(dest) = instruction.dest_register() {
                        constants.remove(&dest.to_string());
                    }
                    new_instructions.push(instruction.clone());
                }
            }
        }

        self.program.instructions = new_instructions;
    }

    fn fold_binary_op(
        left: &RuntimeValue,
        right: &RuntimeValue,
        op: &BinaryOperator,
    ) -> Option<RuntimeValue> {
        use crate::runtime::{BoolValue, FloatValue, IntValue, StrValue};

        match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                let result = match op {
                    BinaryOperator::Add => Some(IntValue::new(l.value + r.value)),
                    BinaryOperator::Subtract => Some(IntValue::new(l.value - r.value)),
                    BinaryOperator::Multiply => Some(IntValue::new(l.value * r.value)),
                    BinaryOperator::Divide => {
                        if r.value != 0 {
                            Some(IntValue::new(l.value / r.value))
                        } else {
                            None
                        }
                    }
                    BinaryOperator::Modulo => {
                        if r.value != 0 {
                            Some(IntValue::new(l.value % r.value))
                        } else {
                            None
                        }
                    }
                    BinaryOperator::Equal => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value == r.value)))
                    }
                    BinaryOperator::NotEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value != r.value)))
                    }
                    BinaryOperator::LessThan => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                    }
                    BinaryOperator::GreaterThan => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                    }
                    BinaryOperator::LessEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                    }
                    BinaryOperator::GreaterEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                    }
                    _ => None,
                };
                result.map(RuntimeValue::Int)
            }

            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                let result = match op {
                    BinaryOperator::Add => Some(FloatValue::new(l.value + r.value)),
                    BinaryOperator::Subtract => Some(FloatValue::new(l.value - r.value)),
                    BinaryOperator::Multiply => Some(FloatValue::new(l.value * r.value)),
                    BinaryOperator::Divide => Some(FloatValue::new(l.value / r.value)),
                    BinaryOperator::Modulo => Some(FloatValue::new(l.value % r.value)),
                    BinaryOperator::Equal => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value == r.value)))
                    }
                    BinaryOperator::NotEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value != r.value)))
                    }
                    BinaryOperator::LessThan => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                    }
                    BinaryOperator::GreaterThan => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                    }
                    BinaryOperator::LessEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                    }
                    BinaryOperator::GreaterEqual => {
                        return Some(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                    }
                    _ => None,
                };
                result.map(RuntimeValue::Float)
            }

            (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                if matches!(op, BinaryOperator::Add) {
                    Some(RuntimeValue::Str(StrValue::new(format!(
                        "{}{}",
                        l.value, r.value
                    ))))
                } else if matches!(op, BinaryOperator::Equal) {
                    Some(RuntimeValue::Bool(BoolValue::new(l.value == r.value)))
                } else if matches!(op, BinaryOperator::NotEqual) {
                    Some(RuntimeValue::Bool(BoolValue::new(l.value != r.value)))
                } else {
                    None
                }
            }

            (RuntimeValue::Bool(l), RuntimeValue::Bool(r)) => {
                let result = match op {
                    BinaryOperator::And => BoolValue::new(l.value && r.value),
                    BinaryOperator::Or => BoolValue::new(l.value || r.value),
                    BinaryOperator::Equal => BoolValue::new(l.value == r.value),
                    BinaryOperator::NotEqual => BoolValue::new(l.value != r.value),
                    _ => return None,
                };
                Some(RuntimeValue::Bool(result))
            }

            _ => None,
        }
    }

    fn fold_unary_op(
        operand: &RuntimeValue,
        op: &crate::tokens::UnaryOperator,
    ) -> Option<RuntimeValue> {
        use crate::runtime::{BoolValue, FloatValue, IntValue};

        match operand {
            RuntimeValue::Int(i) => {
                if matches!(op, crate::tokens::UnaryOperator::Negate) {
                    Some(RuntimeValue::Int(IntValue::new(-i.value)))
                } else {
                    None
                }
            }

            RuntimeValue::Float(f) => {
                if matches!(op, crate::tokens::UnaryOperator::Negate) {
                    Some(RuntimeValue::Float(FloatValue::new(-f.value)))
                } else {
                    None
                }
            }

            RuntimeValue::Bool(b) => {
                if matches!(op, crate::tokens::UnaryOperator::Not) {
                    Some(RuntimeValue::Bool(BoolValue::new(!b.value)))
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    fn dead_code_elimination(&mut self) {
        let reachable = self.compute_reachable_instructions();
        let mut new_instructions = Vec::new();

        for (i, instruction) in self.program.instructions.iter().enumerate() {
            if reachable.contains(&i) {
                new_instructions.push(instruction.clone());
            }
        }

        self.program.instructions = new_instructions;
    }

    fn compute_reachable_instructions(&self) -> HashSet<usize> {
        let mut reachable = HashSet::new();
        let mut worklist = vec![0];

        while let Some(pos) = worklist.pop() {
            if pos >= self.program.instructions.len() || reachable.contains(&pos) {
                continue;
            }

            reachable.insert(pos);

            let instruction = &self.program.instructions[pos];

            match instruction {
                Instruction::Jump { label } => {
                    if let Some(target) = self.program.labels.get(label) {
                        worklist.push(*target);
                    }
                }

                Instruction::JumpIfFalse { label, .. } | Instruction::JumpIfTrue { label, .. } => {
                    if let Some(target) = self.program.labels.get(label) {
                        worklist.push(*target);
                    }
                    worklist.push(pos + 1);
                }

                Instruction::Return { .. } | Instruction::Throw { .. } => {}

                _ => {
                    worklist.push(pos + 1);
                }
            }
        }

        reachable
    }

    fn remove_nops(&mut self) {
        self.program
            .instructions
            .retain(|inst| !matches!(inst, Instruction::Nop));
    }

    fn jump_threading(&mut self) {
        let mut label_targets: HashMap<String, String> = HashMap::new();

        for instruction in &self.program.instructions {
            if let Instruction::Label { name } = instruction {
                let next_pos = self.program.labels.get(name).map(|&p| p + 1);
                if let Some(pos) = next_pos {
                    if pos < self.program.instructions.len() {
                        if let Instruction::Jump { label } = &self.program.instructions[pos] {
                            label_targets.insert(name.clone(), label.clone());
                        }
                    }
                }
            }
        }

        let mut new_instructions = Vec::new();
        for instruction in &self.program.instructions {
            let new_inst = match instruction {
                Instruction::Jump { label } => {
                    let final_label = Self::resolve_jump_chain(label, &label_targets);
                    Instruction::Jump { label: final_label }
                }

                Instruction::JumpIfFalse { condition, label } => {
                    let final_label = Self::resolve_jump_chain(label, &label_targets);
                    Instruction::JumpIfFalse {
                        condition: condition.clone(),
                        label: final_label,
                    }
                }

                Instruction::JumpIfTrue { condition, label } => {
                    let final_label = Self::resolve_jump_chain(label, &label_targets);
                    Instruction::JumpIfTrue {
                        condition: condition.clone(),
                        label: final_label,
                    }
                }

                _ => instruction.clone(),
            };

            new_instructions.push(new_inst);
        }

        self.program.instructions = new_instructions;
    }

    fn resolve_jump_chain(label: &str, label_targets: &HashMap<String, String>) -> String {
        let mut current = label.to_string();
        let mut visited = HashSet::new();

        while let Some(next) = label_targets.get(&current) {
            if visited.contains(&current) {
                break;
            }
            visited.insert(current.clone());
            current = next.clone();
        }

        current
    }

    #[allow(dead_code)]
    fn copy_propagation(&mut self) {
        let mut copies: HashMap<String, String> = HashMap::new();
        let mut new_instructions = Vec::new();

        for instruction in &self.program.instructions {
            match instruction {
                Instruction::Move { dest, src } => {
                    copies.insert(dest.to_string(), src.to_string());
                    new_instructions.push(instruction.clone());
                }

                _ => {
                    let _sources = instruction.source_registers();
                    let new_inst = instruction.clone();

                    new_instructions.push(new_inst);

                    if let Some(dest) = instruction.dest_register() {
                        copies.remove(&dest.to_string());
                    }
                }
            }
        }

        self.program.instructions = new_instructions;
    }

    #[allow(dead_code)]
    fn strength_reduction(&mut self) {
        let mut new_instructions = Vec::new();

        for instruction in &self.program.instructions {
            match instruction {
                Instruction::BinaryOp {
                    dest: _,
                    left: _,
                    right: _,
                    op: _,
                } => {
                    new_instructions.push(instruction.clone());
                }

                _ => new_instructions.push(instruction.clone()),
            }
        }

        self.program.instructions = new_instructions;
    }
}
