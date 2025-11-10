use std::collections::HashMap;

use super::declarations::Declarations;
use super::state::ParserState;
use super::utilities::Parser;
use crate::tokens::AccessModifier;
use crate::{
    ast::nodes::*,
    ast::types::Type,
    tokens::{BinaryOperator, UnaryOperator},
    RaccoonError, TokenType,
};

pub struct Expressions;

impl Expressions {
    pub fn expression(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        Self::assignment(state)
    }

    pub fn assignment(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        if state.check(&TokenType::Match) {
            state.advance();
            return Self::match_expr(state);
        }

        if state.check(&TokenType::Class) {
            state.advance();
            return Self::class_expr(state);
        }

        if state.check(&TokenType::Async) {
            let saved_pos = state.current;
            state.advance();
            if state.check(&TokenType::LeftParen) {
                if let Ok(arrow) = Self::try_parse_arrow_function(state, true) {
                    return Ok(Expr::ArrowFn(arrow));
                }
            }
            state.current = saved_pos;
        }

        if state.check(&TokenType::LeftParen) {
            let saved_pos = state.current;
            if let Ok(arrow) = Self::try_parse_arrow_function(state, false) {
                return Ok(Expr::ArrowFn(arrow));
            }
            state.current = saved_pos;
        }

        let expr = Self::conditional(state)?;

        if Parser::match_token(
            state,
            &[
                TokenType::Assign,
                TokenType::PlusAssign,
                TokenType::MinusAssign,
                TokenType::MultiplyAssign,
                TokenType::DivideAssign,
                TokenType::ModuloAssign,
                TokenType::AmpersandAssign,
                TokenType::BitwiseOrAssign,
                TokenType::BitwiseXorAssign,
                TokenType::LeftShiftAssign,
                TokenType::RightShiftAssign,
                TokenType::UnsignedRightShiftAssign,
                TokenType::ExponentAssign,
            ],
        ) {
            let operator = state.previous().unwrap().token_type;
            let value = Box::new(Self::assignment(state)?);
            let position = expr.position();

            return Ok(Expr::Assignment(Assignment {
                target: Box::new(expr),
                value,
                operator,
                position,
            }));
        }

        Ok(expr)
    }

    pub fn conditional(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let expr = Self::null_coalescing(state)?;

        if Parser::match_token(state, &[TokenType::Question]) {
            let then_expr = Box::new(Self::expression(state)?);
            Parser::consume(
                state,
                TokenType::Colon,
                "Expected ':' after then branch of conditional expression",
            )?;
            let else_expr = Box::new(Self::conditional(state)?);
            let position = expr.position();

            return Ok(Expr::Conditional(ConditionalExpr {
                condition: Box::new(expr),
                then_expr,
                else_expr,
                position,
            }));
        }

        Ok(expr)
    }

    pub fn null_coalescing(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::logical_or(state)?;

        while Parser::match_token(state, &[TokenType::QuestionQuestion]) {
            let right = Box::new(Self::logical_or(state)?);
            let position = expr.position();
            expr = Expr::NullCoalescing(NullCoalescingExpr {
                left: Box::new(expr),
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn logical_or(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::logical_and(state)?;

        while Parser::match_token(state, &[TokenType::Or]) {
            let right = Box::new(Self::logical_and(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn logical_and(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::equality(state)?;

        while Parser::match_token(state, &[TokenType::And]) {
            let right = Box::new(Self::equality(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn equality(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::comparison(state)?;

        while Parser::match_token(state, &[TokenType::Eq, TokenType::Neq]) {
            let operator = if state.previous().unwrap().token_type == TokenType::Eq {
                BinaryOperator::Equal
            } else {
                BinaryOperator::NotEqual
            };
            let right = Box::new(Self::comparison(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn comparison(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::bitwise_or(state)?;

        loop {
            if Parser::match_token(
                state,
                &[TokenType::Lt, TokenType::Gt, TokenType::Lte, TokenType::Gte],
            ) {
                let operator = match state.previous().unwrap().token_type {
                    TokenType::Lt => BinaryOperator::LessThan,
                    TokenType::Gt => BinaryOperator::GreaterThan,
                    TokenType::Lte => BinaryOperator::LessEqual,
                    TokenType::Gte => BinaryOperator::GreaterEqual,
                    _ => unreachable!(),
                };
                let right = Box::new(Self::bitwise_or(state)?);
                let position = expr.position();
                expr = Expr::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator,
                    right,
                    position,
                });
            } else if Parser::match_token(state, &[TokenType::Instanceof]) {
                let position = expr.position();
                let type_name = Parser::consume(
                    state,
                    TokenType::Identifier,
                    "Expected class name after 'instanceof'",
                )?
                .value
                .clone();
                expr = Expr::InstanceOf(InstanceOfExpr {
                    operand: Box::new(expr),
                    type_name,
                    position,
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    pub fn bitwise_or(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::bitwise_xor(state)?;

        while Parser::match_token(state, &[TokenType::BitwiseOr]) {
            let right = Box::new(Self::bitwise_xor(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseOr,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn bitwise_xor(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::bitwise_and(state)?;

        while Parser::match_token(state, &[TokenType::BitwiseXor]) {
            let right = Box::new(Self::bitwise_and(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseXor,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn bitwise_and(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::shift(state)?;

        while Parser::match_token(state, &[TokenType::Ampersand]) {
            let right = Box::new(Self::shift(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseAnd,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn shift(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::range(state)?;

        while Parser::match_token(
            state,
            &[
                TokenType::LeftShift,
                TokenType::RightShift,
                TokenType::UnsignedRightShift,
            ],
        ) {
            let operator = match state.previous().unwrap().token_type {
                TokenType::LeftShift => BinaryOperator::LeftShift,
                TokenType::RightShift => BinaryOperator::RightShift,
                TokenType::UnsignedRightShift => BinaryOperator::UnsignedRightShift,
                _ => unreachable!(),
            };
            let right = Box::new(Self::range(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn range(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let expr = Self::term(state)?;

        if Parser::match_token(state, &[TokenType::Range]) {
            let end = Box::new(Self::term(state)?);
            let position = expr.position();
            return Ok(Expr::Range(RangeExpr {
                start: Box::new(expr),
                end,
                position,
            }));
        }

        Ok(expr)
    }

    pub fn term(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::factor(state)?;

        while Parser::match_token(state, &[TokenType::Plus, TokenType::Minus]) {
            let operator = if state.previous().unwrap().token_type == TokenType::Plus {
                BinaryOperator::Add
            } else {
                BinaryOperator::Subtract
            };
            let right = Box::new(Self::factor(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn factor(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::exponent(state)?;

        while Parser::match_token(
            state,
            &[TokenType::Multiply, TokenType::Divide, TokenType::Modulo],
        ) {
            let operator = match state.previous().unwrap().token_type {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                TokenType::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = Box::new(Self::exponent(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn exponent(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::unary(state)?;

        if Parser::match_token(state, &[TokenType::Exponent]) {
            let right = Box::new(Self::exponent(state)?);
            let position = expr.position();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::Exponent,
                right,
                position,
            });
        }

        Ok(expr)
    }

    pub fn unary(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        if Parser::match_token(state, &[TokenType::Typeof]) {
            let position = state.previous().unwrap().position;
            let operand = Box::new(Self::unary(state)?);
            return Ok(Expr::TypeOf(TypeOfExpr { operand, position }));
        }

        if Parser::match_token(
            state,
            &[TokenType::Minus, TokenType::Bang, TokenType::BitwiseNot],
        ) {
            let operator = match state.previous().unwrap().token_type {
                TokenType::Minus => UnaryOperator::Negate,
                TokenType::Bang => UnaryOperator::Not,
                TokenType::BitwiseNot => UnaryOperator::BitwiseNot,
                _ => unreachable!(),
            };
            let operand = Box::new(Self::unary(state)?);
            let position = state.previous().unwrap().position;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                operand,
                position,
            }));
        }

        if Parser::match_token(state, &[TokenType::Increment, TokenType::Decrement]) {
            let operator = if state.previous().unwrap().token_type == TokenType::Increment {
                crate::ast::nodes::UpdateOperator::Increment
            } else {
                crate::ast::nodes::UpdateOperator::Decrement
            };
            let position = state.previous().unwrap().position;
            let operand = Box::new(Self::unary(state)?);
            return Ok(Expr::UnaryUpdate(UnaryUpdateExpr {
                operator,
                operand,
                is_prefix: true,
                position,
            }));
        }

        if Parser::match_token(state, &[TokenType::Await]) {
            let position = state.previous().unwrap().position;
            let expression = Box::new(Self::unary(state)?);
            return Ok(Expr::Await(AwaitExpr {
                expression,
                position,
            }));
        }

        if Parser::match_token(state, &[TokenType::New]) {
            return Self::new_expression(state);
        }

        Self::postfix(state)
    }

    pub fn postfix(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let expr = Self::call(state)?;

        if Parser::match_token(state, &[TokenType::Increment, TokenType::Decrement]) {
            let operator = if state.previous().unwrap().token_type == TokenType::Increment {
                crate::ast::nodes::UpdateOperator::Increment
            } else {
                crate::ast::nodes::UpdateOperator::Decrement
            };
            let position = state.previous().unwrap().position;
            return Ok(Expr::UnaryUpdate(UnaryUpdateExpr {
                operator,
                operand: Box::new(expr),
                is_prefix: false,
                position,
            }));
        }

        if Parser::match_token(state, &[TokenType::Bang]) {
            let position = state.previous().unwrap().position;
            return Ok(Expr::NullAssertion(NullAssertionExpr {
                operand: Box::new(expr),
                position,
            }));
        }

        Ok(expr)
    }

    pub fn new_expression(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;
        let class_name = Parser::consume(
            state,
            TokenType::Identifier,
            "Expected class name after new",
        )?
        .value
        .clone();

        let mut type_args = Vec::new();
        if Parser::match_token(state, &[TokenType::Lt]) {
            loop {
                type_args.push(crate::ast::types::PrimitiveType::any());
                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
            Parser::consume(state, TokenType::Gt, "Expected '>' after type arguments")?;
        }

        let mut args = Vec::new();
        if Parser::match_token(state, &[TokenType::LeftParen]) {
            if !state.check(&TokenType::RightParen) {
                loop {
                    args.push(Self::expression(state)?);
                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
            }
            Parser::consume(state, TokenType::RightParen, "Expected ')' after arguments")?;
        }

        Ok(Expr::New(NewExpr {
            class_name,
            type_args,
            args,
            position,
        }))
    }

    pub fn call(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let mut expr = Self::primary(state)?;

        loop {
            if state.check(&TokenType::Lt)
                && Parser::check_next(state, &[TokenType::Identifier, TokenType::LeftBracket])
            {
                let saved_pos = state.current;
                if let Ok(_) = Self::try_parse_call_type_arguments(state) {
                    if Parser::match_token(state, &[TokenType::LeftParen]) {
                        expr = Self::finish_call(state, expr)?;
                        continue;
                    }
                }

                state.current = saved_pos;
            }

            if Parser::match_token(state, &[TokenType::LeftParen]) {
                expr = Self::finish_call(state, expr)?;
            } else if Parser::match_token(state, &[TokenType::QuestionDot]) {
                let name = Parser::consume_property_name(state)?;
                let position = expr.position();
                expr = Expr::OptionalChaining(OptionalChainingExpr {
                    object: Box::new(expr),
                    property: name,
                    position,
                });
            } else if Parser::match_token(state, &[TokenType::Dot]) {
                let name = Parser::consume_property_name(state)?;
                let position = expr.position();

                if state.check(&TokenType::LeftParen) {
                    state.advance();
                    let mut args = Vec::new();
                    if !state.check(&TokenType::RightParen) {
                        loop {
                            args.push(Self::expression(state)?);
                            if !Parser::match_token(state, &[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    Parser::consume(
                        state,
                        TokenType::RightParen,
                        "Expected ')' after method arguments",
                    )?;
                    expr = Expr::MethodCall(MethodCallExpr {
                        object: Box::new(expr),
                        method: name,
                        args,
                        position,
                    });
                } else {
                    expr = Expr::Member(MemberExpr {
                        object: Box::new(expr),
                        property: name,
                        position,
                    });
                }
            } else if Parser::match_token(state, &[TokenType::LeftBracket]) {
                let index = Box::new(Self::expression(state)?);
                Parser::consume(state, TokenType::RightBracket, "Expected ']' after index")?;
                let position = expr.position();
                expr = Expr::Index(IndexExpr {
                    object: Box::new(expr),
                    index,
                    position,
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(state: &mut ParserState, callee: Expr) -> Result<Expr, RaccoonError> {
        let mut args = Vec::new();
        let mut named_args = HashMap::new();

        if !state.check(&TokenType::RightParen) {
            loop {
                if Parser::match_token(state, &[TokenType::Spread]) {
                    let position = state.previous().unwrap().position;
                    let argument = Box::new(Self::expression(state)?);
                    args.push(Expr::Spread(SpreadExpr { argument, position }));
                } else if state.check(&TokenType::Identifier)
                    && Parser::check_next(state, &[TokenType::Colon])
                {
                    let name = state.advance().unwrap().value.clone();
                    state.advance();
                    let value = Self::expression(state)?;
                    named_args.insert(name, value);
                } else {
                    if !named_args.is_empty() {
                        return Err(RaccoonError::new(
                            "Positional arguments must come before named arguments",
                            state.peek().unwrap().position,
                            state.file.clone(),
                        ));
                    }
                    args.push(Self::expression(state)?);
                }

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::RightParen, "Expected ')' after arguments")?;
        let position = callee.position();

        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            args,
            named_args,
            position,
        }))
    }

    pub fn primary(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        if Parser::match_token(state, &[TokenType::This]) {
            return Ok(Expr::This(ThisExpr {
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::Super]) {
            return Ok(Expr::Super(SuperExpr {
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::Identifier]) {
            return Ok(Expr::Identifier(Identifier {
                name: state.previous().unwrap().value.clone(),
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::IntLiteral]) {
            let token_value = &state.previous().unwrap().value;
            let clean_value = token_value.replace('_', "");
            let value = if clean_value.starts_with("0b") || clean_value.starts_with("0B") {
                i64::from_str_radix(&clean_value[2..], 2).unwrap_or(0)
            } else if clean_value.starts_with("0o") || clean_value.starts_with("0O") {
                i64::from_str_radix(&clean_value[2..], 8).unwrap_or(0)
            } else if clean_value.starts_with("0x") || clean_value.starts_with("0X") {
                i64::from_str_radix(&clean_value[2..], 16).unwrap_or(0)
            } else {
                clean_value.parse::<i64>().unwrap_or(i64::MAX)
            };
            return Ok(Expr::IntLiteral(IntLiteral {
                value,
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::BigIntLiteral]) {
            let value = state.previous().unwrap().value.clone();
            return Ok(Expr::BigIntLiteral(BigIntLiteral {
                value,
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::FloatLiteral]) {
            let value = state.previous().unwrap().value.parse::<f64>().unwrap();
            return Ok(Expr::FloatLiteral(FloatLiteral {
                value,
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::StrLiteral]) {
            return Ok(Expr::StrLiteral(StrLiteral {
                value: state.previous().unwrap().value.clone(),
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::True]) {
            return Ok(Expr::BoolLiteral(BoolLiteral {
                value: true,
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::False]) {
            return Ok(Expr::BoolLiteral(BoolLiteral {
                value: false,
                position: state.previous().unwrap().position,
            }));
        }

        if Parser::match_token(state, &[TokenType::NullLiteral]) {
            return Ok(Expr::NullLiteral(NullLiteral {
                position: state.previous().unwrap().position,
            }));
        }

        if state.check(&TokenType::LeftParen) {
            let saved_pos = state.current;

            if let Ok(arrow) = Self::try_parse_arrow_function(state, false) {
                return Ok(Expr::ArrowFn(arrow));
            }

            state.current = saved_pos;
            state.advance();
            let expr = Self::expression(state)?;
            Parser::consume(
                state,
                TokenType::RightParen,
                "Expected ')' after expression",
            )?;
            return Ok(expr);
        }

        if Parser::match_token(state, &[TokenType::LeftBracket]) {
            return Self::array_literal(state);
        }

        if Parser::match_token(state, &[TokenType::LeftBrace]) {
            return Self::object_literal(state);
        }

        if Parser::match_token(state, &[TokenType::TemplateStrStart]) {
            return Self::template_string(state);
        }

        Err(RaccoonError::new(
            "Expected expression",
            state.peek().unwrap().position,
            state.file.clone(),
        ))
    }

    pub fn array_literal(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut elements = Vec::new();

        if !state.check(&TokenType::RightBracket) {
            loop {
                if Parser::match_token(state, &[TokenType::Spread]) {
                    let position = state.previous().unwrap().position;
                    let argument = Box::new(Self::expression(state)?);
                    elements.push(Expr::Spread(SpreadExpr { argument, position }));
                } else {
                    elements.push(Self::expression(state)?);
                }

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::RightBracket, "Expected ']'")?;
        Ok(Expr::ArrayLiteral(ArrayLiteral { elements, position }))
    }

    pub fn object_literal(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut properties = Vec::new();

        if !state.check(&TokenType::RightBrace) {
            loop {
                if Parser::match_token(state, &[TokenType::Spread]) {
                    let spread_expr = Self::expression(state)?;
                    properties.push(ObjectLiteralProperty::Spread(spread_expr));
                } else {
                    let key = if state.check(&TokenType::StrLiteral) {
                        state.advance().unwrap().value.clone()
                    } else {
                        Parser::consume(
                            state,
                            TokenType::Identifier,
                            "Expected property name or string literal",
                        )?
                        .value
                        .clone()
                    };

                    if state.check(&TokenType::Comma) || state.check(&TokenType::RightBrace) {
                        let value = Expr::Identifier(Identifier {
                            name: key.clone(),
                            position: state.previous().unwrap().position,
                        });
                        properties.push(ObjectLiteralProperty::KeyValue { key, value });
                    } else {
                        Parser::consume(
                            state,
                            TokenType::Colon,
                            "Expected ':' after property name",
                        )?;
                        let value = Self::expression(state)?;
                        properties.push(ObjectLiteralProperty::KeyValue { key, value });
                    }
                }

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::RightBrace, "Expected '}'")?;
        Ok(Expr::ObjectLiteral(ObjectLiteral {
            properties,
            position,
        }))
    }

    pub fn template_string(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut parts = Vec::new();

        while !state.check(&TokenType::TemplateStrEnd) {
            if Parser::match_token(state, &[TokenType::TemplateStrPart]) {
                parts.push(TemplateStrPart::String(StrLiteral {
                    value: state.previous().unwrap().value.clone(),
                    position: state.previous().unwrap().position,
                }));
            } else if Parser::match_token(state, &[TokenType::TemplateInterpolationStart]) {
                let expr = Self::expression(state)?;
                Parser::consume(
                    state,
                    TokenType::TemplateInterpolationEnd,
                    "Expected '}' after template interpolation",
                )?;
                parts.push(TemplateStrPart::Expr(expr));
            } else {
                return Err(RaccoonError::new(
                    "Expected template string part or interpolation",
                    state.peek().unwrap().position,
                    state.file.clone(),
                ));
            }
        }

        Parser::consume(
            state,
            TokenType::TemplateStrEnd,
            "Expected end of template string",
        )?;
        Ok(Expr::TemplateStr(TemplateStrExpr { parts, position }))
    }

    pub fn try_parse_arrow_function(
        state: &mut ParserState,
        is_async: bool,
    ) -> Result<ArrowFnExpr, RaccoonError> {
        let position = state.peek().unwrap().position;

        Parser::consume(state, TokenType::LeftParen, "Expected '('")?;
        let parameters = Self::arrow_function_parameters(state)?;
        Parser::consume(state, TokenType::RightParen, "Expected ')'")?;

        let mut return_type = None;
        if Parser::match_token(state, &[TokenType::Colon]) {
            return_type = None;
        }

        if !Parser::match_token(state, &[TokenType::Arrow]) {
            return Err(RaccoonError::new(
                "Expected '=>' for arrow function",
                state.peek().unwrap().position,
                state.file.clone(),
            ));
        }

        let body = if state.check(&TokenType::LeftBrace) {
            state.advance();

            let stmts = Vec::new();
            ArrowFnBody::Block(stmts)
        } else {
            let expr = Self::conditional(state)?;
            ArrowFnBody::Expr(Box::new(expr))
        };

        Ok(ArrowFnExpr {
            parameters,
            return_type,
            body,
            is_async,
            position,
        })
    }

    pub fn arrow_function_parameters(
        state: &mut ParserState,
    ) -> Result<Vec<FnParam>, RaccoonError> {
        let mut params = Vec::new();
        let mut has_optional = false;

        if !state.check(&TokenType::RightParen) {
            loop {
                let is_rest = Parser::match_token(state, &[TokenType::Spread]);

                if state.check(&TokenType::LeftBracket) || state.check(&TokenType::LeftBrace) {
                    let pattern = VarPattern::Identifier("TODO".to_string());
                    Parser::consume(
                        state,
                        TokenType::Colon,
                        "Expected ':' after destructuring pattern",
                    )?;

                    let param_type = crate::ast::types::PrimitiveType::any();

                    let is_optional = Parser::match_token(state, &[TokenType::Question]);
                    if is_optional && is_rest {
                        return Err(RaccoonError::new(
                            "Rest parameters cannot be optional".to_string(),
                            state.peek().unwrap().position.clone(),
                            None::<String>,
                        ));
                    }
                    if is_optional {
                        has_optional = true;
                    } else if has_optional {
                        return Err(RaccoonError::new(
                            "Required parameters cannot follow optional parameters".to_string(),
                            state.peek().unwrap().position.clone(),
                            None::<String>,
                        ));
                    }

                    params.push(FnParam {
                        pattern,
                        param_type,
                        default_value: None,
                        is_rest,
                        is_optional,
                    });
                } else {
                    let name =
                        Parser::consume(state, TokenType::Identifier, "Expected parameter name")?
                            .value
                            .clone();

                    let is_optional = Parser::match_token(state, &[TokenType::Question]);

                    let param_type = if Parser::match_token(state, &[TokenType::Colon]) {
                        crate::ast::types::PrimitiveType::any()
                    } else {
                        crate::ast::types::PrimitiveType::any()
                    };

                    if is_optional && is_rest {
                        return Err(RaccoonError::new(
                            "Rest parameters cannot be optional".to_string(),
                            state.peek().unwrap().position.clone(),
                            None::<String>,
                        ));
                    }
                    if is_optional {
                        has_optional = true;
                    } else if has_optional {
                        return Err(RaccoonError::new(
                            "Required parameters cannot follow optional parameters".to_string(),
                            state.peek().unwrap().position.clone(),
                            None::<String>,
                        ));
                    }

                    let mut default_value = None;
                    if Parser::match_token(state, &[TokenType::Assign]) {
                        default_value = Some(Self::expression(state)?);
                    }

                    params.push(FnParam {
                        pattern: VarPattern::Identifier(name),
                        param_type,
                        default_value,
                        is_rest,
                        is_optional,
                    });
                }

                if is_rest {
                    break;
                }

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Ok(params)
    }

    fn try_parse_call_type_arguments(state: &mut ParserState) -> Result<Vec<Type>, RaccoonError> {
        if !Parser::match_token(state, &[TokenType::Lt]) {
            return Err(RaccoonError::new(
                "Expected '<'",
                state.peek().unwrap().position,
                state.file.clone(),
            ));
        }

        let mut type_args = Vec::new();
        loop {
            type_args.push(crate::ast::types::PrimitiveType::any());
            if !Parser::match_token(state, &[TokenType::Comma]) {
                break;
            }
        }

        Parser::consume(state, TokenType::Gt, "Expected '>' after type arguments")?;
        Ok(type_args)
    }

    pub fn match_expr(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;

        let scrutinee = Box::new(Self::conditional(state)?);

        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' after match scrutinee",
        )?;

        let mut arms = Vec::new();

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            let pattern = Self::parse_pattern(state)?;
            let position = pattern.position();

            Parser::consume(state, TokenType::Arrow, "Expected '=>' after pattern")?;

            let body = Box::new(Self::conditional(state)?);

            arms.push(MatchArm {
                pattern,
                guard: None,
                body,
                position,
            });

            if !state.check(&TokenType::RightBrace) {
                Parser::consume(state, TokenType::Comma, "Expected ',' after match arm")?;
            }
        }

        Parser::consume(
            state,
            TokenType::RightBrace,
            "Expected '}' after match arms",
        )?;

        Ok(Expr::Match(MatchExpr {
            scrutinee,
            arms,
            position,
        }))
    }

    pub fn parse_pattern(state: &mut ParserState) -> Result<Pattern, RaccoonError> {
        if Parser::match_token(state, &[TokenType::Underscore]) {
            return Ok(Pattern::Wildcard(state.previous().unwrap().position));
        }

        if Parser::match_token(state, &[TokenType::Identifier]) {
            let name = state.previous().unwrap().value.clone();
            return Ok(Pattern::Variable(name));
        }

        if Parser::match_token(state, &[TokenType::LeftBracket]) {
            let _position = state.previous().unwrap().position;
            let mut patterns = Vec::new();

            if !state.check(&TokenType::RightBracket) {
                loop {
                    patterns.push(Self::parse_pattern(state)?);
                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
            }

            Parser::consume(
                state,
                TokenType::RightBracket,
                "Expected ']' after array pattern",
            )?;
            return Ok(Pattern::Array(patterns));
        }

        if Parser::match_token(state, &[TokenType::LeftBrace]) {
            let _position = state.previous().unwrap().position;
            let mut properties = Vec::new();

            if !state.check(&TokenType::RightBrace) {
                loop {
                    let key = Parser::consume(
                        state,
                        TokenType::Identifier,
                        "Expected property name in object pattern",
                    )?
                    .value
                    .clone();

                    let pattern = if Parser::match_token(state, &[TokenType::Colon]) {
                        Self::parse_pattern(state)?
                    } else {
                        Pattern::Variable(key.clone())
                    };

                    properties.push((key, pattern));

                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
            }

            Parser::consume(
                state,
                TokenType::RightBrace,
                "Expected '}' after object pattern",
            )?;
            return Ok(Pattern::Object(properties));
        }

        if state.check(&TokenType::IntLiteral)
            || state.check(&TokenType::FloatLiteral)
            || state.check(&TokenType::StrLiteral)
            || state.check(&TokenType::True)
            || state.check(&TokenType::False)
            || state.check(&TokenType::NullLiteral)
        {
            let expr = Self::primary(state)?;
            return Ok(Pattern::Literal(Box::new(expr)));
        }

        Err(RaccoonError::new(
            "Expected pattern",
            state.peek().unwrap().position,
            state.file.clone(),
        ))
    }

    pub fn class_expr(state: &mut ParserState) -> Result<Expr, RaccoonError> {
        let position = state.previous().unwrap().position;

        let type_parameters = Self::parse_type_parameters(state)?;

        let mut superclass = None;
        if Parser::match_token(state, &[TokenType::Extends]) {
            superclass = Some(
                Parser::consume(state, TokenType::Identifier, "Expected superclass name")?
                    .value
                    .clone(),
            );
        }

        if Parser::match_token(state, &[TokenType::Implements]) {
            loop {
                Parser::consume(state, TokenType::Identifier, "Expected interface name")?;
                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::LeftBrace, "Expected '{' for class body")?;

        let mut properties = Vec::new();
        let mut methods = Vec::new();
        let mut accessors = Vec::new();
        let mut constructor = None;

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            let mut member_decorators = Vec::new();
            while Parser::match_token(state, &[TokenType::At]) {
                member_decorators.push(Declarations::parse_decorator(state)?);
            }

            let mut is_static = false;
            let mut access_modifier = AccessModifier::Public;

            if Parser::match_token(state, &[TokenType::Static]) {
                is_static = true;
            }

            if Parser::match_token(state, &[TokenType::Public]) {
                access_modifier = AccessModifier::Public;
            } else if Parser::match_token(state, &[TokenType::Private]) {
                access_modifier = AccessModifier::Private;
            } else if Parser::match_token(state, &[TokenType::Protected]) {
                access_modifier = AccessModifier::Protected;
            }

            if !is_static && Parser::match_token(state, &[TokenType::Static]) {
                is_static = true;
            }

            let mut is_async = false;
            if Parser::match_token(state, &[TokenType::Async]) {
                is_async = true;
            }

            if Parser::match_token(state, &[TokenType::Constructor]) {
                if constructor.is_some() {
                    return Err(RaccoonError::new(
                        "Class cannot have multiple constructors",
                        state.previous().unwrap().position,
                        state.file.clone(),
                    ));
                }
                constructor = Some(Declarations::parse_constructor(state)?);
            } else if !is_static
                && !is_async
                && state.check(&TokenType::Get)
                && Parser::check_next(state, &[TokenType::Identifier])
                && Declarations::check_next_next(state, &[TokenType::LeftParen])
            {
                accessors.push(Declarations::parse_accessor(
                    state,
                    crate::ast::nodes::AccessorKind::Get,
                    member_decorators,
                    access_modifier,
                )?);
            } else if !is_static
                && !is_async
                && state.check(&TokenType::Set)
                && Parser::check_next(state, &[TokenType::Identifier])
                && Declarations::check_next_next(state, &[TokenType::LeftParen])
            {
                accessors.push(Declarations::parse_accessor(
                    state,
                    crate::ast::nodes::AccessorKind::Set,
                    member_decorators,
                    access_modifier,
                )?);
            } else if (state.check(&TokenType::Identifier)
                || (is_static || is_async)
                    && (state.check(&TokenType::Get) || state.check(&TokenType::Set)))
                && Parser::check_next(state, &[TokenType::LeftParen])
            {
                methods.push(Declarations::parse_method(
                    state,
                    member_decorators,
                    access_modifier,
                    is_static,
                    is_async,
                )?);
            } else {
                properties.push(Declarations::parse_class_property(
                    state,
                    member_decorators,
                    access_modifier,
                )?);
            }
        }

        Parser::consume(
            state,
            TokenType::RightBrace,
            "Expected '}' after class body",
        )?;

        Ok(Expr::Class(ClassExpr {
            type_parameters,
            superclass,
            properties,
            constructor,
            methods,
            accessors,
            position,
        }))
    }

    fn parse_type_parameters(
        state: &mut ParserState,
    ) -> Result<Vec<crate::ast::types::TypeParameter>, RaccoonError> {
        let mut type_params = Vec::new();
        if Parser::match_token(state, &[TokenType::Lt]) {
            loop {
                let name =
                    Parser::consume(state, TokenType::Identifier, "Expected type parameter name")?
                        .value
                        .clone();

                type_params.push(crate::ast::types::TypeParameter {
                    name,
                    constraint: None,
                });

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
            Parser::consume(state, TokenType::Gt, "Expected '>' after type parameters")?;
        }
        Ok(type_params)
    }
}
