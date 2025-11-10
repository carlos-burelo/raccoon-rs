pub mod arithmetic;
pub mod bitwise;
pub mod casting;
pub mod comparison;
pub mod compatibility;
pub mod conversion;
pub mod logical;
pub mod type_narrowing;

use crate::error::RaccoonError;
use crate::runtime::{CallStack, RuntimeValue};
use crate::tokens::{BinaryOperator, Position};

pub async fn apply_binary_operation(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BinaryOperator,
    position: Position,
    file: &Option<String>,
    call_stack: &CallStack,
) -> Result<RuntimeValue, RaccoonError> {
    match operator {
        BinaryOperator::Add => arithmetic::add(left, right, position, file),
        BinaryOperator::Subtract => arithmetic::subtract(left, right, position, file),
        BinaryOperator::Multiply => arithmetic::multiply(left, right, position, file),
        BinaryOperator::Divide => arithmetic::divide(left, right, position, file, call_stack),
        BinaryOperator::Modulo => arithmetic::modulo(left, right, position, file),
        BinaryOperator::Exponent => arithmetic::exponent(left, right, position, file),

        BinaryOperator::BitwiseAnd => bitwise::bitwise_and(left, right, position, file),
        BinaryOperator::BitwiseOr => bitwise::bitwise_or(left, right, position, file),
        BinaryOperator::BitwiseXor => bitwise::bitwise_xor(left, right, position, file),
        BinaryOperator::LeftShift => bitwise::left_shift(left, right, position, file),
        BinaryOperator::RightShift => bitwise::right_shift(left, right, position, file),
        BinaryOperator::UnsignedRightShift => {
            bitwise::unsigned_right_shift(left, right, position, file)
        }

        BinaryOperator::Equal => comparison::equal(left, right, position, file),
        BinaryOperator::NotEqual => comparison::not_equal(left, right, position, file),
        BinaryOperator::LessThan => comparison::less_than(left, right, position, file),
        BinaryOperator::LessEqual => comparison::less_or_equal(left, right, position, file),
        BinaryOperator::GreaterThan => comparison::greater_than(left, right, position, file),
        BinaryOperator::GreaterEqual => comparison::greater_or_equal(left, right, position, file),

        BinaryOperator::And => logical::and(left, right, position, file),
        BinaryOperator::Or => logical::or(left, right, position, file),

        BinaryOperator::Range | BinaryOperator::NullCoalesce => Err(RaccoonError::new(
            "Range and NullCoalesce operations are handled in interpreter".to_string(),
            position,
            file.clone(),
        )),
    }
}
