/// Logical operations module
/// Centralizes logical operations: &&, ||
/// Note: These support short-circuit evaluation, but the short-circuit
/// logic is handled by the interpreter, not here
use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;

/// Helper function to determine if a value is truthy
fn is_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Bool(b) => b.value,
        RuntimeValue::Null(_) => false,
        RuntimeValue::Int(i) => i.value != 0,
        RuntimeValue::Float(f) => f.value != 0.0,
        RuntimeValue::Str(s) => !s.value.is_empty(),
        _ => true,
    }
}

/// Logical AND operation: left && right
/// Returns left if falsy, otherwise returns right
/// (Evaluates to the actual value, not necessarily a bool)
pub fn and(
    left: RuntimeValue,
    right: RuntimeValue,
    _position: Position,
    _file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    if is_truthy(&left) {
        Ok(right)
    } else {
        Ok(left)
    }
}

/// Logical OR operation: left || right
/// Returns left if truthy, otherwise returns right
/// (Evaluates to the actual value, not necessarily a bool)
pub fn or(
    left: RuntimeValue,
    right: RuntimeValue,
    _position: Position,
    _file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    if is_truthy(&left) {
        Ok(left)
    } else {
        Ok(right)
    }
}
