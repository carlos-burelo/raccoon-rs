// Primitives module - scalar/singleton types with methods
pub mod integers;
pub mod floats;
pub mod bool;
pub mod string;
pub mod char;
pub mod null;
pub mod unit;

// Re-export for convenience
pub use self::bool::BoolType;
pub use self::char::CharType;
pub use self::floats::{DecimalType, Float32Type, Float64Type, FloatType};
pub use self::integers::{BigIntType, I16Type, I32Type, I64Type, I8Type, IntType, U16Type, U32Type, U64Type, U8Type};
pub use self::null::NullType;
pub use self::string::StrType;
pub use self::unit::UnitType;
