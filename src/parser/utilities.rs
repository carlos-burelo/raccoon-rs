use super::state::ParserState;
use crate::{RaccoonError, Token, TokenType};

pub struct Parser;

impl Parser {
    pub fn optional_semicolon(state: &mut ParserState) {
        if Self::match_token(state, &[TokenType::Semicolon]) || Self::can_insert_semicolon(state) {
            return;
        }
    }

    pub fn can_insert_semicolon(state: &ParserState) -> bool {
        state.check(&TokenType::RightBrace)
            || state.is_at_end()
            || Self::previous_token_on_different_line(state)
    }

    pub fn previous_token_on_different_line(state: &ParserState) -> bool {
        if state.current == 0 || state.current >= state.tokens.len() {
            return false;
        }
        state.tokens[state.current].position.0 > state.tokens[state.current - 1].position.0
    }

    pub fn match_token(state: &mut ParserState, types: &[TokenType]) -> bool {
        for token_type in types {
            if state.check(token_type) {
                state.advance();
                return true;
            }
        }
        false
    }

    pub fn check_next(state: &ParserState, types: &[TokenType]) -> bool {
        if state.current + 1 >= state.tokens.len() {
            return false;
        }
        types.contains(&state.tokens[state.current + 1].token_type)
    }

    pub fn check_next_next(state: &ParserState, types: &[TokenType]) -> bool {
        if state.current + 2 >= state.tokens.len() {
            return false;
        }
        types.contains(&state.tokens[state.current + 2].token_type)
    }

    pub fn consume<'a>(
        state: &'a mut ParserState,
        token_type: TokenType,
        message: &str,
    ) -> Result<&'a Token, RaccoonError> {
        if state.check(&token_type) {
            return Ok(state.advance().unwrap());
        }
        Err(RaccoonError::new(
            message,
            state.current_position(),
            state.file.clone(),
        ))
    }

    pub fn synchronize(state: &mut ParserState) {
        state.advance();

        while !state.is_at_end() {
            if state.previous().unwrap().token_type == TokenType::Semicolon {
                return;
            }

            match state.peek().unwrap().token_type {
                TokenType::Let
                | TokenType::Const
                | TokenType::Fn
                | TokenType::Class
                | TokenType::Interface
                | TokenType::Enum
                | TokenType::TypeAlias
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Return => return,
                _ => {}
            }

            state.advance();
        }
    }

    pub fn consume_property_name(state: &mut ParserState) -> Result<String, RaccoonError> {
        let token = state.peek().unwrap();

        match token.token_type {
            TokenType::Identifier
            | TokenType::Get
            | TokenType::Set
            | TokenType::Static
            | TokenType::Public
            | TokenType::Private
            | TokenType::Protected
            | TokenType::Async
            | TokenType::Constructor
            | TokenType::Default
            | TokenType::TypeAlias
            | TokenType::Typeof
            | TokenType::Instanceof
            | TokenType::Await
            | TokenType::New
            | TokenType::This
            | TokenType::Super
            | TokenType::Catch
            | TokenType::Finally
            | TokenType::Try
            | TokenType::Throw
            | TokenType::Return
            | TokenType::Break
            | TokenType::Continue
            | TokenType::If
            | TokenType::Else
            | TokenType::While
            | TokenType::For
            | TokenType::Import
            | TokenType::Export
            | TokenType::From
            | TokenType::As => {
                let name = state.advance().unwrap().value.clone();
                Ok(name)
            }
            _ => Err(RaccoonError::new(
                "Expected property name after '.'",
                token.position,
                state.file.clone(),
            )),
        }
    }
}
