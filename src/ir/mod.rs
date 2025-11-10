pub mod compiler;
pub mod instruction;
pub mod optimizer;
pub mod vm;

pub use compiler::IRCompiler;
pub use instruction::{Instruction, Register};
pub use optimizer::IROptimizer;
pub use vm::VM;
