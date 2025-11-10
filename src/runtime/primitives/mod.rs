//! Primitive operations organized by context
//!
//! This module provides a lazy-loading system for primitives.
//! Primitives are organized by their operational context and are only
//! loaded when requested.

#[macro_use]
pub mod macros;

pub mod array;
pub mod contexts;
pub mod http;
pub mod io;
pub mod json;
pub mod math;
pub mod registry;
pub mod string;
pub mod system;
pub mod time;

pub use contexts::PrimitiveContext;
pub use registry::LazyPrimitiveRegistry;

// Re-export registration functions
pub use array::register_array_primitives;
pub use http::register_http_primitives;
pub use io::register_io_primitives;
pub use json::register_json_primitives;
pub use math::register_math_primitives;
pub use string::register_string_primitives;
pub use system::register_system_primitives;
pub use time::register_time_primitives;
