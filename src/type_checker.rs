use crate::{
    ast::types::*,
    error::RaccoonError,
    symbol_table::SymbolItem,
    tokens::{AccessModifier, BinaryOperator, Position, TokenType, UnaryOperator},
};
use std::collections::HashMap;

pub struct TypeChecker {
    pub file: Option<String>,
    pub current_function: Option<SymbolItem>,
    pub current_class: Option<SymbolItem>,
    pub in_loop: bool,
    pub in_async_function: bool,
}

impl TypeChecker {
    pub fn new(file: Option<String>) -> Self {
        Self {
            file,
            current_function: None,
            current_class: None,
            in_loop: false,
            in_async_function: false,
        }
    }

    pub fn infer_binary_type(
        &self,
        operator: BinaryOperator,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<Type, RaccoonError> {
        match operator {
            BinaryOperator::Range => {
                if self.is_integer_type(left_type) && self.is_integer_type(right_type) {
                    return Ok(Type::List(Box::new(ListType {
                        element_type: left_type.clone(),
                    })));
                }
                return Err(RaccoonError::new(
                    "Range operator requires integer operands",
                    (0, 0),
                    self.file.clone(),
                ));
            }

            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Modulo => {
                if operator == BinaryOperator::Add
                    && (left_type.kind() == TypeKind::Str || right_type.kind() == TypeKind::Str)
                {
                    return Ok(PrimitiveType::str());
                }
                return Ok(self.get_wider_numeric_type(left_type, right_type));
            }

            BinaryOperator::Divide => {
                if matches!(
                    left_type.kind(),
                    TypeKind::Decimal | TypeKind::F64 | TypeKind::I64 | TypeKind::U64
                ) || matches!(
                    right_type.kind(),
                    TypeKind::Decimal | TypeKind::F64 | TypeKind::I64 | TypeKind::U64
                ) {
                    return Ok(Type::Primitive(PrimitiveType::new(TypeKind::F64, "f64")));
                }

                if matches!(left_type.kind(), TypeKind::F32)
                    || matches!(right_type.kind(), TypeKind::F32)
                {
                    return Ok(Type::Primitive(PrimitiveType::new(TypeKind::F32, "f32")));
                }

                return Ok(PrimitiveType::float());
            }

            BinaryOperator::Exponent => Ok(self.get_wider_numeric_type(left_type, right_type)),

            BinaryOperator::BitwiseAnd
            | BinaryOperator::BitwiseOr
            | BinaryOperator::BitwiseXor
            | BinaryOperator::LeftShift
            | BinaryOperator::RightShift
            | BinaryOperator::UnsignedRightShift => {
                Ok(self.get_wider_integer_type(left_type, right_type))
            }

            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
            | BinaryOperator::And
            | BinaryOperator::Or => Ok(PrimitiveType::bool()),

            _ => Err(RaccoonError::new(
                format!(
                    "Cannot apply operator {:?} to types {:?} and {:?}",
                    operator, left_type, right_type
                ),
                (0, 0),
                self.file.clone(),
            )),
        }
    }

    fn get_wider_numeric_type(&self, left: &Type, right: &Type) -> Type {
        let type_priority = HashMap::from([
            (TypeKind::Decimal, 13),
            (TypeKind::F64, 12),
            (TypeKind::F32, 11),
            (TypeKind::I64, 10),
            (TypeKind::U64, 10),
            (TypeKind::I32, 8),
            (TypeKind::U32, 8),
            (TypeKind::I16, 6),
            (TypeKind::U16, 6),
            (TypeKind::I8, 4),
            (TypeKind::U8, 4),
            (TypeKind::Float, 2),
            (TypeKind::Int, 1),
        ]);

        let left_priority = type_priority.get(&left.kind()).unwrap_or(&0);
        let right_priority = type_priority.get(&right.kind()).unwrap_or(&0);

        if left_priority >= right_priority {
            left.clone()
        } else {
            right.clone()
        }
    }

    fn get_wider_integer_type(&self, left: &Type, right: &Type) -> Type {
        let type_priority = HashMap::from([
            (TypeKind::I64, 9),
            (TypeKind::U64, 9),
            (TypeKind::I32, 7),
            (TypeKind::U32, 7),
            (TypeKind::I16, 5),
            (TypeKind::U16, 5),
            (TypeKind::I8, 3),
            (TypeKind::U8, 3),
            (TypeKind::Int, 1),
        ]);

        let left_priority = type_priority.get(&left.kind()).unwrap_or(&0);
        let right_priority = type_priority.get(&right.kind()).unwrap_or(&0);

        if left_priority >= right_priority {
            left.clone()
        } else {
            right.clone()
        }
    }

    pub fn are_types_compatible(
        &self,
        left: &Type,
        right: &Type,
        operator: BinaryOperator,
    ) -> bool {
        if matches!(left.kind(), TypeKind::Any) || matches!(right.kind(), TypeKind::Any) {
            return true;
        }

        if matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual) {
            if matches!(left.kind(), TypeKind::Null) || matches!(right.kind(), TypeKind::Null) {
                return true;
            }
        }

        if operator == BinaryOperator::Add {
            if matches!(left.kind(), TypeKind::Str) || matches!(right.kind(), TypeKind::Str) {
                return true;
            }
        }

        let numeric_ops = [
            BinaryOperator::Add,
            BinaryOperator::Subtract,
            BinaryOperator::Multiply,
            BinaryOperator::Divide,
            BinaryOperator::Modulo,
            BinaryOperator::Exponent,
        ];

        if numeric_ops.contains(&operator) {
            return self.is_numeric_type(left) && self.is_numeric_type(right);
        }

        let bitwise_ops = [
            BinaryOperator::BitwiseAnd,
            BinaryOperator::BitwiseOr,
            BinaryOperator::BitwiseXor,
            BinaryOperator::LeftShift,
            BinaryOperator::RightShift,
            BinaryOperator::UnsignedRightShift,
        ];

        if bitwise_ops.contains(&operator) {
            return self.is_integer_type(left) && self.is_integer_type(right);
        }

        let comparison_ops = [
            BinaryOperator::LessThan,
            BinaryOperator::GreaterThan,
            BinaryOperator::LessEqual,
            BinaryOperator::GreaterEqual,
        ];

        if comparison_ops.contains(&operator) {
            if self.is_numeric_type(left) && self.is_numeric_type(right) {
                return true;
            }
            return left.equals(right);
        }

        if matches!(operator, BinaryOperator::And | BinaryOperator::Or) {
            return matches!(left.kind(), TypeKind::Bool) && matches!(right.kind(), TypeKind::Bool);
        }

        if matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual) {
            return true;
        }

        false
    }

    pub fn is_numeric_type(&self, type_: &Type) -> bool {
        matches!(
            type_.kind(),
            TypeKind::Int
                | TypeKind::Float
                | TypeKind::Decimal
                | TypeKind::I8
                | TypeKind::I16
                | TypeKind::I32
                | TypeKind::I64
                | TypeKind::U8
                | TypeKind::U16
                | TypeKind::U32
                | TypeKind::U64
                | TypeKind::F32
                | TypeKind::F64
        )
    }

    pub fn is_integer_type(&self, type_: &Type) -> bool {
        matches!(
            type_.kind(),
            TypeKind::Int
                | TypeKind::I8
                | TypeKind::I16
                | TypeKind::I32
                | TypeKind::I64
                | TypeKind::U8
                | TypeKind::U16
                | TypeKind::U32
                | TypeKind::U64
        )
    }

    pub fn check_member_access(
        &self,
        class_type: &ClassType,
        member_name: &str,
        access_modifier: &AccessModifier,
        position: Position,
    ) -> Result<(), RaccoonError> {
        match access_modifier {
            AccessModifier::Public => Ok(()),

            AccessModifier::Private => {
                if let Some(ref current_class) = self.current_class {
                    if let Type::Class(ref current_class_type) = current_class.symbol_type {
                        if current_class_type.name == class_type.name {
                            return Ok(());
                        }
                    }
                }
                Err(RaccoonError::new(
                    format!(
                        "Cannot access private member '{}' of class '{}'",
                        member_name, class_type.name
                    ),
                    position,
                    self.file.clone(),
                ))
            }

            AccessModifier::Protected => {
                if self.current_class.is_none() {
                    return Err(RaccoonError::new(
                        format!(
                            "Cannot access protected member '{}' outside of class",
                            member_name
                        ),
                        position,
                        self.file.clone(),
                    ));
                }

                if let Some(ref current_class) = self.current_class {
                    if let Type::Class(ref current_class_type) = current_class.symbol_type {
                        if current_class_type.name == class_type.name {
                            return Ok(());
                        }
                    }
                }

                Err(RaccoonError::new(
                    format!(
                        "Cannot access protected member '{}' of class '{}'",
                        member_name, class_type.name
                    ),
                    position,
                    self.file.clone(),
                ))
            }
        }
    }

    pub fn infer_unary_type(
        &self,
        operator: UnaryOperator,
        operand_type: &Type,
        position: Position,
    ) -> Result<Type, RaccoonError> {
        match operator {
            UnaryOperator::Negate => {
                if self.is_numeric_type(operand_type) {
                    Ok(operand_type.clone())
                } else {
                    Err(RaccoonError::new(
                        format!("Cannot negate type '{:?}'", operand_type),
                        position,
                        self.file.clone(),
                    ))
                }
            }

            UnaryOperator::Not => {
                if matches!(operand_type.kind(), TypeKind::Bool) {
                    Ok(PrimitiveType::bool())
                } else {
                    Err(RaccoonError::new(
                        format!("Cannot apply logical not to type '{:?}'", operand_type),
                        position,
                        self.file.clone(),
                    ))
                }
            }

            UnaryOperator::BitwiseNot => {
                if self.is_integer_type(operand_type) {
                    Ok(operand_type.clone())
                } else {
                    Err(RaccoonError::new(
                        format!(
                            "Cannot apply bitwise not to type '{:?}' (requires integer type)",
                            operand_type
                        ),
                        position,
                        self.file.clone(),
                    ))
                }
            }
        }
    }

    pub fn validate_assignment(
        &self,
        target_type: &Type,
        value_type: &Type,
        operator: TokenType,
        position: Position,
    ) -> Result<Type, RaccoonError> {
        let final_value_type = if operator != TokenType::Assign {
            let binary_op = self.token_type_to_binary_op(operator, position)?;

            if !self.are_types_compatible(target_type, value_type, binary_op) {
                return Err(RaccoonError::new(
                    format!(
                        "Cannot apply operator {:?} to types '{:?}' and '{:?}'",
                        binary_op, target_type, value_type
                    ),
                    position,
                    self.file.clone(),
                ));
            }

            self.infer_binary_type(binary_op, target_type, value_type)?
        } else {
            value_type.clone()
        };

        if !final_value_type.is_assignable_to(target_type) {
            return Err(RaccoonError::new(
                format!(
                    "Cannot assign type '{:?}' to '{:?}'",
                    final_value_type, target_type
                ),
                position,
                self.file.clone(),
            ));
        }

        Ok(final_value_type)
    }

    fn token_type_to_binary_op(
        &self,
        token_type: TokenType,
        position: Position,
    ) -> Result<BinaryOperator, RaccoonError> {
        match token_type {
            TokenType::PlusAssign => Ok(BinaryOperator::Add),
            TokenType::MinusAssign => Ok(BinaryOperator::Subtract),
            TokenType::MultiplyAssign => Ok(BinaryOperator::Multiply),
            TokenType::DivideAssign => Ok(BinaryOperator::Divide),
            TokenType::ModuloAssign => Ok(BinaryOperator::Modulo),
            TokenType::AmpersandAssign => Ok(BinaryOperator::BitwiseAnd),
            TokenType::BitwiseOrAssign => Ok(BinaryOperator::BitwiseOr),
            TokenType::BitwiseXorAssign => Ok(BinaryOperator::BitwiseXor),
            TokenType::LeftShiftAssign => Ok(BinaryOperator::LeftShift),
            TokenType::RightShiftAssign => Ok(BinaryOperator::RightShift),
            TokenType::UnsignedRightShiftAssign => Ok(BinaryOperator::UnsignedRightShift),
            TokenType::ExponentAssign => Ok(BinaryOperator::Exponent),
            _ => Err(RaccoonError::new(
                "Invalid compound assignment",
                position,
                self.file.clone(),
            )),
        }
    }

    pub fn validate_range_expr(
        &self,
        start_type: &Type,
        end_type: &Type,
        position: Position,
    ) -> Result<Type, RaccoonError> {
        if !matches!(start_type.kind(), TypeKind::Int) || !matches!(end_type.kind(), TypeKind::Int)
        {
            return Err(RaccoonError::new(
                "Range requires integer operands",
                position,
                self.file.clone(),
            ));
        }

        Ok(Type::List(Box::new(ListType {
            element_type: PrimitiveType::int(),
        })))
    }

    pub fn validate_index_expr(
        &self,
        object_type: &Type,
        index_type: &Type,
        position: Position,
    ) -> Result<Type, RaccoonError> {
        if !matches!(index_type.kind(), TypeKind::Int) {
            return Err(RaccoonError::new(
                format!("Index must be integer, got '{:?}'", index_type),
                position,
                self.file.clone(),
            ));
        }

        if let Type::List(list_type) = object_type {
            return Ok(list_type.element_type.clone());
        }

        if matches!(object_type.kind(), TypeKind::Str) {
            return Ok(PrimitiveType::str());
        }

        Err(RaccoonError::new(
            format!("Cannot index type '{:?}'", object_type),
            position,
            self.file.clone(),
        ))
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new(Option::None)
    }
}
