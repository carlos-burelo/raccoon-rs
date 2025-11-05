use crate::{Position, Token, TokenType};

pub struct ParserState {
    pub tokens: Vec<Token>,
    pub file: Option<String>,
    pub current: usize,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>, file: Option<String>) -> Self {
        Self {
            tokens,
            file,
            current: 0,
        }
    }

    /// Retorna el token actual sin avanzar
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Retorna el token anterior
    pub fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    /// Avanza al siguiente token
    pub fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Verifica si está al final
    pub fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.token_type == TokenType::Eof)
            .unwrap_or(true)
    }

    /// Verifica si el token actual es del tipo especificado
    pub fn check(&self, token_type: &TokenType) -> bool {
        self.peek()
            .map(|t| std::mem::discriminant(&t.token_type) == std::mem::discriminant(token_type))
            .unwrap_or(false)
    }

    /// Retorna la posición actual
    pub fn current_position(&self) -> Position {
        self.peek()
            .map(|t| t.position)
            .or_else(|| self.previous().map(|t| t.position))
            .unwrap_or((0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_state_creation() {
        let tokens = vec![
            Token {
                token_type: TokenType::Let,
                value: "let".to_string(),
                position: (1, 1),
            },
            Token {
                token_type: TokenType::Eof,
                value: "".to_string(),
                position: (1, 4),
            },
        ];

        let state = ParserState::new(tokens, Some("test.rcc".to_string()));
        assert_eq!(state.current, 0);
        assert!(!state.is_at_end());
    }

    #[test]
    fn test_parser_state_peek() {
        let tokens = vec![Token {
            token_type: TokenType::Let,
            value: "let".to_string(),
            position: (1, 1),
        }];

        let state = ParserState::new(tokens, None);
        assert!(state.peek().is_some());
        assert_eq!(state.peek().unwrap().token_type, TokenType::Let);
    }

    #[test]
    fn test_parser_state_advance() {
        let tokens = vec![
            Token {
                token_type: TokenType::Let,
                value: "let".to_string(),
                position: (1, 1),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "x".to_string(),
                position: (1, 5),
            },
            Token {
                token_type: TokenType::Eof,
                value: "".to_string(),
                position: (1, 6),
            },
        ];

        let mut state = ParserState::new(tokens, None);
        assert_eq!(state.current, 0);

        state.advance();
        assert_eq!(state.current, 1);

        state.advance();
        assert_eq!(state.current, 2);
    }

    #[test]
    fn test_parser_state_check() {
        let tokens = vec![Token {
            token_type: TokenType::Let,
            value: "let".to_string(),
            position: (1, 1),
        }];

        let state = ParserState::new(tokens, None);
        assert!(state.check(&TokenType::Let));
        assert!(!state.check(&TokenType::Const));
    }
}
