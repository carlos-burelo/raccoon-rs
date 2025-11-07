/// Stdlib native functions - DEPRECATED
/// These functions should NOT be exposed directly to users with native_* prefix.
/// All functionality should be available through:
/// 1. Instance methods on primitives (str.toUpper(), arr.push(), etc.)
/// 2. Standard library modules (std:math, std:json, etc.)
///
/// This file is kept for backward compatibility but does nothing.

use crate::runtime::Environment;

#[deprecated(note = "native_* functions should not be exposed as globals")]
pub fn register_all_stdlib_natives(_env: &mut Environment) {
    // INTENTIONALLY EMPTY
    // All native functions are now accessed through:
    // - Instance methods: str.toUpper(), arr.push(), etc.
    // - Module namespaces: Math.sqrt(), JSON.parse(), etc.
    //
    // The implementations remain in:
    // - src/runtime/types/primitives/*.rs (for instance methods)
    // - src/runtime/natives/*.rs (for module functions)
}
