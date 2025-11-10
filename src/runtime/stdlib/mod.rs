//! Standard library system for Raccoon runtime
//!
//! Organized into logical modules:
//! - `loader`: Module loading and caching system for stdlib .rcc files
//! - `natives`: Native function registrations (DEPRECATED - kept for compatibility)
//! - `wrappers`: Wrapper functions that expose native module functions to stdlib modules

pub mod loader;
pub mod natives;
pub mod wrappers;

pub use loader::StdLibLoader;
pub use natives::register_all_stdlib_natives;
pub use wrappers::register_stdlib_wrappers;
