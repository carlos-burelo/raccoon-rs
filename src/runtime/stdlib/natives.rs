use crate::runtime::Environment;

#[deprecated(note = "native_* functions should not be exposed as globals")]
pub fn register_all_stdlib_natives(_env: &mut Environment) {}
