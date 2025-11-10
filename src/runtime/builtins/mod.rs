//! Built-in functions and types for Raccoon runtime
//!
//! Organized into logical modules:
//! - `global`: Global functions (print, println, eprint, input, len)
//! - `primitives`: Primitive types (int, str, float, bool)
//! - `objects`: Built-in objects (Future, Object, Type)
//! - `array`: Array functional methods
//! - `builders`: Utilities for Future collection and TypeMethodBuilder
//! - `macros`: Macros for function type definitions

pub mod array;
pub mod builders;
pub mod global;
pub mod macros;
pub mod objects;
pub mod primitives;

pub use builders::{
    check_arg_count, check_arg_count_range, collect_futures, error_future, extract_array,
    extract_int, extract_map, extract_string, resolved_future, validate_futures_array,
    FutureCollectionStrategy, TypeMethodBuilder,
};

use crate::runtime::Environment;

/// Sets up all built-in functions, types, and methods in the environment
pub fn setup_builtins(env: &mut Environment) {
    global::register(env);
    primitives::register(env);
    objects::register(env);
    array::register(env);
}
