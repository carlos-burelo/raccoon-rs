use std::collections::HashMap;

use super::state::ParserState;
use super::utilities::Parser;
use crate::{ast::types::*, RaccoonError, TokenType};

pub struct Types;

impl Types {
    /// Parse a type: Union | Intersection | Postfix | Primary
    pub fn parse_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        let mut type_ = Self::parse_intersection_type(state)?;

        if Parser::match_token(state, &[TokenType::BitwiseOr]) {
            let mut types = vec![type_];
            loop {
                types.push(Self::parse_intersection_type(state)?);
                if !Parser::match_token(state, &[TokenType::BitwiseOr]) {
                    break;
                }
            }
            type_ = Type::Union(Box::new(UnionType::new(types)));
        }

        Ok(type_)
    }

    /// Parse intersection type: Type & Type & Type
    pub fn parse_intersection_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        let mut type_ = Self::parse_postfix_type(state)?;

        if Parser::match_token(state, &[TokenType::Ampersand]) {
            let mut types = vec![type_];
            loop {
                types.push(Self::parse_postfix_type(state)?);
                if !Parser::match_token(state, &[TokenType::Ampersand]) {
                    break;
                }
            }
            type_ = Type::Intersection(Box::new(IntersectionType::new(types)));
        }

        Ok(type_)
    }

    /// Parse postfix type: Type? or Type[] or Type[][]
    pub fn parse_postfix_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        let mut type_ = Self::parse_primary_type(state)?;

        if Parser::match_token(state, &[TokenType::Question]) {
            type_ = Type::Nullable(Box::new(NullableType { inner_type: type_ }));
        }

        while Parser::match_token(state, &[TokenType::LeftBracket]) {
            Parser::consume(state, TokenType::RightBracket, "Expected ']'")?;
            type_ = Type::List(Box::new(ListType {
                element_type: type_,
            }));
        }

        Ok(type_)
    }

    /// Parse primary type: readonly Type, keyof Type, typeof Name, { ... }, [ ... ], (params) => RetType, null, Identifier, or builtin types
    pub fn parse_primary_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        if Parser::match_token(state, &[TokenType::Readonly]) {
            let inner_type = Self::parse_primary_type(state)?;
            return Ok(Type::Readonly(Box::new(ReadonlyType::new(inner_type))));
        }

        if Parser::match_token(state, &[TokenType::KeyOf]) {
            let target_type = Self::parse_primary_type(state)?;
            return Ok(Type::KeyOf(Box::new(KeyOfType::new(target_type))));
        }

        if Parser::match_token(state, &[TokenType::Typeof]) {
            let expression_name = Parser::consume(
                state,
                TokenType::Identifier,
                "Expected identifier after typeof",
            )?
            .value
            .clone();
            return Ok(Type::TypeOf(Box::new(TypeOfType::new(expression_name))));
        }

        if state.check(&TokenType::LeftBrace) {
            return Self::parse_object_type(state);
        }

        if state.check(&TokenType::LeftBracket) {
            return Self::parse_tuple_type(state);
        }

        if Parser::match_token(state, &[TokenType::NullLiteral]) {
            return Ok(PrimitiveType::null());
        }

        if state.check(&TokenType::Identifier) {
            let name = state.advance().unwrap().value.clone();

            let builtin_type = match name.as_str() {
                "int" => Some(PrimitiveType::int()),
                "float" => Some(PrimitiveType::float()),
                "str" => Some(PrimitiveType::str()),
                "bool" => Some(PrimitiveType::bool()),
                "null" => Some(PrimitiveType::null()),
                "void" => Some(PrimitiveType::void()),
                "any" => Some(PrimitiveType::any()),
                "unknown" => Some(PrimitiveType::unknown()),
                "never" => Some(PrimitiveType::never()),
                "func" => Some(PrimitiveType::func()),
                _ => None,
            };

            if let Some(t) = builtin_type {
                return Ok(t);
            }

            if Parser::match_token(state, &[TokenType::Lt]) {
                let mut type_args = Vec::new();
                loop {
                    type_args.push(Self::parse_type(state)?);
                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
                Parser::consume(state, TokenType::Gt, "Expected '>' after type arguments")?;

                if name == "Future" {
                    if type_args.len() != 1 {
                        return Err(RaccoonError::new(
                            "Future requires exactly one type argument",
                            state.previous().unwrap().position,
                            state.file.clone(),
                        ));
                    }
                    return Ok(Type::Future(Box::new(FutureType {
                        inner_type: type_args.into_iter().next().unwrap(),
                    })));
                }

                if name == "Map" {
                    if type_args.len() != 2 {
                        return Err(RaccoonError::new(
                            "Map requires exactly two type arguments",
                            state.previous().unwrap().position,
                            state.file.clone(),
                        ));
                    }
                    let mut iter = type_args.into_iter();
                    return Ok(Type::Map(Box::new(MapType {
                        key_type: iter.next().unwrap(),
                        value_type: iter.next().unwrap(),
                    })));
                }

                return Ok(Type::Generic(Box::new(GenericType {
                    base: Type::TypeRef(TypeReference {
                        name,
                        file: state.file.clone(),
                    }),
                    type_args,
                })));
            }

            return Ok(Type::TypeRef(TypeReference {
                name,
                file: state.file.clone(),
            }));
        }

        if Parser::match_token(state, &[TokenType::LeftParen]) {
            let mut param_types = Vec::new();
            if !state.check(&TokenType::RightParen) {
                loop {
                    if state.check(&TokenType::Identifier)
                        && Parser::check_next(state, &[TokenType::Colon])
                    {
                        state.advance();
                        state.advance();
                    }
                    param_types.push(Self::parse_type(state)?);
                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
            }
            Parser::consume(state, TokenType::RightParen, "Expected ')'")?;
            Parser::consume(state, TokenType::Arrow, "Expected '=>' or '->'")?;
            let return_type = Self::parse_type(state)?;

            return Ok(Type::Function(Box::new(FunctionType {
                params: param_types,
                return_type,
                is_variadic: false,
            })));
        }

        Err(RaccoonError::new(
            "Expected type",
            state.peek().unwrap().position,
            state.file.clone(),
        ))
    }

    /// Parse tuple type: [Type1, Type2, ...]
    pub fn parse_tuple_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        state.advance();
        let mut element_types = Vec::new();

        if !state.check(&TokenType::RightBracket) {
            loop {
                element_types.push(Self::parse_type(state)?);
                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(
            state,
            TokenType::RightBracket,
            "Expected ']' after tuple types",
        )?;
        Ok(Type::Tuple(Box::new(TupleType::new(element_types))))
    }

    /// Parse object type: { name: Type, readonly name2?: Type, ... }
    pub fn parse_object_type(state: &mut ParserState) -> Result<Type, RaccoonError> {
        state.advance();
        let mut properties = HashMap::new();

        if !state.check(&TokenType::RightBrace) {
            loop {
                let is_readonly = Parser::match_token(state, &[TokenType::Readonly]);

                let prop_name =
                    Parser::consume(state, TokenType::Identifier, "Expected property name")?
                        .value
                        .clone();

                let is_optional = Parser::match_token(state, &[TokenType::Question]);

                let prop_type = if Parser::match_token(state, &[TokenType::LeftParen]) {
                    let mut param_types = Vec::new();
                    if !state.check(&TokenType::RightParen) {
                        loop {
                            if state.check(&TokenType::Identifier)
                                && Parser::check_next(state, &[TokenType::Colon])
                            {
                                state.advance();
                                state.advance();
                            }
                            param_types.push(Self::parse_type(state)?);
                            if !Parser::match_token(state, &[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    Parser::consume(
                        state,
                        TokenType::RightParen,
                        "Expected ')' after method parameters",
                    )?;
                    Parser::consume(
                        state,
                        TokenType::Colon,
                        "Expected ':' after method signature",
                    )?;
                    let return_type = Self::parse_type(state)?;

                    Type::Function(Box::new(FunctionType {
                        params: param_types,
                        return_type,
                        is_variadic: false,
                    }))
                } else {
                    Parser::consume(state, TokenType::Colon, "Expected ':' after property name")?;
                    Self::parse_type(state)?
                };

                let mut object_prop = ObjectProperty::new(prop_type);
                if is_optional {
                    object_prop = object_prop.optional();
                }
                if is_readonly {
                    object_prop = object_prop.readonly();
                }

                properties.insert(prop_name, object_prop);

                let has_separator = Parser::match_token(state, &[TokenType::Comma])
                    || Parser::match_token(state, &[TokenType::Semicolon]);

                if state.check(&TokenType::RightBrace) {
                    break;
                }

                if !has_separator {
                    return Err(RaccoonError::new(
                        "Expected ',' or ';' after property type",
                        state.peek().unwrap().position,
                        state.file.clone(),
                    ));
                }
            }
        }

        Parser::consume(
            state,
            TokenType::RightBrace,
            "Expected '}' after object type",
        )?;
        Ok(Type::Object(Box::new(ObjectType::new(properties))))
    }
}
