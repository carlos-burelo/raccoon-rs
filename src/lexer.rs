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
    "of" => TokenType::Of,
    "do" => TokenType::Do,
    "match" => TokenType::Match,
    "switch" => TokenType::Switch,
    "case" => TokenType::Case,
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

            if char == '_' && (self.is_at_end() || !self.is_alpha_numeric(self.peek_next())) {
                // Underscore as wildcard pattern
                self.add_token(
                    TokenType::Underscore,
                    "_".to_string(),
                    (self.line, self.column),
                );
                self.advance();
            } else if self.is_alpha(char) {
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

                let start_pos = (self.line, self.column);

                if let Some(&token_type) = COMPOUND_OPERATORS.get(&four.as_str()) {
                    self.advance();
                    self.advance();
                    self.advance();
                    self.advance();
                    self.add_token(token_type, four, start_pos);
                } else {
                    let three = format!("{}{}{}", char, next, self.peek_ahead(2));

                    if let Some(&token_type) = COMPOUND_OPERATORS.get(&three.as_str()) {
                        self.advance();
                        self.advance();
                        self.advance();
                        self.add_token(token_type, three, start_pos);
                    } else {
                        let compound = format!("{}{}", char, next);

                        if let Some(&token_type) = COMPOUND_OPERATORS.get(&compound.as_str()) {
                            self.advance();
                            self.advance();
                            self.add_token(token_type, compound, start_pos);
                        } else if let Some(&token_type) = SIMPLE_OPERATORS.get(&char) {
                            self.advance();
                            self.add_token(token_type, char.to_string(), start_pos);
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

        let eof_pos = (self.line, self.column);
        self.add_token(TokenType::Eof, String::new(), eof_pos);
        Ok(self.tokens.clone())
    }

    fn identifier(&mut self) -> Result<(), RaccoonError> {
        let start = self.position;
        let start_pos = (self.line, self.column);

        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[start..self.position].iter().collect();
        let token_type = KEYWORDS
            .get(&text.as_str())
            .copied()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type, text, start_pos);
        Ok(())
    }

    fn number(&mut self) -> Result<(), RaccoonError> {
        let start = self.position;
        let start_pos = (self.line, self.column);
        let mut is_float = false;
        let mut is_bigint = false;

        // Check for binary (0b), octal (0o), or hex (0x) prefix
        if self.source[start] == '0' && self.position + 1 < self.source.len() {
            let next = self.source[self.position + 1];
            if next == 'b'
                || next == 'B'
                || next == 'o'
                || next == 'O'
                || next == 'x'
                || next == 'X'
            {
                self.advance(); // consume '0'
                self.advance(); // consume 'b', 'o', or 'x'

                // Scan binary, octal, or hex digits
                while self.position < self.source.len() {
                    let ch = self.peek();
                    if ch == '_' {
                        self.advance(); // skip numeric separator
                        continue;
                    }
                    if (next == 'b' || next == 'B') && (ch == '0' || ch == '1') {
                        self.advance();
                    } else if (next == 'o' || next == 'O') && ch >= '0' && ch <= '7' {
                        self.advance();
                    } else if (next == 'x' || next == 'X') && ch.is_ascii_hexdigit() {
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Check for BigInt suffix
                if self.peek() == 'n' {
                    is_bigint = true;
                    self.advance();
                }

                let text: String = self.source[start..self.position].iter().collect();
                let token_type = if is_bigint {
                    TokenType::BigIntLiteral
                } else {
                    TokenType::IntLiteral
                };

                self.add_token(token_type, text, start_pos);
                return Ok(());
            }
        }

        // Regular decimal number with optional numeric separators
        while self.is_digit(self.peek()) || self.peek() == '_' {
            if self.peek() != '_' {
                self.advance();
            } else {
                self.advance(); // consume separator but it won't be in the final value
            }
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            is_float = true;
            self.advance();

            while self.is_digit(self.peek()) || self.peek() == '_' {
                if self.peek() != '_' {
                    self.advance();
                } else {
                    self.advance();
                }
            }
        }

        // Check for BigInt suffix 'n'
        if self.peek() == 'n' && !is_float {
            is_bigint = true;
            self.advance();
        }

        let text: String = self.source[start..self.position].iter().collect();
        let token_type = if is_bigint {
            TokenType::BigIntLiteral
        } else if is_float {
            TokenType::FloatLiteral
        } else {
            TokenType::IntLiteral
        };

        self.add_token(token_type, text, start_pos);
        Ok(())
    }

    fn string(&mut self) -> Result<(), RaccoonError> {
        let start_pos = (self.line, self.column);
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
                    self.column = 1;
                    value.push(self.source[self.position]);
                    self.position += 1;
                } else {
                    value.push(self.advance());
                }
            }
        }

        if self.is_at_end() {
            return Err(RaccoonError::new(
                "Unterminated string",
                start_pos,
                self.file.clone(),
            ));
        }

        self.advance();
        self.add_token(TokenType::StrLiteral, value, start_pos);
        Ok(())
    }

    fn template_string(&mut self) -> Result<(), RaccoonError> {
        let start_pos = (self.line, self.column);
        self.advance();

        self.add_token(
            TokenType::TemplateStrStart,
            "`".to_string(),
            (start_pos.0, start_pos.1),
        );

        let mut value = String::new();

        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '$' && self.peek_next() == '{' {
                if !value.is_empty() {
                    let part_pos = (self.line, self.column.saturating_sub(value.len()));
                    self.add_token(TokenType::TemplateStrPart, value.clone(), part_pos);
                    value.clear();
                }

                let interp_start_pos = (self.line, self.column);
                self.advance();
                self.advance();

                self.add_token(
                    TokenType::TemplateInterpolationStart,
                    "${".to_string(),
                    interp_start_pos,
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
                        let brace_pos = (self.line, self.column);
                        self.advance();
                        self.add_token(TokenType::LeftBrace, "{".to_string(), brace_pos);
                    } else if char == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                        let brace_pos = (self.line, self.column);
                        self.advance();
                        self.add_token(TokenType::RightBrace, "}".to_string(), brace_pos);
                    } else {
                        let op_start_pos = (self.line, self.column);
                        let next = self.peek_next();
                        let compound = format!("{}{}", char, next);

                        if let Some(&token_type) = COMPOUND_OPERATORS.get(&compound.as_str()) {
                            self.advance();
                            self.advance();
                            self.add_token(token_type, compound, op_start_pos);
                        } else if let Some(&token_type) = SIMPLE_OPERATORS.get(&char) {
                            self.advance();
                            self.add_token(token_type, char.to_string(), op_start_pos);
                        } else {
                            return Err(RaccoonError::new(
                                format!("Unexpected character in interpolation: '{}'", char),
                                (self.line, self.column),
                                self.file.clone(),
                            ));
                        }
                    }
                }

                let interp_end_pos = (self.line, self.column);
                self.advance();
                self.add_token(
                    TokenType::TemplateInterpolationEnd,
                    "}".to_string(),
                    interp_end_pos,
                );
            } else {
                if self.peek() == '\\' {
                    self.advance();
                    value.push(self.get_escaped_char(self.peek()));
                    self.advance();
                } else {
                    if self.peek() == '\n' {
                        self.line += 1;
                        self.column = 1;
                        value.push(self.source[self.position]);
                        self.position += 1;
                    } else {
                        value.push(self.advance());
                    }
                }
            }
        }

        if self.is_at_end() {
            return Err(RaccoonError::new(
                "Unterminated template string",
                start_pos,
                self.file.clone(),
            ));
        }

        if !value.is_empty() {
            let part_pos = (self.line, self.column.saturating_sub(value.len()));
            self.add_token(TokenType::TemplateStrPart, value.clone(), part_pos);
        }

        let end_pos = (self.line, self.column);
        self.advance();
        self.add_token(TokenType::TemplateStrEnd, "`".to_string(), end_pos);

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
                self.column = 1;
                self.position += 1;
            } else {
                self.advance();
            }
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
                self.column = 1;
                self.position += 1;
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

    fn add_token(&mut self, token_type: TokenType, value: String, start_pos: Position) {
        self.tokens.push(Token::new(token_type, value, start_pos));
    }
}
