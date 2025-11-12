use crate::runtime::RuntimeValue;
use crate::tokens::{BinaryOperator, UnaryOperator};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Temp(usize),

    Local(String),

    Global(String),
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::Temp(n) => write!(f, "r{}", n),
            Register::Local(name) => write!(f, "local:{}", name),
            Register::Global(name) => write!(f, "global:{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst {
        dest: Register,
        value: RuntimeValue,
    },

    Move {
        dest: Register,
        src: Register,
    },

    Declare {
        name: String,
        is_const: bool,
    },

    Store {
        name: String,
        src: Register,
    },

    Load {
        dest: Register,
        name: String,
    },

    BinaryOp {
        dest: Register,
        left: Register,
        right: Register,
        op: BinaryOperator,
    },

    UnaryOp {
        dest: Register,
        operand: Register,
        op: UnaryOperator,
    },

    Jump {
        label: String,
    },

    JumpIfFalse {
        condition: Register,
        label: String,
    },

    JumpIfTrue {
        condition: Register,
        label: String,
    },

    Label {
        name: String,
    },

    Call {
        dest: Register,
        callee: Register,
        args: Vec<Register>,
    },

    Return {
        value: Option<Register>,
    },

    CreateFunction {
        dest: Register,
        name: String,
        params: Vec<String>,
        body: Vec<Instruction>,
        labels: std::collections::HashMap<String, usize>,
        is_async: bool,
    },

    CreateArray {
        dest: Register,
        elements: Vec<Register>,
    },

    LoadIndex {
        dest: Register,
        array: Register,
        index: Register,
    },

    StoreIndex {
        array: Register,
        index: Register,
        value: Register,
    },

    CreateObject {
        dest: Register,
        properties: Vec<(String, Register)>,
    },

    LoadProperty {
        dest: Register,
        object: Register,
        property: String,
    },

    StoreProperty {
        object: Register,
        property: String,
        value: Register,
    },

    MethodCall {
        dest: Register,
        object: Register,
        method: String,
        args: Vec<Register>,
    },

    NewInstance {
        dest: Register,
        class_name: String,
        args: Vec<Register>,
    },

    Await {
        dest: Register,
        future: Register,
    },

    TypeOf {
        dest: Register,
        operand: Register,
    },

    InstanceOf {
        dest: Register,
        operand: Register,
        type_name: String,
    },

    Throw {
        value: Register,
    },

    Nop,

    Comment {
        text: String,
    },

    DestructureArray {
        dests: Vec<Register>,
        src: Register,
        has_rest: bool,
        rest_dest: Option<Register>,
    },

    DestructureObject {
        mappings: Vec<(String, Register)>,
        src: Register,
        rest_dest: Option<Register>,
    },

    Increment {
        dest: Register,
        operand: Register,
        is_prefix: bool,
    },

    Decrement {
        dest: Register,
        operand: Register,
        is_prefix: bool,
    },

    CreateTemplate {
        dest: Register,
        parts: Vec<TemplatePart>,
    },

    Match {
        dest: Register,
        scrutinee: Register,
        arms: Vec<MatchArm>,
    },

    CreateRange {
        dest: Register,
        start: Register,
        end: Register,
    },

    Spread {
        dest: Register,
        operand: Register,
    },

    Conditional {
        dest: Register,
        condition: Register,
        then_val: Register,
        else_val: Register,
    },

    NullCoalesce {
        dest: Register,
        left: Register,
        right: Register,
    },

    OptionalChain {
        dest: Register,
        object: Register,
        property: String,
    },

    PushScope,

    PopScope,

    Break,

    Continue,

    TryCatch {
        try_body: Vec<Instruction>,
        catch_handler: Option<(String, Vec<Instruction>)>,
        finally_body: Option<Vec<Instruction>>,
    },

    CreateClass {
        name: String,
        constructor: Option<(Vec<String>, Vec<Instruction>)>,
        methods: Vec<(String, Vec<String>, Vec<Instruction>, std::collections::HashMap<String, usize>, bool)>,
        properties: Vec<(String, Register)>,
    },

    LoadThis {
        dest: Register,
    },

    CallSuper {
        dest: Register,
        method: String,
        args: Vec<Register>,
    },

    SpreadArray {
        dest: Register,
        operand: Register,
    },

    SpreadObject {
        dest: Register,
        operand: Register,
    },

    SpreadCall {
        dest: Register,
        operand: Register,
    },

    Import {
        dest: Register,
        path: String,
        items: Vec<String>,
    },

    Export {
        name: String,
        value: Register,
    },

    ForIn {
        variable: String,
        object: Register,
        body: Vec<Instruction>,
    },

    ForOf {
        variable: String,
        iterable: Register,
        body: Vec<Instruction>,
    },

    CompoundAssign {
        dest: Register,
        src: Register,
        op: BinaryOperator,
    },

    GetIterator {
        dest: Register,
        iterable: Register,
    },

    IteratorNext {
        dest: Register,
        iterator: Register,
    },

    Yield {
        value: Option<Register>,
    },

    CreateGenerator {
        dest: Register,
        name: String,
        params: Vec<String>,
        body: Vec<Instruction>,
    },

    Catch {
        dest: Register,
        promise: Register,
        handler: Vec<Instruction>,
    },

    Finally {
        block: Vec<Instruction>,
    },

    TaggedTemplate {
        dest: Register,
        tag: Register,
        parts: Vec<String>,
        expressions: Vec<Register>,
    },

    NullAssert {
        dest: Register,
        value: Register,
    },

    DeleteProperty {
        dest: Register,
        object: Register,
        property: String,
    },

    In {
        dest: Register,
        property: String,
        object: Register,
    },
}

#[derive(Debug, Clone)]
pub enum TemplatePart {
    String(String),
    Expr(Register),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub guard: Option<Register>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    Wildcard,
    Literal(RuntimeValue),
    Range(RuntimeValue, RuntimeValue),
    Variable(String),
    Array(Vec<MatchPattern>),
    Object(Vec<(String, MatchPattern)>),
    Or(Vec<MatchPattern>),
}

impl Instruction {
    pub fn is_control_flow(&self) -> bool {
        matches!(
            self,
            Instruction::Jump { .. }
                | Instruction::JumpIfFalse { .. }
                | Instruction::JumpIfTrue { .. }
                | Instruction::Return { .. }
                | Instruction::Throw { .. }
                | Instruction::Break
                | Instruction::Continue
                | Instruction::TryCatch { .. }
                | Instruction::ForIn { .. }
                | Instruction::ForOf { .. }
        )
    }

    pub fn has_side_effects(&self) -> bool {
        matches!(
            self,
            Instruction::Store { .. }
                | Instruction::StoreIndex { .. }
                | Instruction::StoreProperty { .. }
                | Instruction::Call { .. }
                | Instruction::MethodCall { .. }
                | Instruction::Throw { .. }
                | Instruction::Declare { .. }
        )
    }

    pub fn dest_register(&self) -> Option<&Register> {
        match self {
            Instruction::LoadConst { dest, .. }
            | Instruction::Move { dest, .. }
            | Instruction::Load { dest, .. }
            | Instruction::BinaryOp { dest, .. }
            | Instruction::UnaryOp { dest, .. }
            | Instruction::Call { dest, .. }
            | Instruction::CreateFunction { dest, .. }
            | Instruction::CreateArray { dest, .. }
            | Instruction::LoadIndex { dest, .. }
            | Instruction::CreateObject { dest, .. }
            | Instruction::LoadProperty { dest, .. }
            | Instruction::MethodCall { dest, .. }
            | Instruction::NewInstance { dest, .. }
            | Instruction::Await { dest, .. }
            | Instruction::TypeOf { dest, .. }
            | Instruction::InstanceOf { dest, .. }
            | Instruction::Increment { dest, .. }
            | Instruction::Decrement { dest, .. }
            | Instruction::CreateTemplate { dest, .. }
            | Instruction::Match { dest, .. }
            | Instruction::CreateRange { dest, .. }
            | Instruction::Spread { dest, .. }
            | Instruction::Conditional { dest, .. }
            | Instruction::NullCoalesce { dest, .. }
            | Instruction::OptionalChain { dest, .. }
            | Instruction::LoadThis { dest }
            | Instruction::CallSuper { dest, .. }
            | Instruction::SpreadArray { dest, .. }
            | Instruction::SpreadObject { dest, .. }
            | Instruction::SpreadCall { dest, .. }
            | Instruction::Import { dest, .. }
            | Instruction::GetIterator { dest, .. }
            | Instruction::IteratorNext { dest, .. }
            | Instruction::CreateGenerator { dest, .. }
            | Instruction::TaggedTemplate { dest, .. }
            | Instruction::NullAssert { dest, .. }
            | Instruction::DeleteProperty { dest, .. }
            | Instruction::In { dest, .. } => Some(dest),
            _ => None,
        }
    }

    pub fn source_registers(&self) -> Vec<&Register> {
        let mut sources = Vec::new();
        match self {
            Instruction::LoadConst { .. }
            | Instruction::Declare { .. }
            | Instruction::Label { .. }
            | Instruction::Jump { .. }
            | Instruction::Load { .. }
            | Instruction::Nop
            | Instruction::Comment { .. }
            | Instruction::PushScope
            | Instruction::PopScope
            | Instruction::Break
            | Instruction::Continue
            | Instruction::LoadThis { .. }
            | Instruction::Import { .. }
            | Instruction::CreateGenerator { .. }
            | Instruction::Finally { .. }
            | Instruction::TryCatch { .. }
            | Instruction::CreateClass { .. } => {}

            Instruction::Return { value } => {
                if let Some(val) = value {
                    sources.push(val);
                }
            }

            Instruction::Move { src, .. } => sources.push(src),
            Instruction::Store { src, .. } => sources.push(src),

            Instruction::BinaryOp { left, right, .. } => {
                sources.push(left);
                sources.push(right);
            }

            Instruction::UnaryOp { operand, .. } => sources.push(operand),

            Instruction::JumpIfFalse { condition, .. }
            | Instruction::JumpIfTrue { condition, .. } => sources.push(condition),

            Instruction::Call { callee, args, .. } => {
                sources.push(callee);
                sources.extend(args.iter());
            }

            Instruction::CreateArray { elements, .. } => sources.extend(elements.iter()),
            Instruction::LoadIndex { array, index, .. } => {
                sources.push(array);
                sources.push(index);
            }
            Instruction::StoreIndex {
                array,
                index,
                value,
            } => {
                sources.push(array);
                sources.push(index);
                sources.push(value);
            }

            Instruction::CreateObject { properties, .. } => {
                sources.extend(properties.iter().map(|(_, reg)| reg));
            }
            Instruction::LoadProperty { object, .. } => sources.push(object),
            Instruction::StoreProperty { object, value, .. } => {
                sources.push(object);
                sources.push(value);
            }
            Instruction::DeleteProperty { object, .. } => sources.push(object),
            Instruction::In { object, .. } => sources.push(object),

            Instruction::MethodCall { object, args, .. } => {
                sources.push(object);
                sources.extend(args.iter());
            }

            Instruction::NewInstance { args, .. } => sources.extend(args.iter()),

            Instruction::Await { future, .. } => sources.push(future),

            Instruction::TypeOf { operand, .. } | Instruction::InstanceOf { operand, .. } => {
                sources.push(operand)
            }

            Instruction::Throw { value } => sources.push(value),
            Instruction::Catch { promise, .. } => sources.push(promise),

            Instruction::DestructureArray { src, .. }
            | Instruction::DestructureObject { src, .. } => sources.push(src),

            Instruction::Increment { operand, .. } | Instruction::Decrement { operand, .. } => {
                sources.push(operand)
            }

            Instruction::CreateTemplate { parts, .. } => {
                for part in parts {
                    if let TemplatePart::Expr(reg) = part {
                        sources.push(reg);
                    }
                }
            }
            Instruction::TaggedTemplate {
                tag, expressions, ..
            } => {
                sources.push(tag);
                sources.extend(expressions.iter());
            }

            Instruction::Match {
                scrutinee, arms, ..
            } => {
                sources.push(scrutinee);
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        sources.push(guard);
                    }
                }
            }

            Instruction::CreateRange { start, end, .. } => {
                sources.push(start);
                sources.push(end);
            }

            Instruction::Spread { operand, .. }
            | Instruction::SpreadArray { operand, .. }
            | Instruction::SpreadObject { operand, .. }
            | Instruction::SpreadCall { operand, .. } => sources.push(operand),

            Instruction::Conditional {
                condition,
                then_val,
                else_val,
                ..
            } => {
                sources.push(condition);
                sources.push(then_val);
                sources.push(else_val);
            }

            Instruction::NullCoalesce { left, right, .. } => {
                sources.push(left);
                sources.push(right);
            }
            Instruction::NullAssert { value, .. } => sources.push(value),
            Instruction::OptionalChain { object, .. } => sources.push(object),

            Instruction::CallSuper { args, .. } => sources.extend(args.iter()),

            Instruction::Export { value, .. } => sources.push(value),

            Instruction::ForIn { object, .. } => sources.push(object),
            Instruction::ForOf { iterable, .. } => sources.push(iterable),

            Instruction::CompoundAssign { dest, src, .. } => {
                sources.push(dest);
                sources.push(src);
            }

            Instruction::GetIterator { iterable, .. } => sources.push(iterable),
            Instruction::IteratorNext { iterator, .. } => sources.push(iterator),

            Instruction::Yield { value } => {
                if let Some(val) = value {
                    sources.push(val);
                }
            }

            Instruction::CreateFunction { .. } => {}
        }
        sources
    }
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<RuntimeValue>,
    pub labels: std::collections::HashMap<String, usize>,
}

impl IRProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
            labels: std::collections::HashMap::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn emit_label(&mut self, name: String) {
        let position = self.instructions.len();
        self.labels.insert(name.clone(), position);
        self.emit(Instruction::Label { name });
    }

    pub fn current_position(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_constant(&mut self, value: RuntimeValue) -> usize {
        for (i, existing) in self.constant_pool.iter().enumerate() {
            if std::ptr::eq(existing, &value) {
                return i;
            }
        }
        let index = self.constant_pool.len();
        self.constant_pool.push(value);
        index
    }
}

impl Default for IRProgram {
    fn default() -> Self {
        Self::new()
    }
}
