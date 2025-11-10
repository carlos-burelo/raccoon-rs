use crate::runtime::RuntimeValue;
use crate::tokens::{BinaryOperator, UnaryOperator};
use std::fmt;

/// Register-based IR with bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    /// Temporary register (r0, r1, r2, ...)
    Temp(usize),
    /// Local variable register
    Local(String),
    /// Global variable register
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

/// IR Instruction Set - designed for optimization and efficient execution
#[derive(Debug, Clone)]
pub enum Instruction {
    // === Constants and Literals ===
    /// Load a constant value into a register
    LoadConst {
        dest: Register,
        value: RuntimeValue,
    },

    // === Variable Operations ===
    /// Load from a register to another register
    Move {
        dest: Register,
        src: Register,
    },
    /// Declare a variable
    Declare {
        name: String,
        is_const: bool,
    },
    /// Store value from register to variable
    Store {
        name: String,
        src: Register,
    },
    /// Load variable value into register
    Load {
        dest: Register,
        name: String,
    },

    // === Arithmetic Operations ===
    /// Binary operation: dest = left op right
    BinaryOp {
        dest: Register,
        left: Register,
        right: Register,
        op: BinaryOperator,
    },
    /// Unary operation: dest = op operand
    UnaryOp {
        dest: Register,
        operand: Register,
        op: UnaryOperator,
    },

    // === Control Flow ===
    /// Unconditional jump to label
    Jump {
        label: String,
    },
    /// Jump if condition register is false
    JumpIfFalse {
        condition: Register,
        label: String,
    },
    /// Jump if condition register is true
    JumpIfTrue {
        condition: Register,
        label: String,
    },
    /// Label marker for jumps
    Label {
        name: String,
    },

    // === Function Operations ===
    /// Call a function: dest = callee(args...)
    Call {
        dest: Register,
        callee: Register,
        args: Vec<Register>,
    },
    /// Return from function with optional value
    Return {
        value: Option<Register>,
    },
    /// Create a function/closure
    CreateFunction {
        dest: Register,
        name: String,
        params: Vec<String>,
        body: Vec<Instruction>,
        is_async: bool,
    },

    // === Array Operations ===
    /// Create an array: dest = [elements...]
    CreateArray {
        dest: Register,
        elements: Vec<Register>,
    },
    /// Load array element: dest = array[index]
    LoadIndex {
        dest: Register,
        array: Register,
        index: Register,
    },
    /// Store array element: array[index] = value
    StoreIndex {
        array: Register,
        index: Register,
        value: Register,
    },

    // === Object Operations ===
    /// Create an object: dest = { key1: val1, key2: val2, ... }
    CreateObject {
        dest: Register,
        properties: Vec<(String, Register)>,
    },
    /// Load object property: dest = obj.property
    LoadProperty {
        dest: Register,
        object: Register,
        property: String,
    },
    /// Store object property: obj.property = value
    StoreProperty {
        object: Register,
        property: String,
        value: Register,
    },
    /// Method call: dest = obj.method(args...)
    MethodCall {
        dest: Register,
        object: Register,
        method: String,
        args: Vec<Register>,
    },

    // === Class Operations ===
    /// Create a new instance: dest = new Class(args...)
    NewInstance {
        dest: Register,
        class_name: String,
        args: Vec<Register>,
    },

    // === Special Operations ===
    /// Await async operation: dest = await future
    Await {
        dest: Register,
        future: Register,
    },
    /// Type check: dest = typeof operand
    TypeOf {
        dest: Register,
        operand: Register,
    },
    /// Instance check: dest = operand instanceof type_name
    InstanceOf {
        dest: Register,
        operand: Register,
        type_name: String,
    },
    /// Throw an exception
    Throw {
        value: Register,
    },

    // === Optimization Hints ===
    /// No-op (useful for optimization passes)
    Nop,
    /// Comment/debug info (removed in production)
    Comment {
        text: String,
    },

    // === Destructuring ===
    /// Destructure array: [dest1, dest2, ...] = src
    DestructureArray {
        dests: Vec<Register>,
        src: Register,
        has_rest: bool,
        rest_dest: Option<Register>,
    },
    /// Destructure object: {key1: dest1, key2: dest2, ...} = src
    DestructureObject {
        mappings: Vec<(String, Register)>,
        src: Register,
        rest_dest: Option<Register>,
    },

    // === Increment/Decrement ===
    /// Increment: dest = operand++
    Increment {
        dest: Register,
        operand: Register,
        is_prefix: bool,
    },
    /// Decrement: dest = operand--
    Decrement {
        dest: Register,
        operand: Register,
        is_prefix: bool,
    },

    // === Template Strings ===
    /// Create template string from parts
    CreateTemplate {
        dest: Register,
        parts: Vec<TemplatePart>,
    },

    // === Pattern Matching ===
    /// Match expression with multiple arms
    Match {
        dest: Register,
        scrutinee: Register,
        arms: Vec<MatchArm>,
    },

    // === Range ===
    /// Create a range: dest = start..end
    CreateRange {
        dest: Register,
        start: Register,
        end: Register,
    },

    // === Spread ===
    /// Spread operation (used in array/object literals and calls)
    Spread {
        dest: Register,
        operand: Register,
    },

    // === Conditional ===
    /// Ternary conditional: dest = condition ? then_val : else_val
    Conditional {
        dest: Register,
        condition: Register,
        then_val: Register,
        else_val: Register,
    },

    // === Null Operations ===
    /// Null coalescing: dest = left ?? right
    NullCoalesce {
        dest: Register,
        left: Register,
        right: Register,
    },
    /// Optional chaining: dest = obj?.property
    OptionalChain {
        dest: Register,
        object: Register,
        property: String,
    },

    // === Environment Management ===
    /// Push a new environment scope
    PushScope,
    /// Pop the current environment scope
    PopScope,

    // === Loop Control ===
    /// Break from loop (handled specially by loop constructs)
    Break,
    /// Continue in loop (handled specially by loop constructs)
    Continue,

    // === Try-Catch ===
    /// Try-catch block: try { body } catch(err) { handler }
    TryCatch {
        try_body: Vec<Instruction>,
        catch_handler: Option<(String, Vec<Instruction>)>, // (error_var_name, handler_body)
        finally_body: Option<Vec<Instruction>>,
    },

    // === Class Operations ===
    /// Create a class definition
    CreateClass {
        name: String,
        constructor: Option<(Vec<String>, Vec<Instruction>)>, // (params, body)
        methods: Vec<(String, Vec<String>, Vec<Instruction>, bool)>, // (name, params, body, is_async)
        properties: Vec<(String, Register)>,
    },

    // === This and Super ===
    /// Load 'this' value
    LoadThis { dest: Register },
    /// Call super method
    CallSuper {
        dest: Register,
        method: String,
        args: Vec<Register>,
    },

    // === Advanced Spread ===
    /// Spread in array context
    SpreadArray {
        dest: Register,
        operand: Register,
    },
    /// Spread in object context
    SpreadObject {
        dest: Register,
        operand: Register,
    },
    /// Spread in call arguments
    SpreadCall {
        dest: Register,
        operand: Register,
    },

    // === Module Operations ===
    /// Import statement
    Import {
        dest: Register,
        path: String,
        items: Vec<String>, // specific imports or empty for default
    },
    /// Export statement
    Export {
        name: String,
        value: Register,
    },

    // === For-In/For-Of ===
    /// For-in loop: for (var in obj)
    ForIn {
        variable: String,
        object: Register,
        body: Vec<Instruction>,
    },
    /// For-of loop: for (var of iterable)
    ForOf {
        variable: String,
        iterable: Register,
        body: Vec<Instruction>,
    },

    // === Assignment Operations ===
    /// Compound assignment: dest +=/−=/etc src
    CompoundAssign {
        dest: Register,
        src: Register,
        op: BinaryOperator,
    },

    // === Iterator Protocol ===
    /// Get iterator from iterable
    GetIterator {
        dest: Register,
        iterable: Register,
    },
    /// Call next() on iterator
    IteratorNext {
        dest: Register,
        iterator: Register,
    },

    // === Generators ===
    /// Yield a value (in generator function)
    Yield {
        value: Option<Register>,
    },
    /// Create a generator function
    CreateGenerator {
        dest: Register,
        name: String,
        params: Vec<String>,
        body: Vec<Instruction>,
    },

    // === Promise/Async ===
    /// Catch promise error
    Catch {
        dest: Register,
        promise: Register,
        handler: Vec<Instruction>,
    },
    /// Finally block for promise
    Finally {
        block: Vec<Instruction>,
    },

    // === Tagged Template ===
    /// Tagged template string
    TaggedTemplate {
        dest: Register,
        tag: Register,
        parts: Vec<String>,
        expressions: Vec<Register>,
    },

    // === Null Assertion ===
    /// Assert value is not null (!. operator)
    NullAssert {
        dest: Register,
        value: Register,
    },

    // === Delete Operator ===
    /// Delete property from object
    DeleteProperty {
        dest: Register,
        object: Register,
        property: String,
    },

    // === In Operator ===
    /// Check if property exists in object
    In {
        dest: Register,
        property: String,
        object: Register,
    },
}

/// Template string part (literal or expression)
#[derive(Debug, Clone)]
pub enum TemplatePart {
    String(String),
    Expr(Register),
}

/// Match arm for pattern matching
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub guard: Option<Register>,
    pub body: Vec<Instruction>,
}

/// Pattern for matching
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
    /// Returns true if this instruction is a control flow instruction
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

    /// Returns true if this instruction has side effects
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

    /// Returns the destination register if this instruction writes to one
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

    /// Returns all source registers used by this instruction
    pub fn source_registers(&self) -> Vec<&Register> {
        let mut sources = Vec::new();
        match self {
            // Instructions that don't use any registers
            Instruction::LoadConst { .. } | Instruction::Declare { .. } | Instruction::Label { .. }
            | Instruction::Jump { .. } | Instruction::Load { .. } | Instruction::Nop
            | Instruction::Comment { .. } | Instruction::PushScope | Instruction::PopScope
            | Instruction::Break | Instruction::Continue | Instruction::LoadThis { .. }
            | Instruction::Import { .. } | Instruction::CreateGenerator { .. }
            | Instruction::Finally { .. } | Instruction::TryCatch { .. } | Instruction::CreateClass { .. } => {},

            // Instructions that use value register
            Instruction::Return { value } => {
                if let Some(val) = value {
                    sources.push(val);
                }
            }

            // Basic move and store
            Instruction::Move { src, .. } => sources.push(src),
            Instruction::Store { src, .. } => sources.push(src),

            // Binary operations
            Instruction::BinaryOp { left, right, .. } => {
                sources.push(left);
                sources.push(right);
            }

            // Unary operations
            Instruction::UnaryOp { operand, .. } => sources.push(operand),

            // Conditional jumps
            Instruction::JumpIfFalse { condition, .. }
            | Instruction::JumpIfTrue { condition, .. } => sources.push(condition),

            // Function calls
            Instruction::Call { callee, args, .. } => {
                sources.push(callee);
                sources.extend(args.iter());
            }

            // Array operations
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

            // Object operations
            Instruction::CreateObject { properties, .. } => {
                sources.extend(properties.iter().map(|(_, reg)| reg));
            }
            Instruction::LoadProperty { object, .. } => sources.push(object),
            Instruction::StoreProperty {
                object, value, ..
            } => {
                sources.push(object);
                sources.push(value);
            }
            Instruction::DeleteProperty { object, .. } => sources.push(object),
            Instruction::In { object, .. } => sources.push(object),

            // Method calls
            Instruction::MethodCall {
                object, args, ..
            } => {
                sources.push(object);
                sources.extend(args.iter());
            }

            // Instance operations
            Instruction::NewInstance { args, .. } => sources.extend(args.iter()),

            // Async operations
            Instruction::Await { future, .. } => sources.push(future),

            // Type operations
            Instruction::TypeOf { operand, .. } | Instruction::InstanceOf { operand, .. } => {
                sources.push(operand)
            }

            // Exception handling
            Instruction::Throw { value } => sources.push(value),
            Instruction::Catch { promise, .. } => sources.push(promise),

            // Destructuring
            Instruction::DestructureArray { src, .. }
            | Instruction::DestructureObject { src, .. } => sources.push(src),

            // Increment/Decrement
            Instruction::Increment { operand, .. } | Instruction::Decrement { operand, .. } => {
                sources.push(operand)
            }

            // Template operations
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

            // Match expression
            Instruction::Match { scrutinee, arms, .. } => {
                sources.push(scrutinee);
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        sources.push(guard);
                    }
                }
            }

            // Range operations
            Instruction::CreateRange { start, end, .. } => {
                sources.push(start);
                sources.push(end);
            }

            // Spread operations
            Instruction::Spread { operand, .. }
            | Instruction::SpreadArray { operand, .. }
            | Instruction::SpreadObject { operand, .. }
            | Instruction::SpreadCall { operand, .. } => sources.push(operand),

            // Conditional operation
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

            // Null operations
            Instruction::NullCoalesce { left, right, .. } => {
                sources.push(left);
                sources.push(right);
            }
            Instruction::NullAssert { value, .. } => sources.push(value),
            Instruction::OptionalChain { object, .. } => sources.push(object),

            // Class operations
            Instruction::CallSuper { args, .. } => sources.extend(args.iter()),

            // Module operations
            Instruction::Export { value, .. } => sources.push(value),

            // Loop operations
            Instruction::ForIn { object, .. } => sources.push(object),
            Instruction::ForOf { iterable, .. } => sources.push(iterable),

            // Assignment operations
            Instruction::CompoundAssign { dest, src, .. } => {
                sources.push(dest);
                sources.push(src);
            }

            // Iterator operations
            Instruction::GetIterator { iterable, .. } => sources.push(iterable),
            Instruction::IteratorNext { iterator, .. } => sources.push(iterator),

            // Generator operations
            Instruction::Yield { value } => {
                if let Some(val) = value {
                    sources.push(val);
                }
            }

            // Function definition
            Instruction::CreateFunction { .. } => {},
        }
        sources
    }
}

/// IR Program - sequence of instructions with metadata
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

    /// Add an instruction to the program
    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Add a label at the current position
    pub fn emit_label(&mut self, name: String) {
        let position = self.instructions.len();
        self.labels.insert(name.clone(), position);
        self.emit(Instruction::Label { name });
    }

    /// Get the current instruction position
    pub fn current_position(&self) -> usize {
        self.instructions.len()
    }

    /// Add a constant to the pool and return its index
    pub fn add_constant(&mut self, value: RuntimeValue) -> usize {
        // Check if constant already exists
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
