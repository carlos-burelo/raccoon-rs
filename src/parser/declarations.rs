use super::expressions::Expressions;
use super::state::ParserState;
use super::utilities::Parser;
use crate::{
    ast::nodes::*,
    ast::types::{PrimitiveType, Type, TypeParameter},
    tokens::AccessModifier,
    RaccoonError, TokenType,
};

pub struct Declarations;

impl Declarations {
    /// Entry point dispatcher for declarations
    pub fn declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let mut decorators = Vec::new();

        while Parser::match_token(state, &[TokenType::At]) {
            decorators.push(Self::parse_decorator(state)?);
        }

        if Parser::match_token(state, &[TokenType::Import]) {
            return Self::import_declaration(state);
        }
        if Parser::match_token(state, &[TokenType::Export]) {
            return Self::export_declaration(state);
        }
        if Parser::match_token(state, &[TokenType::Let, TokenType::Const]) {
            return Self::var_declaration(state);
        }
        if Parser::match_token(state, &[TokenType::Declare]) {
            return Self::function_declaration(state, decorators, true);
        }
        if Parser::match_token(state, &[TokenType::Async]) {
            return Self::function_declaration(state, decorators, false);
        }
        if Parser::match_token(state, &[TokenType::Fn]) {
            return Self::function_declaration(state, decorators, false);
        }
        if Parser::match_token(state, &[TokenType::Class]) {
            return Self::class_declaration(state, decorators);
        }
        if Parser::match_token(state, &[TokenType::Interface]) {
            return Self::interface_declaration(state);
        }
        if Parser::match_token(state, &[TokenType::Enum]) {
            return Self::enum_declaration(state);
        }
        if Parser::match_token(state, &[TokenType::TypeAlias]) {
            return Self::type_alias_declaration(state);
        }

        if !decorators.is_empty() {
            return Err(RaccoonError::new(
                "Decorators can only be applied to classes and functions",
                decorators[0].position,
                state.file.clone(),
            ));
        }

        // TODO: Call statement() - needs to be from statements module
        Err(RaccoonError::new(
            "Expected declaration or statement",
            state.current_position(),
            state.file.clone(),
        ))
    }

    /// Variable declaration: let x: T = init; or const x: T = init;
    pub fn var_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let is_constant = state.previous().unwrap().token_type == TokenType::Const;

        let pattern = if state.check(&TokenType::LeftBracket) || state.check(&TokenType::LeftBrace)
        {
            VarPattern::Destructuring(Self::parse_destructuring_pattern(state)?)
        } else {
            VarPattern::Identifier(
                Parser::consume(state, TokenType::Identifier, "Expected variable name")?
                    .value
                    .clone(),
            )
        };

        let mut type_annotation = PrimitiveType::any();

        if Parser::match_token(state, &[TokenType::Colon]) {
            // TODO: Call parse_type() from types module
            type_annotation = PrimitiveType::any();
        }

        let mut initializer = None;

        if Parser::match_token(state, &[TokenType::Assign]) {
            initializer = Some(Expressions::expression(state)?);
        } else if is_constant {
            return Err(RaccoonError::new(
                "Constants must be initialized",
                state.previous().unwrap().position,
                state.file.clone(),
            ));
        } else if type_annotation == PrimitiveType::any() {
            return Err(RaccoonError::new(
                "Variable declaration must have either a type annotation or an initializer",
                state.previous().unwrap().position,
                state.file.clone(),
            ));
        }

        Parser::optional_semicolon(state);

        Ok(Stmt::VarDecl(VarDecl {
            pattern,
            type_annotation,
            initializer,
            is_constant,
            position: state.previous().unwrap().position,
        }))
    }

    /// Function declaration: fn name<T>(params: Types): RetType { body }
    pub fn function_declaration(
        state: &mut ParserState,
        decorators: Vec<DecoratorDecl>,
        is_declare: bool,
    ) -> Result<Stmt, RaccoonError> {
        let is_async = if is_declare {
            Parser::match_token(state, &[TokenType::Async])
        } else {
            state.previous().unwrap().token_type == TokenType::Async
        };

        if is_declare || is_async {
            Parser::consume(
                state,
                TokenType::Fn,
                "Expected 'fn' after 'declare' or 'async'",
            )?;
        }

        let name = Parser::consume(state, TokenType::Identifier, "Expected function name")?
            .value
            .clone();
        let position = state.previous().unwrap().position;

        let type_parameters = Self::parse_type_parameters(state)?;

        Parser::consume(
            state,
            TokenType::LeftParen,
            "Expected '(' after function name",
        )?;
        let parameters = Self::function_parameters(state)?;
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after parameters",
        )?;

        let return_type = if Parser::match_token(state, &[TokenType::Colon]) {
            // TODO: Call parse_type() from types module
            None
        } else {
            None
        };

        let body = if is_declare {
            Parser::optional_semicolon(state);
            Vec::new()
        } else {
            Parser::consume(
                state,
                TokenType::LeftBrace,
                "Expected '{' before function body",
            )?;
            // TODO: Call block_statements() from statements module
            Vec::new()
        };

        Ok(Stmt::FnDecl(FnDecl {
            name,
            type_parameters,
            parameters,
            return_type,
            body,
            is_async,
            is_declare,
            decorators,
            position,
        }))
    }

    /// Parse type parameters: <T, U extends Base>
    pub fn parse_type_parameters(
        state: &mut ParserState,
    ) -> Result<Vec<TypeParameter>, RaccoonError> {
        let mut type_params = Vec::new();

        if !Parser::match_token(state, &[TokenType::Lt]) {
            return Ok(type_params);
        }

        loop {
            let param_name =
                Parser::consume(state, TokenType::Identifier, "Expected type parameter name")?
                    .value
                    .clone();

            let mut constraint = None;
            if Parser::match_token(state, &[TokenType::Extends]) {
                // TODO: Call parse_type() from types module
                constraint = None;
            }

            type_params.push(TypeParameter {
                name: param_name,
                constraint,
            });

            if !Parser::match_token(state, &[TokenType::Comma]) {
                break;
            }
        }

        Parser::consume(state, TokenType::Gt, "Expected '>' after type parameters")?;
        Ok(type_params)
    }

    /// Check if lookahead is class or interface (for export default detection)
    pub fn lookahead_is_class_or_interface(state: &ParserState) -> bool {
        if state.current + 1 >= state.tokens.len() {
            return false;
        }
        let next_token_type = state.tokens[state.current + 1].token_type;
        next_token_type == TokenType::Class || next_token_type == TokenType::Interface
    }

    /// Class declaration: class Name<T> extends Base implements I { ... }
    pub fn class_declaration(
        state: &mut ParserState,
        decorators: Vec<DecoratorDecl>,
    ) -> Result<Stmt, RaccoonError> {
        let name = Parser::consume(state, TokenType::Identifier, "Expected class name")?
            .value
            .clone();
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

        Parser::consume(state, TokenType::LeftBrace, "Expected '{' after class name")?;

        let mut properties = Vec::new();
        let mut methods = Vec::new();
        let mut accessors = Vec::new();
        let mut constructor = None;

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            let mut member_decorators = Vec::new();
            while Parser::match_token(state, &[TokenType::At]) {
                member_decorators.push(Self::parse_decorator(state)?);
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
                constructor = Some(Self::parse_constructor(state)?);
            } else if !is_static
                && !is_async
                && state.check(&TokenType::Get)
                && Parser::check_next(state, &[TokenType::Identifier])
                && Declarations::check_next_next(state, &[TokenType::LeftParen])
            {
                accessors.push(Self::parse_accessor(
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
                accessors.push(Self::parse_accessor(
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
                methods.push(Self::parse_method(
                    state,
                    member_decorators,
                    access_modifier,
                    is_static,
                    is_async,
                )?);
            } else {
                properties.push(Self::parse_class_property(
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

        Ok(Stmt::ClassDecl(ClassDecl {
            name,
            type_parameters,
            superclass,
            properties,
            constructor,
            methods,
            accessors,
            decorators,
            position,
        }))
    }

    /// Parse class property: name: Type = init;
    pub fn parse_class_property(
        state: &mut ParserState,
        decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
    ) -> Result<ClassProperty, RaccoonError> {
        let name = Parser::consume(state, TokenType::Identifier, "Expected property name")?
            .value
            .clone();

        Parser::consume(state, TokenType::Colon, "Expected ':' after property name")?;
        // TODO: Call parse_type() from types module
        let property_type = PrimitiveType::any();

        let mut initializer = None;
        if Parser::match_token(state, &[TokenType::Assign]) {
            // TODO: Call expression() from expressions module
            initializer = None;
        }

        Parser::optional_semicolon(state);

        Ok(ClassProperty {
            name,
            property_type,
            initializer,
            decorators,
            access_modifier,
        })
    }

    /// Parse constructor: constructor(params: Types) { body }
    pub fn parse_constructor(state: &mut ParserState) -> Result<ConstructorDecl, RaccoonError> {
        let position = state.previous().unwrap().position;

        Parser::consume(
            state,
            TokenType::LeftParen,
            "Expected '(' after constructor",
        )?;
        let parameters = Self::function_parameters(state)?;
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after parameters",
        )?;

        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' before constructor body",
        )?;
        // TODO: Call block_statements() from statements module
        let body = Vec::new();

        Ok(ConstructorDecl {
            parameters,
            body,
            position,
        })
    }

    /// Parse class method: [static] [async] name(params: Types): RetType { body }
    pub fn parse_method(
        state: &mut ParserState,
        decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
        is_static: bool,
        is_async: bool,
    ) -> Result<ClassMethod, RaccoonError> {
        let name = if (state.check(&TokenType::Get) || state.check(&TokenType::Set))
            && Parser::check_next(state, &[TokenType::LeftParen])
        {
            state.advance().unwrap().value.clone()
        } else {
            Parser::consume(state, TokenType::Identifier, "Expected method name")?
                .value
                .clone()
        };

        Parser::consume(
            state,
            TokenType::LeftParen,
            "Expected '(' after method name",
        )?;
        let parameters = Self::function_parameters(state)?;
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after parameters",
        )?;

        let return_type = if Parser::match_token(state, &[TokenType::Colon]) {
            // TODO: Call parse_type() from types module
            None
        } else {
            None
        };

        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' before method body",
        )?;
        // TODO: Call block_statements() from statements module
        let body = Vec::new();

        Ok(ClassMethod {
            name,
            parameters,
            return_type,
            body,
            is_async,
            decorators,
            access_modifier,
            is_static,
        })
    }

    /// Parse accessor (getter/setter): get/set name(params: Types): RetType { body }
    pub fn parse_accessor(
        state: &mut ParserState,
        kind: AccessorKind,
        _decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
    ) -> Result<PropertyAccessor, RaccoonError> {
        state.advance();
        let position = state.previous().unwrap().position;
        let name = Parser::consume(state, TokenType::Identifier, "Expected accessor name")?
            .value
            .clone();

        Parser::consume(
            state,
            TokenType::LeftParen,
            "Expected '(' after accessor name",
        )?;
        let parameters = Self::function_parameters(state)?;
        Parser::consume(
            state,
            TokenType::RightParen,
            "Expected ')' after parameters",
        )?;

        if kind == AccessorKind::Get && !parameters.is_empty() {
            return Err(RaccoonError::new(
                "Getter cannot have parameters",
                position,
                state.file.clone(),
            ));
        }
        if kind == AccessorKind::Set && parameters.len() != 1 {
            return Err(RaccoonError::new(
                "Setter must have exactly one parameter",
                position,
                state.file.clone(),
            ));
        }

        let return_type = if Parser::match_token(state, &[TokenType::Colon]) {
            // TODO: Call parse_type() from types module
            None
        } else {
            None
        };

        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' before accessor body",
        )?;
        // TODO: Call block_statements() from statements module
        let body = Vec::new();

        Ok(PropertyAccessor {
            name,
            kind,
            parameters,
            return_type,
            body,
            access_modifier,
            position,
        })
    }

    /// Parse decorator: @name or @name(arg1, arg2)
    pub fn parse_decorator(state: &mut ParserState) -> Result<DecoratorDecl, RaccoonError> {
        let position = state.previous().unwrap().position;
        let name = Parser::consume(state, TokenType::Identifier, "Expected decorator name")?
            .value
            .clone();

        let mut args = Vec::new();

        if Parser::match_token(state, &[TokenType::LeftParen]) {
            if !state.check(&TokenType::RightParen) {
                loop {
                    // TODO: Call expression() from expressions module
                    args.push(Expr::Identifier(Identifier {
                        name: "TODO".to_string(),
                        position,
                    }));
                    if !Parser::match_token(state, &[TokenType::Comma]) {
                        break;
                    }
                }
            }
            Parser::consume(
                state,
                TokenType::RightParen,
                "Expected ')' after decorator arguments",
            )?;
        }

        Ok(DecoratorDecl {
            name,
            args,
            position,
        })
    }

    /// Parse function parameters: (name: Type, name2: Type2 = default, ...rest: Type3)
    pub fn function_parameters(state: &mut ParserState) -> Result<Vec<FnParam>, RaccoonError> {
        let mut params = Vec::new();
        let mut has_optional = false;

        if !state.check(&TokenType::RightParen) {
            loop {
                let is_rest = Parser::match_token(state, &[TokenType::Spread]);

                if state.check(&TokenType::LeftBracket) || state.check(&TokenType::LeftBrace) {
                    let pattern =
                        VarPattern::Destructuring(Self::parse_destructuring_pattern(state)?);
                    Parser::consume(
                        state,
                        TokenType::Colon,
                        "Expected ':' after destructuring pattern",
                    )?;
                    // TODO: Call parse_type() from types module
                    let param_type = PrimitiveType::any();

                    let is_optional = Parser::match_token(state, &[TokenType::Question]);
                    if is_optional && is_rest {
                        return Err(RaccoonError::new(
                            "Rest parameters cannot be optional".to_string(),
                            state.peek().unwrap().position,
                            None::<String>,
                        ));
                    }
                    if is_optional {
                        has_optional = true;
                    } else if has_optional {
                        return Err(RaccoonError::new(
                            "Required parameters cannot follow optional parameters".to_string(),
                            state.peek().unwrap().position,
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
                    Parser::consume(state, TokenType::Colon, "Expected ':' after parameter name")?;
                    // TODO: Call parse_type() from types module
                    let param_type = PrimitiveType::any();

                    if is_optional && is_rest {
                        return Err(RaccoonError::new(
                            "Rest parameters cannot be optional".to_string(),
                            state.peek().unwrap().position,
                            None::<String>,
                        ));
                    }
                    if is_optional {
                        has_optional = true;
                    } else if has_optional {
                        return Err(RaccoonError::new(
                            "Required parameters cannot follow optional parameters".to_string(),
                            state.peek().unwrap().position,
                            None::<String>,
                        ));
                    }

                    let mut default_value = None;
                    if Parser::match_token(state, &[TokenType::Assign]) {
                        // TODO: Call expression() from expressions module
                        default_value = None;
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

    /// Parse destructuring pattern: [a, b] or { x, y: z }
    pub fn parse_destructuring_pattern(
        state: &mut ParserState,
    ) -> Result<DestructuringPattern, RaccoonError> {
        if Parser::match_token(state, &[TokenType::LeftBracket]) {
            return Ok(DestructuringPattern::Array(Self::parse_array_pattern(state)?));
        }
        if Parser::match_token(state, &[TokenType::LeftBrace]) {
            return Ok(DestructuringPattern::Object(Self::parse_object_pattern(
                state,
            )?));
        }
        Err(RaccoonError::new(
            "Expected destructuring pattern",
            state.peek().unwrap().position,
            state.file.clone(),
        ))
    }

    /// Parse list destructuring pattern: [a, b, ...rest]
    pub fn parse_array_pattern(state: &mut ParserState) -> Result<ArrayPattern, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut elements = Vec::new();
        let mut rest = None;

        if !state.check(&TokenType::RightBracket) {
            loop {
                if Parser::match_token(state, &[TokenType::Spread]) {
                    let name = Parser::consume(
                        state,
                        TokenType::Identifier,
                        "Expected identifier after ...",
                    )?
                    .value
                    .clone();
                    rest = Some(RestElement {
                        argument: Identifier {
                            name,
                            position: state.previous().unwrap().position,
                        },
                        position: state.previous().unwrap().position,
                    });
                    break;
                }

                if state.check(&TokenType::Comma) {
                    elements.push(None);
                } else if Parser::match_token(state, &[TokenType::LeftBracket]) {
                    elements.push(Some(ArrayPatternElement::List(Box::new(
                        Self::parse_array_pattern(state)?,
                    ))));
                } else if Parser::match_token(state, &[TokenType::LeftBrace]) {
                    elements.push(Some(ArrayPatternElement::Object(Box::new(
                        Self::parse_object_pattern(state)?,
                    ))));
                } else {
                    let name =
                        Parser::consume(state, TokenType::Identifier, "Expected identifier")?
                            .value
                            .clone();
                    elements.push(Some(ArrayPatternElement::Identifier(Identifier {
                        name,
                        position: state.previous().unwrap().position,
                    })));
                }

                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }

                if state.check(&TokenType::RightBracket) {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::RightBracket, "Expected ']'")?;
        Ok(ArrayPattern {
            elements,
            rest,
            position,
        })
    }

    /// Parse object destructuring pattern: { x, y: z, ...rest }
    pub fn parse_object_pattern(state: &mut ParserState) -> Result<ObjectPattern, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut properties = Vec::new();
        let mut rest = None;

        if !state.check(&TokenType::RightBrace) {
            loop {
                if Parser::match_token(state, &[TokenType::Spread]) {
                    let name = Parser::consume(
                        state,
                        TokenType::Identifier,
                        "Expected identifier after ...",
                    )?
                    .value
                    .clone();
                    rest = Some(RestElement {
                        argument: Identifier {
                            name,
                            position: state.previous().unwrap().position,
                        },
                        position: state.previous().unwrap().position,
                    });
                    break;
                }

                let key = Parser::consume(state, TokenType::Identifier, "Expected property name")?
                    .value
                    .clone();

                let value = if Parser::match_token(state, &[TokenType::Colon]) {
                    if Parser::match_token(state, &[TokenType::LeftBracket]) {
                        ObjectPatternValue::Array(Self::parse_array_pattern(state)?)
                    } else if Parser::match_token(state, &[TokenType::LeftBrace]) {
                        ObjectPatternValue::Object(Self::parse_object_pattern(state)?)
                    } else {
                        let value_name =
                            Parser::consume(state, TokenType::Identifier, "Expected identifier")?
                                .value
                                .clone();
                        ObjectPatternValue::Identifier(Identifier {
                            name: value_name,
                            position: state.previous().unwrap().position,
                        })
                    }
                } else {
                    ObjectPatternValue::Identifier(Identifier {
                        name: key.clone(),
                        position: state.previous().unwrap().position,
                    })
                };

                properties.push(ObjectPatternProperty { key, value });

                if !Parser::match_token(state, &[TokenType::Comma])
                    || state.check(&TokenType::RightBrace)
                {
                    break;
                }
            }
        }

        Parser::consume(state, TokenType::RightBrace, "Expected '}'")?;
        Ok(ObjectPattern {
            properties,
            rest,
            position,
        })
    }

    /// Interface declaration: interface Name<T> extends Base { ... }
    pub fn interface_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let name = Parser::consume(state, TokenType::Identifier, "Expected interface name")?
            .value
            .clone();
        let position = state.previous().unwrap().position;

        let type_parameters = Self::parse_type_parameters(state)?;

        let mut extends = Vec::new();
        if Parser::match_token(state, &[TokenType::Extends]) {
            loop {
                extends.push(
                    Parser::consume(state, TokenType::Identifier, "Expected interface name")?
                        .value
                        .clone(),
                );
                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
        }

        Parser::consume(
            state,
            TokenType::LeftBrace,
            "Expected '{' after interface name",
        )?;

        let mut properties = Vec::new();

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            let prop_name =
                Parser::consume(state, TokenType::Identifier, "Expected property name")?
                    .value
                    .clone();

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
                        // TODO: Call parse_type() from types module
                        param_types.push(PrimitiveType::any());
                        if !Parser::match_token(state, &[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                Parser::consume(
                    state,
                    TokenType::RightParen,
                    "Expected ')' after parameters",
                )?;
                Parser::consume(
                    state,
                    TokenType::Colon,
                    "Expected ':' after method signature",
                )?;
                // TODO: Call parse_type() from types module
                let return_type = PrimitiveType::any();

                properties.push(InterfaceDeclProperty {
                    name: prop_name,
                    property_type: Type::Function(Box::new(crate::ast::types::FunctionType {
                        params: param_types,
                        return_type,
                        is_variadic: false,
                    })),
                    optional: false,
                });
                Parser::optional_semicolon(state);
            } else {
                Parser::consume(state, TokenType::Colon, "Expected ':' after property name")?;
                // TODO: Call parse_type() from types module
                let prop_type = PrimitiveType::any();
                properties.push(InterfaceDeclProperty {
                    name: prop_name,
                    property_type: prop_type,
                    optional: false,
                });
                Parser::optional_semicolon(state);
            }
        }

        Parser::consume(
            state,
            TokenType::RightBrace,
            "Expected '}' after interface body",
        )?;

        Ok(Stmt::InterfaceDecl(InterfaceDecl {
            name,
            type_parameters,
            properties,
            extends,
            position,
        }))
    }

    /// Enum declaration: enum Name { A, B = 1, ... }
    pub fn enum_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let name = Parser::consume(state, TokenType::Identifier, "Expected enum name")?
            .value
            .clone();
        let position = state.previous().unwrap().position;

        Parser::consume(state, TokenType::LeftBrace, "Expected '{' after enum name")?;

        let mut members = Vec::new();

        while !state.check(&TokenType::RightBrace) && !state.is_at_end() {
            let member_name =
                Parser::consume(state, TokenType::Identifier, "Expected enum member name")?
                    .value
                    .clone();

            let mut value = None;
            if Parser::match_token(state, &[TokenType::Assign]) {
                // TODO: Call expression() from expressions module
                value = None;
            }

            members.push(EnumMember {
                name: member_name,
                value,
            });

            if !state.check(&TokenType::RightBrace) {
                Parser::consume(state, TokenType::Comma, "Expected ',' between enum members")?;
            }
        }

        Parser::consume(state, TokenType::RightBrace, "Expected '}' after enum body")?;

        Ok(Stmt::EnumDecl(EnumDecl {
            name,
            members,
            position,
        }))
    }

    /// Type alias: type Name = Type;
    pub fn type_alias_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let name = Parser::consume(state, TokenType::Identifier, "Expected type alias name")?
            .value
            .clone();
        let position = state.previous().unwrap().position;

        Parser::consume(
            state,
            TokenType::Assign,
            "Expected '=' after type alias name",
        )?;
        // TODO: Call parse_type() from types module
        let alias_type = PrimitiveType::any();

        Parser::optional_semicolon(state);

        Ok(Stmt::TypeAliasDecl(TypeAliasDecl {
            name,
            alias_type,
            position,
        }))
    }

    /// Import declaration: import defaultName, { a, b as c } from 'module'; or import * as ns from 'module';
    pub fn import_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;
        let mut default_import = None;
        let mut namespace_import = None;
        let mut named_imports = Vec::new();

        if Parser::match_token(state, &[TokenType::Multiply]) {
            Parser::consume(state, TokenType::As, "Expected 'as' after '*'")?;
            namespace_import = Some(
                Parser::consume(state, TokenType::Identifier, "Expected namespace name")?
                    .value
                    .clone(),
            );
        } else if state.check(&TokenType::Identifier) {
            default_import = Some(state.advance().unwrap().value.clone());
            if Parser::match_token(state, &[TokenType::Comma]) {
                Parser::consume(
                    state,
                    TokenType::LeftBrace,
                    "Expected '{' after default import",
                )?;
                Self::parse_named_imports(state, &mut named_imports)?;
                Parser::consume(
                    state,
                    TokenType::RightBrace,
                    "Expected '}' after named imports",
                )?;
            }
        } else if Parser::match_token(state, &[TokenType::LeftBrace]) {
            Self::parse_named_imports(state, &mut named_imports)?;
            Parser::consume(
                state,
                TokenType::RightBrace,
                "Expected '}' after named imports",
            )?;
        }

        Parser::consume(
            state,
            TokenType::From,
            "Expected 'from' after import specifiers",
        )?;
        let module_specifier =
            Parser::consume(state, TokenType::StrLiteral, "Expected module path")?
                .value
                .clone();

        Parser::optional_semicolon(state);

        Ok(Stmt::ImportDecl(ImportDecl {
            default_import,
            named_imports,
            namespace_import,
            module_specifier,
            position,
        }))
    }

    /// Parse named imports: { a, b as c, d }
    pub fn parse_named_imports(
        state: &mut ParserState,
        named_imports: &mut Vec<ImportSpecifier>,
    ) -> Result<(), RaccoonError> {
        loop {
            let imported = Parser::consume(state, TokenType::Identifier, "Expected import name")?
                .value
                .clone();
            let mut local = None;
            if Parser::match_token(state, &[TokenType::As]) {
                local = Some(
                    Parser::consume(state, TokenType::Identifier, "Expected local name after as")?
                        .value
                        .clone(),
                );
            }
            named_imports.push(ImportSpecifier { imported, local });
            if !Parser::match_token(state, &[TokenType::Comma]) {
                break;
            }
        }
        Ok(())
    }

    /// Export declaration: export { a, b as c } from 'module'; or export default expr; or export fn foo() { }
    pub fn export_declaration(state: &mut ParserState) -> Result<Stmt, RaccoonError> {
        let position = state.previous().unwrap().position;

        if Parser::match_token(state, &[TokenType::Default]) {
            if state.check(&TokenType::LeftBrace) && !Self::lookahead_is_class_or_interface(state) {
                // TODO: Call expression() from expressions module
                let expression = Expr::Identifier(Identifier {
                    name: "TODO".to_string(),
                    position,
                });
                Parser::optional_semicolon(state);

                return Ok(Stmt::ExportDecl(ExportDecl {
                    declaration: Some(Box::new(Stmt::ExprStmt(ExprStmt {
                        expression,
                        position,
                    }))),
                    specifiers: Vec::new(),
                    is_default: true,
                    module_specifier: None,
                    position,
                }));
            }

            let declaration = Box::new(Self::declaration(state)?);
            return Ok(Stmt::ExportDecl(ExportDecl {
                declaration: Some(declaration),
                specifiers: Vec::new(),
                is_default: true,
                module_specifier: None,
                position,
            }));
        }

        if Parser::match_token(state, &[TokenType::LeftBrace]) {
            let mut specifiers = Vec::new();
            loop {
                let local = Parser::consume(state, TokenType::Identifier, "Expected export name")?
                    .value
                    .clone();
                let mut exported = None;
                if Parser::match_token(state, &[TokenType::As]) {
                    exported = Some(
                        Parser::consume(
                            state,
                            TokenType::Identifier,
                            "Expected exported name after as",
                        )?
                        .value
                        .clone(),
                    );
                }
                specifiers.push(ExportSpecifier { local, exported });
                if !Parser::match_token(state, &[TokenType::Comma]) {
                    break;
                }
            }
            Parser::consume(
                state,
                TokenType::RightBrace,
                "Expected '}' after export specifiers",
            )?;

            let module_specifier = if Parser::match_token(state, &[TokenType::From]) {
                Some(
                    Parser::consume(
                        state,
                        TokenType::StrLiteral,
                        "Expected module path after 'from'",
                    )?
                    .value
                    .clone(),
                )
            } else {
                None
            };

            Parser::optional_semicolon(state);
            return Ok(Stmt::ExportDecl(ExportDecl {
                declaration: None,
                specifiers,
                is_default: false,
                module_specifier,
                position,
            }));
        }

        let declaration = Box::new(Self::declaration(state)?);
        Ok(Stmt::ExportDecl(ExportDecl {
            declaration: Some(declaration),
            specifiers: Vec::new(),
            is_default: false,
            module_specifier: None,
            position,
        }))
    }

    /// Helper: Check if next two tokens match types (used in class member detection)
    pub fn check_next_next(state: &ParserState, types: &[TokenType]) -> bool {
        if state.current + 2 >= state.tokens.len() {
            return false;
        }
        types.contains(&state.tokens[state.current + 2].token_type)
    }
}
