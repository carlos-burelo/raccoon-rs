pub mod array;
pub mod ffi;
pub mod http;
/// Native function modules (LEGACY)
///
/// NOTE: This module is deprecated in favor of the plugin-based system.
/// See src/runtime/plugin_system.rs and src/runtime/native_bridge_v2.rs
///
/// This module organizes native functions by category. Each submodule provides
/// a register() function for the legacy registration system.
pub mod io;
pub mod json;
pub mod math;
pub mod output;
pub mod random;
pub mod string;
pub mod time;

