/// Operators module - Refactored to use centralized type operations
/// All binary/unary operations are now delegated to src/runtime/types/operations/
/// This module now serves as a thin wrapper for backward compatibility

use crate::error::RaccoonError;
use crate::runtime::{BoolValue, FloatValue, IntValue, ListValue, RuntimeValue, StrValue};
use crate::runtime::types::operations;
use crate::tokens::{BinaryOperator, Position, UnaryOperator};
use crate::ast::types::PrimitiveType;

/// Helper: Check if a value is truthy
/// Moved from operators for backward compatibility
pub fn is_truthy(value: &RuntimeValue) -> bool {
    match value {
        RuntimeValue::Bool(b) => b.value,
        RuntimeValue::Null(_) => false,
        RuntimeValue::Int(i) => i.value != 0,
        RuntimeValue::Float(f) => f.value != 0.0,
        RuntimeValue::Str(s) => !s.value.is_empty(),
        _ => true,
    }
}

/// Main async binary operation dispatcher (backward compatible)
/// Delegates most operations to the centralized operations module
pub async fn apply_binary_op(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BinaryOperator,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    // Operations that are delegated to the centralized module
    match operator {
        // Arithmetic operations - delegated to operations::arithmetic
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Modulo
        | BinaryOperator::Exponent
        | BinaryOperator::BitwiseAnd
        | BinaryOperator::BitwiseOr
        | BinaryOperator::BitwiseXor
        | BinaryOperator::LeftShift
        | BinaryOperator::RightShift
        | BinaryOperator::UnsignedRightShift => {
            operations::apply_binary_operation(left, right, operator, position, file).await
        }

        // Operations that need special handling in the interpreter
        BinaryOperator::Range => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                let mut elements = Vec::new();
                for i in l.value..=r.value {
                    elements.push(RuntimeValue::Int(IntValue::new(i)));
                }
                Ok(RuntimeValue::List(ListValue::new(
                    elements,
                    PrimitiveType::int(),
                )))
            }
            _ => Err(RaccoonError::new(
                "Range operator requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::NullCoalesce => {
            if matches!(left, RuntimeValue::Null(_)) {
                Ok(right)
            } else {
                Ok(left)
            }
        }

        // Unsupported operations
        _ => Err(RaccoonError::new(
            format!("Operator {:?} not supported in apply_binary_op", operator),
            position,
            file.clone(),
        )),
    }
}

/// Binary operation handler with truthiness callback (backward compatible)
/// Combines arithmetic, comparison, and logical operations
pub fn apply_binary_operation<F>(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BinaryOperator,
    position: Position,
    file: &Option<String>,
    _is_truthy_fn: F,
) -> Result<RuntimeValue, RaccoonError>
where
    F: Fn(&RuntimeValue) -> bool,
{
    match operator {
        // Arithmetic operations - implemented here for sync compatibility
        BinaryOperator::Add => match (&left, &right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value + r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value)))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 + r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value + r.value as f64)))
            }
            (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.value, r.value))))
            }
            // String concatenation with any type
            (RuntimeValue::Str(l), r) => {
                Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.value, r.to_string()))))
            }
            (l, RuntimeValue::Str(r)) => {
                Ok(RuntimeValue::Str(StrValue::new(format!("{}{}", l.to_string(), r.value))))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for addition".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::Subtract => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value - r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value)))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 - r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value - r.value as f64)))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for subtraction".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::Multiply => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value * r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value)))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 * r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value * r.value as f64)))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for multiplication".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::Divide => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                if r.value == 0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 / r.value as f64)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                if r.value == 0.0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value)))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                if r.value == 0.0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 / r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                if r.value == 0 {
                    return Err(RaccoonError::new(
                        "Division by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value / r.value as f64)))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for division".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::Modulo => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                if r.value == 0 {
                    return Err(RaccoonError::new(
                        "Modulo by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Int(IntValue::new(l.value % r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                if r.value == 0.0 {
                    return Err(RaccoonError::new(
                        "Modulo by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value % r.value)))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                if r.value == 0.0 {
                    return Err(RaccoonError::new(
                        "Modulo by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value as f64 % r.value)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                if r.value == 0 {
                    return Err(RaccoonError::new(
                        "Modulo by zero".to_string(),
                        position,
                        file.clone(),
                    ));
                }
                Ok(RuntimeValue::Float(FloatValue::new(l.value % r.value as f64)))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for modulo".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::Exponent => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                if r.value < 0 {
                    Ok(RuntimeValue::Float(FloatValue::new((l.value as f64).powf(r.value as f64))))
                } else {
                    Ok(RuntimeValue::Int(IntValue::new(l.value.pow(r.value as u32))))
                }
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value.powf(r.value))))
            }
            (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new((l.value as f64).powf(r.value))))
            }
            (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Float(FloatValue::new(l.value.powf(r.value as f64))))
            }
            _ => Err(RaccoonError::new(
                "Invalid operands for exponentiation".to_string(),
                position,
                file.clone(),
            )),
        },

        // Bitwise operations
        BinaryOperator::BitwiseAnd => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value & r.value)))
            }
            _ => Err(RaccoonError::new(
                "Bitwise AND requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::BitwiseOr => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value | r.value)))
            }
            _ => Err(RaccoonError::new(
                "Bitwise OR requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::BitwiseXor => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value ^ r.value)))
            }
            _ => Err(RaccoonError::new(
                "Bitwise XOR requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::LeftShift => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value << r.value)))
            }
            _ => Err(RaccoonError::new(
                "Left shift requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::RightShift => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new(l.value >> r.value)))
            }
            _ => Err(RaccoonError::new(
                "Right shift requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::UnsignedRightShift => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                Ok(RuntimeValue::Int(IntValue::new((l.value as u64 >> r.value) as i64)))
            }
            _ => Err(RaccoonError::new(
                "Unsigned right shift requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        }

        // Comparison operations - delegated
        BinaryOperator::Equal => {
            Ok(RuntimeValue::Bool(BoolValue::new(left.equals(&right))))
        }
        BinaryOperator::NotEqual => {
            Ok(RuntimeValue::Bool(BoolValue::new(!left.equals(&right))))
        }


        BinaryOperator::LessThan => {
            match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new((l.value as f64) < r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < (r.value as f64))))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value < r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for less than comparison".to_string(),
                    position,
                    file.clone(),
                )),
            }
        }

        BinaryOperator::LessEqual => {
            match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new((l.value as f64) <= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= (r.value as f64))))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value <= r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for less than or equal comparison".to_string(),
                    position,
                    file.clone(),
                )),
            }
        }

        BinaryOperator::GreaterThan => {
            match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new((l.value as f64) > r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > (r.value as f64))))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value > r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for greater than comparison".to_string(),
                    position,
                    file.clone(),
                )),
            }
        }

        BinaryOperator::GreaterEqual => {
            match (left, right) {
                (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                }
                (RuntimeValue::Int(l), RuntimeValue::Float(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new((l.value as f64) >= r.value)))
                }
                (RuntimeValue::Float(l), RuntimeValue::Int(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= (r.value as f64))))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Bool(BoolValue::new(l.value >= r.value)))
                }
                _ => Err(RaccoonError::new(
                    "Invalid operands for greater than or equal comparison".to_string(),
                    position,
                    file.clone(),
                )),
            }
        }

        // Logical operations
        BinaryOperator::And => {
            if !is_truthy(&left) {
                Ok(left)
            } else {
                Ok(right)
            }
        }
        BinaryOperator::Or => {
            if is_truthy(&left) {
                Ok(left)
            } else {
                Ok(right)
            }
        }

        // Special operations
        BinaryOperator::Range => match (left, right) {
            (RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
                let mut elements = Vec::new();
                for i in l.value..=r.value {
                    elements.push(RuntimeValue::Int(IntValue::new(i)));
                }
                Ok(RuntimeValue::List(ListValue::new(
                    elements,
                    PrimitiveType::int(),
                )))
            }
            _ => Err(RaccoonError::new(
                "Range operator requires integer operands".to_string(),
                position,
                file.clone(),
            )),
        },

        BinaryOperator::NullCoalesce => {
            if matches!(left, RuntimeValue::Null(_)) {
                Ok(right)
            } else {
                Ok(left)
            }
        }
    }
}

/// Unary operation handler (backward compatible)
pub fn apply_unary_operation<F>(
    operand: RuntimeValue,
    operator: UnaryOperator,
    position: Position,
    file: &Option<String>,
    is_truthy_fn: F,
) -> Result<RuntimeValue, RaccoonError>
where
    F: Fn(&RuntimeValue) -> bool,
{
    match operator {
        UnaryOperator::Negate => match operand {
            RuntimeValue::Int(v) => Ok(RuntimeValue::Int(IntValue::new(-v.value))),
            RuntimeValue::Float(v) => Ok(RuntimeValue::Float(FloatValue::new(-v.value))),
            _ => Err(RaccoonError::new(
                "Invalid operand for unary minus".to_string(),
                position,
                file.clone(),
            )),
        },
        UnaryOperator::Not => {
            Ok(RuntimeValue::Bool(BoolValue::new(!is_truthy_fn(&operand))))
        }
        UnaryOperator::BitwiseNot => match operand {
            RuntimeValue::Int(v) => Ok(RuntimeValue::Int(IntValue::new(!v.value))),
            _ => Err(RaccoonError::new(
                "Invalid operand for bitwise not".to_string(),
                position,
                file.clone(),
            )),
        },
    }
}
