// Primitives module - scalar/singleton types with methods
pub mod bool;
pub mod bool_refactored;
pub mod char;
pub mod floats;
pub mod integers;
pub mod null;
pub mod numeric_trait;
pub mod string;
pub mod string_refactored;
pub mod unit;

// Re-export for convenience
pub use self::bool::BoolType;
pub use self::char::CharType;
pub use self::floats::{DecimalType, Float32Type, Float64Type, FloatType};
pub use self::integers::{
    BigIntType, I16Type, I32Type, I64Type, I8Type, IntType, U16Type, U32Type, U64Type, U8Type,
};
pub use self::null::NullType;
pub use self::numeric_trait::{
    I16Handler, I32Handler, I64Handler, I8Handler, U16Handler, U32Handler, U64Handler, U8Handler,
};
pub use self::string::StrType;
pub use self::unit::UnitType;
