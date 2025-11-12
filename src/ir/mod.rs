pub mod compiler;
pub mod instruction;
pub mod ir_class;
pub mod ir_function;
pub mod optimizer;
pub mod vm;

pub use compiler::IRCompiler;
pub use instruction::{Instruction, Register};
pub use ir_class::IRClassValue;
pub use ir_function::IRFunctionValue;
pub use optimizer::IROptimizer;
pub use vm::VM;
