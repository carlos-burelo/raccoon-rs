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

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.token_type == TokenType::Eof)
            .unwrap_or(true)
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        self.peek()
            .map(|t| std::mem::discriminant(&t.token_type) == std::mem::discriminant(token_type))
            .unwrap_or(false)
    }

    pub fn current_position(&self) -> Position {
        self.peek()
            .map(|t| t.position)
            .or_else(|| self.previous().map(|t| t.position))
            .unwrap_or((0, 0))
    }
}
