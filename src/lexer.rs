use crate::{
    error::RaccoonError,
    tokens::{Position, Token, TokenType},
};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "let" => TokenType::Let,
    "const" => TokenType::Const,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "while" => TokenType::While,
    "for" => TokenType::For,
    "in" => TokenType::In,
    "break" => TokenType::Break,
    "continue" => TokenType::Continue,
    "fn" => TokenType::Fn,
    "class" => TokenType::Class,
    "new" => TokenType::New,
    "this" => TokenType::This,
    "super" => TokenType::Super,
    "constructor" => TokenType::Constructor,
    "return" => TokenType::Return,
    "interface" => TokenType::Interface,
    "enum" => TokenType::Enum,
    "type" => TokenType::TypeAlias,
    "typeof" => TokenType::Typeof,
    "keyof" => TokenType::KeyOf,
    "instanceof" => TokenType::Instanceof,
    "readonly" => TokenType::Readonly,
    "implements" => TokenType::Implements,
    "extends" => TokenType::Extends,
    "static" => TokenType::Static,
    "private" => TokenType::Private,
    "public" => TokenType::Public,
    "protected" => TokenType::Protected,
    "async" => TokenType::Async,
    "await" => TokenType::Await,
    "try" => TokenType::Try,
    "catch" => TokenType::Catch,
    "finally" => TokenType::Finally,
    "throw" => TokenType::Throw,
    "get" => TokenType::Get,
    "set" => TokenType::Set,
    "import" => TokenType::Import,
    "export" => TokenType::Export,
    "from" => TokenType::From,
    "as" => TokenType::As,
    "default" => TokenType::Default,
    "declare" => TokenType::Declare,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "null" => TokenType::NullLiteral,
};

static SIMPLE_OPERATORS: phf::Map<char, TokenType> = phf_map! {
    '+' => TokenType::Plus,
    '-' => TokenType::Minus,
    '*' => TokenType::Multiply,
    '/' => TokenType::Divide,
    '%' => TokenType::Modulo,
    '=' => TokenType::Assign,
    '!' => TokenType::Bang,
    '<' => TokenType::Lt,
    '>' => TokenType::Gt,
    '.' => TokenType::Dot,
    ',' => TokenType::Comma,
    ':' => TokenType::Colon,
    ';' => TokenType::Semicolon,
    '?' => TokenType::Question,
    '|' => TokenType::BitwiseOr,
    '&' => TokenType::Ampersand,
    '^' => TokenType::BitwiseXor,
    '~' => TokenType::BitwiseNot,
    '@' => TokenType::At,
    '(' => TokenType::LeftParen,
    ')' => TokenType::RightParen,
    '{' => TokenType::LeftBrace,
    '}' => TokenType::RightBrace,
    '[' => TokenType::LeftBracket,
    ']' => TokenType::RightBracket,
};

static COMPOUND_OPERATORS: phf::Map<&'static str, TokenType> = phf_map! {
    "===" => TokenType::Eq,
    "!==" => TokenType::Neq,
    "==" => TokenType::Eq,
    "!=" => TokenType::Neq,
    "<=" => TokenType::Lte,
    ">=" => TokenType::Gte,
    "&&" => TokenType::And,
    "||" => TokenType::Or,
    "..." => TokenType::Spread,
    ".." => TokenType::Range,
    "->" => TokenType::Arrow,
    "=>" => TokenType::Arrow,
    "+=" => TokenType::PlusAssign,
    "-=" => TokenType::MinusAssign,
    "*=" => TokenType::MultiplyAssign,
    "/=" => TokenType::DivideAssign,
    "%=" => TokenType::ModuloAssign,
    "++" => TokenType::Increment,
    "--" => TokenType::Decrement,
    "?." => TokenType::QuestionDot,
    "??" => TokenType::QuestionQuestion,
    "&=" => TokenType::AmpersandAssign,
    "|=" => TokenType::BitwiseOrAssign,
    "^=" => TokenType::BitwiseXorAssign,
    "<<" => TokenType::LeftShift,
    ">>" => TokenType::RightShift,
    "**" => TokenType::Exponent,
    ">>>" => TokenType::UnsignedRightShift,
    "<<=" => TokenType::LeftShiftAssign,
    ">>=" => TokenType::RightShiftAssign,
    "**=" => TokenType::ExponentAssign,
    ">>>=" => TokenType::UnsignedRightShiftAssign,
};

pub struct Lexer {
    source: Vec<char>,
    file: Option<String>,
    tokens: Vec<Token>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: String, file: Option<String>) -> Self {
        Self {
            source: source.chars().collect(),
            file,
            tokens: Vec::new(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, RaccoonError> {
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let char = self.peek();
            let next = self.peek_next();

            if self.is_alpha(char) {
                self.identifier()?;
            } else if self.is_digit(char) {
                self.number()?;
            } else if char == '\'' || char == '"' {
                self.string()?;
            } else if char == '`' {
                self.template_string()?;
            } else if char == '/' && next == '/' {
                self.skip_line_comment();
            } else if char == '/' && next == '*' {
                self.skip_block_comment();
            } else {
                let four = format!(
                    "{}{}{}{}",
                    char,
                    next,
                    self.peek_ahead(2),
                    self.peek_ahead(3)
                );

                if let Some(&token_type) = COMPOUND_OPERATORS.get(&four.as_str()) {
                    self.add_token(token_type, four, None);
                    self.advance();
                    self.advance();
                    self.advance();
                    self.advance();
                } else {
                    let three = format!("{}{}{}", char, next, self.peek_ahead(2));

                    if let Some(&token_type) = COMPOUND_OPERATORS.get(&three.as_str()) {
                        self.add_token(token_type, three, None);
                        self.advance();
                        self.advance();
                        self.advance();
                    } else {
                        let compound = format!("{}{}", char, next);

                        if let Some(&token_type) = COMPOUND_OPERATORS.get(&compound.as_str()) {
                            self.add_token(token_type, compound, None);
                            self.advance();
                            self.advance();
                        } else if let Some(&token_type) = SIMPLE_OPERATORS.get(&char) {
                            self.add_token(token_type, char.to_string(), None);
                            self.advance();
                        } else {
                            return Err(RaccoonError::new(
                                format!("Unexpected character: '{}'", char),
                                (self.line, self.column),
                                self.file.clone(),
                            ));
                        }
                    }
                }
            }
        }

        self.add_token(TokenType::Eof, String::new(), None);
        Ok(self.tokens.clone())
    }

    fn identifier(&mut self) -> Result<(), RaccoonError> {
        let start = self.position;
        let start_column = self.column;

        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[start..self.position].iter().collect();
        let token_type = KEYWORDS
            .get(&text.as_str())
            .copied()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type, text, Some((self.line, start_column)));
        Ok(())
    }

    fn number(&mut self) -> Result<(), RaccoonError> {
        let start = self.position;
        let start_column = self.column;
        let mut is_float = false;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            is_float = true;
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let text: String = self.source[start..self.position].iter().collect();
        let token_type = if is_float {
            TokenType::FloatLiteral
        } else {
            TokenType::IntLiteral
        };

        self.add_token(token_type, text, Some((self.line, start_column)));
        Ok(())
    }

    fn string(&mut self) -> Result<(), RaccoonError> {
        let start_column = self.column;
        let quote = self.advance();
        let mut value = String::new();

        while self.peek() != quote && !self.is_at_end() {
            if self.peek() == '\\' {
                self.advance();
                value.push(self.get_escaped_char(self.peek()));
                self.advance();
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(RaccoonError::new(
                "Unterminated string",
                (self.line, start_column),
                self.file.clone(),
            ));
        }

        self.advance();
        self.add_token(
            TokenType::StrLiteral,
            value,
            Some((self.line, start_column)),
        );
        Ok(())
    }

    fn template_string(&mut self) -> Result<(), RaccoonError> {
        let start_column = self.column;
        self.advance();

        self.add_token(
            TokenType::TemplateStrStart,
            "`".to_string(),
            Some((self.line, start_column)),
        );

        let mut value = String::new();

        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '$' && self.peek_next() == '{' {
                if !value.is_empty() {
                    let val_len = value.len();
                    self.add_token(
                        TokenType::TemplateStrPart,
                        value.clone(),
                        Some((self.line, self.column - val_len)),
                    );
                    value.clear();
                }

                self.advance();
                self.advance();

                self.add_token(
                    TokenType::TemplateInterpolationStart,
                    "${".to_string(),
                    Some((self.line, self.column - 2)),
                );

                let mut brace_count = 1;

                while brace_count > 0 && !self.is_at_end() {
                    if self.is_whitespace(self.peek()) {
                        self.skip_whitespace();
                        continue;
                    }

                    let char = self.peek();

                    if self.is_alpha(char) {
                        self.identifier()?;
                    } else if self.is_digit(char) {
                        self.number()?;
                    } else if char == '\'' || char == '"' {
                        self.string()?;
                    } else if char == '{' {
                        brace_count += 1;
                        self.add_token(TokenType::LeftBrace, char.to_string(), None);
                        self.advance();
                    } else if char == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                        self.add_token(TokenType::RightBrace, char.to_string(), None);
                        self.advance();
                    } else {
                        let next = self.peek_next();
                        let compound = format!("{}{}", char, next);

                        if let Some(&token_type) = COMPOUND_OPERATORS.get(&compound.as_str()) {
                            self.add_token(token_type, compound, None);
                            self.advance();
                            self.advance();
                        } else if let Some(&token_type) = SIMPLE_OPERATORS.get(&char) {
                            self.add_token(token_type, char.to_string(), None);
                            self.advance();
                        } else {
                            return Err(RaccoonError::new(
                                format!("Unexpected character in interpolation: '{}'", char),
                                (self.line, self.column),
                                self.file.clone(),
                            ));
                        }
                    }
                }

                self.advance();
                self.add_token(
                    TokenType::TemplateInterpolationEnd,
                    "}".to_string(),
                    Some((self.line, self.column - 1)),
                );
            } else {
                if self.peek() == '\\' {
                    self.advance();
                    value.push(self.get_escaped_char(self.peek()));
                    self.advance();
                } else {
                    if self.peek() == '\n' {
                        self.line += 1;
                        self.column = 0;
                    }
                    value.push(self.advance());
                }
            }
        }

        if self.is_at_end() {
            return Err(RaccoonError::new(
                "Unterminated template string",
                (self.line, start_column),
                self.file.clone(),
            ));
        }

        if !value.is_empty() {
            let val_len = value.len();
            self.add_token(
                TokenType::TemplateStrPart,
                value,
                Some((self.line, self.column - val_len)),
            );
        }

        self.advance();
        self.add_token(
            TokenType::TemplateStrEnd,
            "`".to_string(),
            Some((self.line, self.column - 1)),
        );

        Ok(())
    }

    fn skip_line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        self.advance();
        self.advance();

        while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if !self.is_at_end() {
            self.advance();
            self.advance();
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let char = self.peek();

            if char == ' ' || char == '\t' || char == '\r' {
                self.advance();
            } else if char == '\n' {
                self.line += 1;
                self.column = 0;
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_escaped_char(&self, char: char) -> char {
        match char {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '\\' => '\\',
            '\'' => '\'',
            '"' => '"',
            '`' => '`',
            '$' => '$',
            _ => char,
        }
    }

    fn is_alpha(&self, char: char) -> bool {
        char.is_ascii_alphabetic() || char == '_'
    }

    fn is_digit(&self, char: char) -> bool {
        char.is_ascii_digit()
    }

    fn is_alpha_numeric(&self, char: char) -> bool {
        self.is_alpha(char) || self.is_digit(char)
    }

    fn is_whitespace(&self, char: char) -> bool {
        char.is_whitespace()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + 1]
        }
    }

    fn peek_ahead(&self, offset: usize) -> char {
        if self.position + offset >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + offset]
        }
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.position];
        self.position += 1;
        self.column += 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType, text: String, position: Option<Position>) {
        let pos = position.unwrap_or((self.line, self.column.saturating_sub(text.len())));
        self.tokens.push(Token::new(token_type, text, pos));
    }
}
