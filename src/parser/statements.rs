use super::state::ParserState;
use super::utilities::Parser;
use crate::{ast::nodes::*, RaccoonError, TokenType};

pub struct Statements;

impl Statements {
    pub fn statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        if Parser::match_token(state, &[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Block {
                statements: Self::block_statements(state)?,
                position: state.previous().unwrap().position,
            }));
        }
        if Parser::match_token(state, &[TokenType::If]) {
            return Self::if_statement(state);
        }
        if Parser::match_token(state, &[TokenType::While]) {
            return Self::while_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Do]) {
            return Self::do_while_statement(state);
        }
        if Parser::match_token(state, &[TokenType::For]) {
            return Self::for_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Switch]) {
            return Self::switch_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Return]) {
            return Self::return_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Try]) {
            return Self::try_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Throw]) {
            return Self::throw_statement(state);
        }
        if Parser::match_token(state, &[TokenType::Break]) {
            let stmt = Stmt::BreakStmt(BreakStmt {
                position: state.previous().unwrap().position,
            });
            Parser::optional_semicolon(state);
            return Ok(stmt);
        }
        if Parser::match_token(state, &[TokenType::Continue]) {
            let stmt = Stmt::ContinueStmt(ContinueStmt {
                position: state.previous().unwrap().position,
            });
            Parser::optional_semicolon(state);
            return Ok(stmt);
        }
        Self::expression_statement(state)
    }

    pub fn try_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;

        Parser::consume(state, TokenType::LeftBrace, "Expected '{' after 'try'")?;
        let try_block = Block {
            statements: Self::block_statements(state)?,
            position,
        };

        let mut catch_clauses = Vec::new();
        while Parser::match_token(state, &[TokenType::Catch]) {
            let catch_pos = state.previous().unwrap().position;
            Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'catch'")?;
            let error_var =
                Parser::consume(state, TokenType::Identifier, "Expected error variable name")?
                    .value
                    .clone();

            let mut error_type = None;
            if Parser::match_token(state, &[TokenType::Colon]) {
                error_type = None;
            }

            Parser::consume(
                state,
                TokenType::RightParen,
                "Expected ')' after catch parameter",
            )?;
            Parser::consume(
                state,
                TokenType::LeftBrace,
                "Expected '{' after catch clause",
            )?;
            let body = Block {
                statements: Self::block_statements(state)?,
                position: catch_pos,
            };

            catch_clauses.push(CatchClause {
                error_var,
                error_type,
                body,
                position: catch_pos,
            });
        }

        let mut finally_block = None;
        if Parser::match_token(state, &[TokenType::Finally]) {
            Parser::consume(state, TokenType::LeftBrace, "Expected '{' after 'finally'")?;
            finally_block = Some(Block {
                statements: Self::block_statements(state)?,
                position: state.previous().unwrap().position,
            });
        }

        if catch_clauses.is_empty() && finally_block.is_none() {
            return Err(RaccoonError::new(
                "Try statement must have at least one catch or finally block",
                position,
                state.file.clone(),
            ));
        }

        Ok(Stmt::TryStmt(TryStmt {
            try_block,
            catch_clauses,
            finally_block,
            position,
        }))
    }

    pub fn throw_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;

        let value = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position,
        });
        Parser::optional_semicolon(state);
        Ok(Stmt::ThrowStmt(ThrowStmt { value, position }))
    }

    pub fn block_statements(state: &mut ParserState) -> Result<Vec<Stmt>, RaccoonError> {
        let mut statements = Vec::new();
        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            statements.push(Stmt::ExprStmt(ExprStmt {
                expression: Expr::Identifier(Identifier {
                    name: "TODO".to_string(),
                    position: (0, 0),
                }),
                position: (0, 0),
            }));
        }
        Parser::consume(state, TokenType::RightBrace, "Expected '}'")?;
        Ok(statements)
    }

    pub fn if_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'if'")?;

        let condition = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position,
        });
        Parser::consume(state, TokenType::RightParen, "Expected ')' after condition")?;

        let then_branch = Box::new(Self::statement(state)?);

        let mut else_branch = None;
        if Parser::match_token(state, &[TokenType::Else]) {
            else_branch = Some(Box::new(Self::statement(state)?));
        }

        Ok(Stmt::IfStmt(IfStmt {
            condition,
            then_branch,
            else_branch,
            position,
        }))
    }

    pub fn while_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'while'")?;

        let condition = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position,
        });
        Parser::consume(state, TokenType::RightParen, "Expected ')' after condition")?;
        let body = Box::new(Self::statement(state)?);

        Ok(Stmt::WhileStmt(WhileStmt {
            condition,
            body,
            position,
        }))
    }

    pub fn for_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'for'")?;

        if Parser::match_token(state, &[TokenType::Let, TokenType::Const]) {
            let is_const = state.previous().unwrap().token_type == TokenType::Const;
            let var_name = Parser::consume(state, TokenType::Identifier, "Expected variable name")?
                .value
                .clone();

            let mut type_annotation = None;
            if Parser::match_token(state, &[TokenType::Colon]) {
                type_annotation = None;
            }

            if Parser::match_token(state, &[TokenType::In]) {
                let iterable = Expr::Identifier(Identifier {
                    name: "TODO".to_string(),
                    position,
                });
                Parser::consume(state, TokenType::RightParen, "Expected ')' after iterable")?;
                let body = Box::new(Self::statement(state)?);
                return Ok(Stmt::ForInStmt(ForInStmt {
                    variable: var_name,
                    is_const,
                    type_annotation,
                    iterable,
                    body,
                    position,
                }));
            }

            if Parser::match_token(state, &[TokenType::Of]) {
                let iterable = Expr::Identifier(Identifier {
                    name: "TODO".to_string(),
                    position,
                });
                Parser::consume(state, TokenType::RightParen, "Expected ')' after iterable")?;
                let body = Box::new(Self::statement(state)?);
                return Ok(Stmt::ForOfStmt(ForOfStmt {
                    variable: var_name,
                    is_const,
                    type_annotation,
                    iterable,
                    body,
                    position,
                }));
            }

            Parser::consume(
                state,
                TokenType::Assign,
                "Expected '=' in variable declaration",
            )?;

            let init_value = Expr::Identifier(Identifier {
                name: "TODO".to_string(),
                position,
            });

            let var_decl = Box::new(Stmt::VarDecl(VarDecl {
                pattern: VarPattern::Identifier(var_name),
                type_annotation: type_annotation.unwrap_or(crate::ast::types::PrimitiveType::any()),
                initializer: Some(init_value),
                is_constant: is_const,
                position,
            }));

            Parser::optional_semicolon(state);

            let mut condition = None;
            if !state.check(&TokenType::Semicolon) {
                condition = None;
            }
            Parser::optional_semicolon(state);

            let mut increment = None;
            if !state.check(&TokenType::RightParen) {
                increment = None;
            }
            Parser::consume(
                state,
                TokenType::RightParen,
                "Expected ')' after for clauses",
            )?;
            let body = Box::new(Self::statement(state)?);

            return Ok(Stmt::ForStmt(ForStmt {
                initializer: Some(var_decl),
                condition,
                increment,
                body,
                position,
            }));
        }

        let mut initializer = None;
        if !Parser::match_token(state, &[TokenType::Semicolon]) {
            initializer = Some(Box::new(Self::expression_statement(state)?));
        }

        let mut condition = None;
        if !state.check(&TokenType::Semicolon) {
            condition = None;
        }
        Parser::optional_semicolon(state);

        let mut increment = None;
        if !state.check(&TokenType::RightParen) {
            increment = None;
        }
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after for clauses",
        )?;
        let body = Box::new(Self::statement(state)?);

        Ok(Stmt::ForStmt(ForStmt {
            initializer,
            condition,
            increment,
            body,
            position,
        }))
    }

    pub fn do_while_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        let body = Box::new(Self::statement(state)?);
        Parser::consume(
            state,
            TokenType::While,
            "Expected 'while' after do-while body",
        )?;
        Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'while'")?;

        let condition = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position,
        });
        Parser::consume(state, TokenType::RightParen, "Expected ')' after condition")?;
        Parser::optional_semicolon(state);

        Ok(Stmt::DoWhileStmt(DoWhileStmt {
            body,
            condition,
            position,
        }))
    }

    pub fn switch_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        Parser::consume(state, TokenType::LeftParen, "Expected '(' after 'switch'")?;

        let discriminant = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position,
        });
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after discriminant",
        )?;
        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' to start switch body",
        )?;

        let mut cases = Vec::new();

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            if Parser::match_token(state, &[TokenType::Case]) {
                let test = Some(Expr::Identifier(Identifier {
                    name: "TODO".to_string(),
                    position,
                }));
                Parser::consume(state, TokenType::Colon, "Expected ':' after case value")?;

                let mut consequent = Vec::new();
                while !state.check(&TokenType::Case)
                    && !state.check(&TokenType::Default)
                    && !state.check(&TokenType::RightBrace)
                    && !state.is_at_end()
                {
                    consequent.push(Self::statement(state)?);
                }

                cases.push(SwitchCase { test, consequent });
            } else if Parser::match_token(state, &[TokenType::Default]) {
                Parser::consume(state, TokenType::Colon, "Expected ':' after 'default'")?;

                let mut consequent = Vec::new();
                while !state.check(&TokenType::Case)
                    && !state.check(&TokenType::Default)
                    && !state.check(&TokenType::RightBrace)
                    && !state.is_at_end()
                {
                    consequent.push(Self::statement(state)?);
                }

                cases.push(SwitchCase {
                    test: None,
                    consequent,
                });
            } else {
                return Err(RaccoonError::new(
                    "Expected 'case' or 'default' in switch statement".to_string(),
                    state.peek().unwrap().position,
                    state.file.clone(),
                ));
            }
        }

        Parser::consume(
            state,
            TokenType::RightBrace,
            "Expected '}' after switch body",
        )?;

        Ok(Stmt::SwitchStmt(SwitchStmt {
            discriminant,
            cases,
            position,
        }))
    }

    pub fn return_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut value = None;

        if !state.check(&TokenType::Semicolon) && !Parser::can_insert_semicolon(state) {
            value = None;
        }

        Parser::optional_semicolon(state);
        Ok(Stmt::ReturnStmt(ReturnStmt { value, position }))
    }

    pub fn expression_statement(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let expr = Expr::Identifier(Identifier {
            name: "TODO".to_string(),
            position: (0, 0),
        });
        let position = expr.position();
        Parser::optional_semicolon(state);
        Ok(Stmt::ExprStmt(ExprStmt {
            expression: expr,
            position,
        }))
    }
}
