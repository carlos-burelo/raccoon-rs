// Integer types module
pub mod bigint;
pub mod signed;
pub mod unsigned;

// Re-export all integer types
pub use bigint::BigIntType;
pub use signed::{I16Type, I32Type, I64Type, I8Type, IntType};
pub use unsigned::{U16Type, U32Type, U64Type, U8Type};
