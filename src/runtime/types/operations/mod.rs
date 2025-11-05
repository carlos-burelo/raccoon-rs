/// Operations module - Centralized type system operations
/// Consolidates all binary operations, casting, type validation, and compatibility checking
///
/// This module eliminates duplicate logic that was spread across:
/// - src/interpreter/operators.rs
/// - src/type_system/checker.rs
/// - src/ast/types.rs
/// - src/runtime/native.rs

pub mod arithmetic;
pub mod bitwise;
pub mod comparison;
pub mod logical;
pub mod casting;
pub mod compatibility;
pub mod type_narrowing;
pub mod conversion;

use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::{BinaryOperator, Position};

/// Applies a binary operation to two runtime values
/// This is the single entry point for all binary operations,
/// replacing the scattered logic in operators.rs
pub async fn apply_binary_operation(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BinaryOperator,
    position: Position,
    file: &Option<String>,
) -> Result<RuntimeValue, RaccoonError> {
    match operator {
        // Arithmetic operations
        BinaryOperator::Add => arithmetic::add(left, right, position, file),
        BinaryOperator::Subtract => arithmetic::subtract(left, right, position, file),
        BinaryOperator::Multiply => arithmetic::multiply(left, right, position, file),
        BinaryOperator::Divide => arithmetic::divide(left, right, position, file),
        BinaryOperator::Modulo => arithmetic::modulo(left, right, position, file),
        BinaryOperator::Exponent => arithmetic::exponent(left, right, position, file),

        // Bitwise operations
        BinaryOperator::BitwiseAnd => bitwise::bitwise_and(left, right, position, file),
        BinaryOperator::BitwiseOr => bitwise::bitwise_or(left, right, position, file),
        BinaryOperator::BitwiseXor => bitwise::bitwise_xor(left, right, position, file),
        BinaryOperator::LeftShift => bitwise::left_shift(left, right, position, file),
        BinaryOperator::RightShift => bitwise::right_shift(left, right, position, file),
        BinaryOperator::UnsignedRightShift => {
            bitwise::unsigned_right_shift(left, right, position, file)
        }

        // Comparison operations
        BinaryOperator::Equal => comparison::equal(left, right, position, file),
        BinaryOperator::NotEqual => comparison::not_equal(left, right, position, file),
        BinaryOperator::LessThan => comparison::less_than(left, right, position, file),
        BinaryOperator::LessEqual => comparison::less_or_equal(left, right, position, file),
        BinaryOperator::GreaterThan => comparison::greater_than(left, right, position, file),
        BinaryOperator::GreaterEqual => {
            comparison::greater_or_equal(left, right, position, file)
        }

        // Logical operations
        BinaryOperator::And => logical::and(left, right, position, file),
        BinaryOperator::Or => logical::or(left, right, position, file),

        // Special operations not handled here (Range, NullCoalesce)
        // These are handled in src/interpreter/operators.rs
        BinaryOperator::Range | BinaryOperator::NullCoalesce => {
            Err(RaccoonError::new(
                "Range and NullCoalesce operations are handled in interpreter".to_string(),
                position,
                file.clone(),
            ))
        }
    }
}
