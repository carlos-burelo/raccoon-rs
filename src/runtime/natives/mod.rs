#[macro_use]
pub mod macros; // Macro system for defining native primitives

pub mod array;
pub mod http;
pub mod io;
pub mod json;
pub mod math;
pub mod output;
pub mod primitives; // Core primitives for std:runtime module
pub mod random;
pub mod string;
pub mod time;

// Export registration functions
pub use array::register_array_module;
pub use http::register_http_module;
pub use io::register_io_module;
pub use json::register_json_module;
pub use math::register_math_module;
pub use primitives::register_core_primitives;
pub use random::register_random_module;
pub use string::register_string_module;
pub use time::register_time_module;
