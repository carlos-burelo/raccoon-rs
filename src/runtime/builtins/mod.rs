pub mod array;
pub mod builders;
pub mod builtin_macros;
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

pub fn setup_builtins(env: &mut Environment) {
    global::register(env);
    primitives::register(env);
    objects::register(env);
    array::register(env);
}
