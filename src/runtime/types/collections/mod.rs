// Collections module - generic/parameterized container types
pub mod list;
pub mod list_refactored;
pub mod map;
pub mod map_refactored;
pub mod optional;
pub mod range;
pub mod set;
pub mod tuple;

// Re-export for convenience
pub use list::ListType;
pub use map::MapType;
pub use optional::OptionalType;
pub use range::RangeType;
pub use set::SetType;
pub use tuple::TupleType;
