use std::collections::HashMap;

use crate::{
    Position, RaccoonError, Token, TokenType,
    ast::{
        nodes::{InterfaceProperty, *},
        types::*,
    },
    tokens::{AccessModifier, BinaryOperator, UnaryOperator},
};

pub struct Parser {
    tokens: Vec<Token>,
    file: Option<String>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file: Option<String>) -> Self {
        Self {
            tokens,
            current: 0,
            file,
        }
    }

    pub fn parse(&mut self) -> Result<Program, RaccoonError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    eprintln!("{}", e);
                    self.synchronize();
                }
            }
        }

        Ok(Program {
            stmts: statements,
            position: (1, 1),
        })
    }

    fn declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let mut decorators = Vec::new();

        while self.match_token(&[TokenType::At]) {
            decorators.push(self.parse_decorator()?);
        }

        if self.match_token(&[TokenType::Import]) {
            return self.import_declaration();
        }
        if self.match_token(&[TokenType::Export]) {
            return self.export_declaration();
        }
        if self.match_token(&[TokenType::Let, TokenType::Const]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::Async]) {
            return self.function_declaration(decorators);
        }
        if self.match_token(&[TokenType::Fn]) {
            return self.function_declaration(decorators);
        }
        if self.match_token(&[TokenType::Class]) {
            return self.class_declaration(decorators);
        }
        if self.match_token(&[TokenType::Interface]) {
            return self.interface_declaration();
        }
        if self.match_token(&[TokenType::Enum]) {
            return self.enum_declaration();
        }
        if self.match_token(&[TokenType::TypeAlias]) {
            return self.type_alias_declaration();
        }

        if !decorators.is_empty() {
            return Err(RaccoonError::new(
                "Decorators can only be applied to classes and functions",
                decorators[0].position,
                self.file.clone(),
            ));
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let is_constant = self.previous().token_type == TokenType::Const;

        let pattern = if self.check(&TokenType::LeftBracket) || self.check(&TokenType::LeftBrace) {
            VarPattern::Destructuring(self.parse_destructuring_pattern()?)
        } else {
            VarPattern::Identifier(
                self.consume(TokenType::Identifier, "Expected variable name")?
                    .value
                    .clone(),
            )
        };

        let mut type_annotation = PrimitiveType::any();

        if self.match_token(&[TokenType::Colon]) {
            type_annotation = self.parse_type()?;
        }

        let mut initializer = None;

        if self.match_token(&[TokenType::Assign]) {
            initializer = Some(self.expression()?);
        } else if is_constant {
            return Err(RaccoonError::new(
                "Constants must be initialized",
                self.previous().position,
                self.file.clone(),
            ));
        } else if type_annotation == PrimitiveType::any() {
            return Err(RaccoonError::new(
                "Variable declaration must have either a type annotation or an initializer",
                self.previous().position,
                self.file.clone(),
            ));
        }

        self.optional_semicolon();

        Ok(Stmt::VarDecl(VarDecl {
            pattern,
            type_annotation,
            initializer,
            is_constant,
            position: self.previous().position,
        }))
    }

    fn function_declaration(
        &mut self,
        decorators: Vec<DecoratorDecl>,
    ) -> Result<Stmt, RaccoonError> {
        let is_async = self.previous().token_type == TokenType::Async;

        if is_async {
            self.consume(TokenType::Fn, "Expected 'fn' after 'async'")?;
        }

        let name = self
            .consume(TokenType::Identifier, "Expected function name")?
            .value
            .clone();
        let position = self.previous().position;

        let type_parameters = self.parse_type_parameters()?;

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let parameters = self.function_parameters()?;
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::Colon, "Expected ':' after parameters")?;
        let return_type = self.parse_type()?;

        self.consume(TokenType::LeftBrace, "Expected '{' before function body")?;
        let body = self.block_statements()?;

        Ok(Stmt::FnDecl(FnDecl {
            name,
            type_parameters,
            parameters,
            return_type,
            body,
            is_async,
            decorators,
            position,
        }))
    }

    fn parse_type_parameters(&mut self) -> Result<Vec<TypeParameter>, RaccoonError> {
        let mut type_params = Vec::new();

        if !self.match_token(&[TokenType::Lt]) {
            return Ok(type_params);
        }

        loop {
            let param_name = self
                .consume(TokenType::Identifier, "Expected type parameter name")?
                .value
                .clone();

            let mut constraint = None;
            if self.match_token(&[TokenType::Extends]) {
                constraint = Some(Box::new(self.parse_type()?));
            }

            type_params.push(TypeParameter {
                name: param_name,
                constraint,
            });

            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }

        self.consume(TokenType::Gt, "Expected '>' after type parameters")?;
        Ok(type_params)
    }

    fn class_declaration(&mut self, decorators: Vec<DecoratorDecl>) -> Result<Stmt, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected class name")?
            .value
            .clone();
        let position = self.previous().position;

        let type_parameters = self.parse_type_parameters()?;

        let mut superclass = None;
        if self.match_token(&[TokenType::Extends]) {
            superclass = Some(
                self.consume(TokenType::Identifier, "Expected superclass name")?
                    .value
                    .clone(),
            );
        }

        if self.match_token(&[TokenType::Implements]) {
            loop {
                self.consume(TokenType::Identifier, "Expected interface name")?;
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::LeftBrace, "Expected '{' after class name")?;

        let mut properties = Vec::new();
        let mut methods = Vec::new();
        let mut accessors = Vec::new();
        let mut constructor = None;

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let mut member_decorators = Vec::new();

            while self.match_token(&[TokenType::At]) {
                member_decorators.push(self.parse_decorator()?);
            }

            let mut is_static = false;
            let mut access_modifier = AccessModifier::Public;

            if self.match_token(&[TokenType::Static]) {
                is_static = true;
            }

            if self.match_token(&[TokenType::Public]) {
                access_modifier = AccessModifier::Public;
            } else if self.match_token(&[TokenType::Private]) {
                access_modifier = AccessModifier::Private;
            } else if self.match_token(&[TokenType::Protected]) {
                access_modifier = AccessModifier::Protected;
            }

            if !is_static && self.match_token(&[TokenType::Static]) {
                is_static = true;
            }

            if self.match_token(&[TokenType::Constructor]) {
                if constructor.is_some() {
                    return Err(RaccoonError::new(
                        "Class cannot have multiple constructors",
                        self.previous().position,
                        self.file.clone(),
                    ));
                }
                constructor = Some(self.parse_constructor()?);
            } else if self.match_token(&[TokenType::Get]) {
                accessors.push(self.parse_accessor(
                    AccessorKind::Get,
                    member_decorators,
                    access_modifier,
                )?);
            } else if self.match_token(&[TokenType::Set]) {
                accessors.push(self.parse_accessor(
                    AccessorKind::Set,
                    member_decorators,
                    access_modifier,
                )?);
            } else if self.check(&TokenType::Identifier) && self.check_next(&[TokenType::LeftParen])
            {
                methods.push(self.parse_method(member_decorators, access_modifier, is_static)?);
            } else {
                properties.push(self.parse_class_property(member_decorators, access_modifier)?);
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after class body")?;

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

    fn parse_class_property(
        &mut self,
        decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
    ) -> Result<ClassProperty, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected property name")?
            .value
            .clone();

        self.consume(TokenType::Colon, "Expected ':' after property name")?;
        let property_type = self.parse_type()?;

        let mut initializer = None;
        if self.match_token(&[TokenType::Assign]) {
            initializer = Some(self.expression()?);
        }

        self.optional_semicolon();

        Ok(ClassProperty {
            name,
            property_type,
            initializer,
            decorators,
            access_modifier,
        })
    }

    fn parse_constructor(&mut self) -> Result<ConstructorDecl, RaccoonError> {
        let position = self.previous().position;

        self.consume(TokenType::LeftParen, "Expected '(' after constructor")?;
        let parameters = self.function_parameters()?;
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before constructor body")?;
        let body = self.block_statements()?;

        Ok(ConstructorDecl {
            parameters,
            body,
            position,
        })
    }

    fn parse_method(
        &mut self,
        decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
        is_static: bool,
    ) -> Result<ClassMethod, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected method name")?
            .value
            .clone();

        self.consume(TokenType::LeftParen, "Expected '(' after method name")?;
        let parameters = self.function_parameters()?;
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::Colon, "Expected ':' after parameters")?;
        let return_type = self.parse_type()?;

        self.consume(TokenType::LeftBrace, "Expected '{' before method body")?;
        let body = self.block_statements()?;

        Ok(ClassMethod {
            name,
            parameters,
            return_type,
            body,
            is_async: false,
            decorators,
            access_modifier,
            is_static,
        })
    }

    fn parse_accessor(
        &mut self,
        kind: AccessorKind,
        _decorators: Vec<DecoratorDecl>,
        access_modifier: AccessModifier,
    ) -> Result<PropertyAccessor, RaccoonError> {
        let position = self.previous().position;
        let name = self
            .consume(TokenType::Identifier, "Expected accessor name")?
            .value
            .clone();

        self.consume(TokenType::LeftParen, "Expected '(' after accessor name")?;
        let parameters = self.function_parameters()?;
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        if kind == AccessorKind::Get && !parameters.is_empty() {
            return Err(RaccoonError::new(
                "Getter cannot have parameters",
                position,
                self.file.clone(),
            ));
        }
        if kind == AccessorKind::Set && parameters.len() != 1 {
            return Err(RaccoonError::new(
                "Setter must have exactly one parameter",
                position,
                self.file.clone(),
            ));
        }

        self.consume(TokenType::Colon, "Expected ':' after parameters")?;
        let return_type = self.parse_type()?;

        self.consume(TokenType::LeftBrace, "Expected '{' before accessor body")?;
        let body = self.block_statements()?;

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

    fn parse_decorator(&mut self) -> Result<DecoratorDecl, RaccoonError> {
        let position = self.previous().position;
        let name = self
            .consume(TokenType::Identifier, "Expected decorator name")?
            .value
            .clone();

        let mut args = Vec::new();

        if self.match_token(&[TokenType::LeftParen]) {
            if !self.check(&TokenType::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(
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

    fn function_parameters(&mut self) -> Result<Vec<FnParam>, RaccoonError> {
        let mut params = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if self.check(&TokenType::LeftBracket) || self.check(&TokenType::LeftBrace) {
                    let pattern = VarPattern::Destructuring(self.parse_destructuring_pattern()?);
                    self.consume(TokenType::Colon, "Expected ':' after destructuring pattern")?;
                    let param_type = self.parse_type()?;
                    params.push(FnParam {
                        pattern,
                        param_type,
                        default_value: None,
                        is_rest: false,
                    });
                } else {
                    let name = self
                        .consume(TokenType::Identifier, "Expected parameter name")?
                        .value
                        .clone();
                    self.consume(TokenType::Colon, "Expected ':' after parameter name")?;
                    let param_type = self.parse_type()?;

                    let mut default_value = None;
                    if self.match_token(&[TokenType::Assign]) {
                        default_value = Some(self.expression()?);
                    }

                    params.push(FnParam {
                        pattern: VarPattern::Identifier(name),
                        param_type,
                        default_value,
                        is_rest: false,
                    });
                }

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        Ok(params)
    }

    fn parse_destructuring_pattern(&mut self) -> Result<DestructuringPattern, RaccoonError> {
        if self.match_token(&[TokenType::LeftBracket]) {
            return Ok(DestructuringPattern::List(self.parse_list_pattern()?));
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(DestructuringPattern::Object(self.parse_object_pattern()?));
        }
        Err(RaccoonError::new(
            "Expected destructuring pattern",
            self.peek().position,
            self.file.clone(),
        ))
    }

    fn parse_list_pattern(&mut self) -> Result<ListPattern, RaccoonError> {
        let position = self.previous().position;
        let mut elements = Vec::new();
        let mut rest = None;

        if !self.check(&TokenType::RightBracket) {
            loop {
                if self.match_token(&[TokenType::Dot, TokenType::Dot, TokenType::Dot]) {
                    let name = self
                        .consume(TokenType::Identifier, "Expected identifier after ...")?
                        .value
                        .clone();
                    rest = Some(RestElement {
                        argument: Identifier {
                            name,
                            position: self.previous().position,
                        },
                        position: self.previous().position,
                    });
                    break;
                }

                if self.match_token(&[TokenType::Comma]) {
                    elements.push(None);
                } else if self.check(&TokenType::LeftBracket) {
                    elements.push(Some(ListPatternElement::List(Box::new(
                        self.parse_list_pattern()?,
                    ))));
                } else if self.check(&TokenType::LeftBrace) {
                    elements.push(Some(ListPatternElement::Object(Box::new(
                        self.parse_object_pattern()?,
                    ))));
                } else {
                    let name = self
                        .consume(TokenType::Identifier, "Expected identifier")?
                        .value
                        .clone();
                    elements.push(Some(ListPatternElement::Identifier(Identifier {
                        name,
                        position: self.previous().position,
                    })));
                }

                if !self.match_token(&[TokenType::Comma]) || self.check(&TokenType::RightBracket) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']'")?;
        Ok(ListPattern {
            elements,
            rest,
            position,
        })
    }

    fn parse_object_pattern(&mut self) -> Result<ObjectPattern, RaccoonError> {
        let position = self.previous().position;
        let mut properties = Vec::new();
        let mut rest = None;

        if !self.check(&TokenType::RightBrace) {
            loop {
                if self.match_token(&[TokenType::Dot, TokenType::Dot, TokenType::Dot]) {
                    let name = self
                        .consume(TokenType::Identifier, "Expected identifier after ...")?
                        .value
                        .clone();
                    rest = Some(RestElement {
                        argument: Identifier {
                            name,
                            position: self.previous().position,
                        },
                        position: self.previous().position,
                    });
                    break;
                }

                let key = self
                    .consume(TokenType::Identifier, "Expected property name")?
                    .value
                    .clone();

                let value = if self.match_token(&[TokenType::Colon]) {
                    if self.check(&TokenType::LeftBracket) {
                        ObjectPatternValue::List(self.parse_list_pattern()?)
                    } else if self.check(&TokenType::LeftBrace) {
                        ObjectPatternValue::Object(self.parse_object_pattern()?)
                    } else {
                        let value_name = self
                            .consume(TokenType::Identifier, "Expected identifier")?
                            .value
                            .clone();
                        ObjectPatternValue::Identifier(Identifier {
                            name: value_name,
                            position: self.previous().position,
                        })
                    }
                } else {
                    ObjectPatternValue::Identifier(Identifier {
                        name: key.clone(),
                        position: self.previous().position,
                    })
                };

                properties.push(ObjectPatternProperty { key, value });

                if !self.match_token(&[TokenType::Comma]) || self.check(&TokenType::RightBrace) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}'")?;
        Ok(ObjectPattern {
            properties,
            rest,
            position,
        })
    }

    fn interface_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected interface name")?
            .value
            .clone();
        let position = self.previous().position;

        let type_parameters = self.parse_type_parameters()?;

        let mut extends = Vec::new();
        if self.match_token(&[TokenType::Extends]) {
            loop {
                extends.push(
                    self.consume(TokenType::Identifier, "Expected interface name")?
                        .value
                        .clone(),
                );
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::LeftBrace, "Expected '{' after interface name")?;

        let mut properties = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let prop_name = self
                .consume(TokenType::Identifier, "Expected property name")?
                .value
                .clone();

            if self.match_token(&[TokenType::LeftParen]) {
                let mut param_types = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        if self.check(&TokenType::Identifier)
                            && self.check_next(&[TokenType::Colon])
                        {
                            self.advance();
                            self.advance();
                        }
                        param_types.push(self.parse_type()?);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
                self.consume(TokenType::Colon, "Expected ':' after method signature")?;
                let return_type = self.parse_type()?;

                properties.push(InterfaceProperty {
                    name: prop_name,
                    property_type: Type::Function(Box::new(FunctionType {
                        params: param_types,
                        return_type,
                        is_variadic: false,
                    })),
                    optional: false,
                });
                self.optional_semicolon();
            } else {
                self.consume(TokenType::Colon, "Expected ':' after property name")?;
                let prop_type = self.parse_type()?;
                properties.push(InterfaceProperty {
                    name: prop_name,
                    property_type: prop_type,
                    optional: false,
                });
                self.optional_semicolon();
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after interface body")?;

        Ok(Stmt::InterfaceDecl(InterfaceDecl {
            name,
            type_parameters,
            properties,
            extends,
            position,
        }))
    }

    fn enum_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected enum name")?
            .value
            .clone();
        let position = self.previous().position;

        self.consume(TokenType::LeftBrace, "Expected '{' after enum name")?;

        let mut members = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let member_name = self
                .consume(TokenType::Identifier, "Expected enum member name")?
                .value
                .clone();

            let mut value = None;
            if self.match_token(&[TokenType::Assign]) {
                value = Some(self.expression()?);
            }

            members.push(EnumMember {
                name: member_name,
                value,
            });

            if !self.check(&TokenType::RightBrace) {
                self.consume(TokenType::Comma, "Expected ',' between enum members")?;
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after enum body")?;

        Ok(Stmt::EnumDecl(EnumDecl {
            name,
            members,
            position,
        }))
    }

    fn type_alias_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let name = self
            .consume(TokenType::Identifier, "Expected type alias name")?
            .value
            .clone();
        let position = self.previous().position;

        self.consume(TokenType::Assign, "Expected '=' after type alias name")?;
        let alias_type = self.parse_type()?;

        self.optional_semicolon();

        Ok(Stmt::TypeAliasDecl(TypeAliasDecl {
            name,
            alias_type,
            position,
        }))
    }

    fn import_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        let mut default_import = None;
        let mut namespace_import = None;
        let mut named_imports = Vec::new();

        if self.match_token(&[TokenType::Multiply]) {
            self.consume(TokenType::As, "Expected 'as' after '*'")?;
            namespace_import = Some(
                self.consume(TokenType::Identifier, "Expected namespace name")?
                    .value
                    .clone(),
            );
        } else if self.check(&TokenType::Identifier) {
            default_import = Some(self.advance().value.clone());
            if self.match_token(&[TokenType::Comma]) {
                self.consume(TokenType::LeftBrace, "Expected '{' after default import")?;
                self.parse_named_imports(&mut named_imports)?;
                self.consume(TokenType::RightBrace, "Expected '}' after named imports")?;
            }
        } else if self.match_token(&[TokenType::LeftBrace]) {
            self.parse_named_imports(&mut named_imports)?;
            self.consume(TokenType::RightBrace, "Expected '}' after named imports")?;
        }

        self.consume(TokenType::From, "Expected 'from' after import specifiers")?;
        let module_specifier = self
            .consume(TokenType::StrLiteral, "Expected module path")?
            .value
            .clone();

        self.optional_semicolon();

        Ok(Stmt::ImportDecl(ImportDecl {
            default_import,
            named_imports,
            namespace_import,
            module_specifier,
            position,
        }))
    }

    fn parse_named_imports(
        &mut self,
        named_imports: &mut Vec<ImportSpecifier>,
    ) -> Result<(), RaccoonError> {
        loop {
            let imported = self
                .consume(TokenType::Identifier, "Expected import name")?
                .value
                .clone();
            let mut local = None;
            if self.match_token(&[TokenType::As]) {
                local = Some(
                    self.consume(TokenType::Identifier, "Expected local name after as")?
                        .value
                        .clone(),
                );
            }
            named_imports.push(ImportSpecifier { imported, local });
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        Ok(())
    }

    fn export_declaration(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;

        if self.match_token(&[TokenType::Default]) {
            let declaration = Box::new(self.declaration()?);
            return Ok(Stmt::ExportDecl(ExportDecl {
                declaration: Some(declaration),
                specifiers: Vec::new(),
                is_default: true,
                position,
            }));
        }

        if self.match_token(&[TokenType::LeftBrace]) {
            let mut specifiers = Vec::new();
            loop {
                let local = self
                    .consume(TokenType::Identifier, "Expected export name")?
                    .value
                    .clone();
                let mut exported = None;
                if self.match_token(&[TokenType::As]) {
                    exported = Some(
                        self.consume(TokenType::Identifier, "Expected exported name after as")?
                            .value
                            .clone(),
                    );
                }
                specifiers.push(ExportSpecifier { local, exported });
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
            self.consume(
                TokenType::RightBrace,
                "Expected '}' after export specifiers",
            )?;
            self.optional_semicolon();
            return Ok(Stmt::ExportDecl(ExportDecl {
                declaration: None,
                specifiers,
                is_default: false,
                position,
            }));
        }

        let declaration = Box::new(self.declaration()?);
        Ok(Stmt::ExportDecl(ExportDecl {
            declaration: Some(declaration),
            specifiers: Vec::new(),
            is_default: false,
            position,
        }))
    }

    fn parse_type(&mut self) -> Result<Type, RaccoonError> {
        let mut type_ = self.parse_primary_type()?;

        if self.match_token(&[TokenType::Question]) {
            type_ = Type::Nullable(Box::new(NullableType { inner_type: type_ }));
        }

        while self.match_token(&[TokenType::LeftBracket]) {
            self.consume(TokenType::RightBracket, "Expected ']'")?;
            type_ = Type::List(Box::new(ListType {
                element_type: type_,
            }));
        }

        if self.match_token(&[TokenType::BitwiseOr]) {
            let mut types = vec![type_];
            loop {
                types.push(self.parse_primary_type()?);
                if !self.match_token(&[TokenType::BitwiseOr]) {
                    break;
                }
            }
            type_ = Type::Union(Box::new(UnionType::new(types)));
        }

        Ok(type_)
    }

    fn parse_primary_type(&mut self) -> Result<Type, RaccoonError> {
        if self.check(&TokenType::Identifier) {
            let name = self.advance().value.clone();

            let builtin_type = match name.as_str() {
                "int" => Some(PrimitiveType::int()),
                "float" => Some(PrimitiveType::float()),
                "str" => Some(PrimitiveType::str()),
                "bool" => Some(PrimitiveType::bool()),
                "null" => Some(PrimitiveType::null()),
                "void" => Some(PrimitiveType::void()),
                "any" => Some(PrimitiveType::any()),
                "unknown" => Some(PrimitiveType::unknown()),
                _ => None,
            };

            if let Some(t) = builtin_type {
                return Ok(t);
            }

            if self.match_token(&[TokenType::Lt]) {
                let mut type_args = Vec::new();
                loop {
                    type_args.push(self.parse_type()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
                self.consume(TokenType::Gt, "Expected '>' after type arguments")?;

                if name == "Future" {
                    if type_args.len() != 1 {
                        return Err(RaccoonError::new(
                            "Future requires exactly one type argument",
                            self.previous().position,
                            self.file.clone(),
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
                            self.previous().position,
                            self.file.clone(),
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
                        file: self.file.clone(),
                    }),
                    type_args,
                })));
            }

            return Ok(Type::TypeRef(TypeReference {
                name,
                file: self.file.clone(),
            }));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let mut param_types = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    param_types.push(self.parse_type()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Expected ')'")?;
            self.consume(TokenType::Arrow, "Expected '->'")?;
            let return_type = self.parse_type()?;

            return Ok(Type::Function(Box::new(FunctionType {
                params: param_types,
                return_type,
                is_variadic: false,
            })));
        }

        Err(RaccoonError::new(
            "Expected type",
            self.peek().position,
            self.file.clone(),
        ))
    }

    fn statement(&mut self) -> Result<Stmt, RaccoonError> {
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Block {
                statements: self.block_statements()?,
                position: self.previous().position,
            }));
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(&[TokenType::Try]) {
            return self.try_statement();
        }
        if self.match_token(&[TokenType::Throw]) {
            return self.throw_statement();
        }
        if self.match_token(&[TokenType::Break]) {
            let stmt = Stmt::BreakStmt(BreakStmt {
                position: self.previous().position,
            });
            self.optional_semicolon();
            return Ok(stmt);
        }
        if self.match_token(&[TokenType::Continue]) {
            let stmt = Stmt::ContinueStmt(ContinueStmt {
                position: self.previous().position,
            });
            self.optional_semicolon();
            return Ok(stmt);
        }
        self.expression_statement()
    }

    fn try_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;

        self.consume(TokenType::LeftBrace, "Expected '{' after 'try'")?;
        let try_block = Block {
            statements: self.block_statements()?,
            position,
        };

        let mut catch_clauses = Vec::new();
        while self.match_token(&[TokenType::Catch]) {
            let catch_pos = self.previous().position;
            self.consume(TokenType::LeftParen, "Expected '(' after 'catch'")?;
            let error_var = self
                .consume(TokenType::Identifier, "Expected error variable name")?
                .value
                .clone();

            let mut error_type = None;
            if self.match_token(&[TokenType::Colon]) {
                error_type = Some(self.parse_type()?);
            }

            self.consume(TokenType::RightParen, "Expected ')' after catch parameter")?;
            self.consume(TokenType::LeftBrace, "Expected '{' after catch clause")?;
            let body = Block {
                statements: self.block_statements()?,
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
        if self.match_token(&[TokenType::Finally]) {
            self.consume(TokenType::LeftBrace, "Expected '{' after 'finally'")?;
            finally_block = Some(Block {
                statements: self.block_statements()?,
                position: self.previous().position,
            });
        }

        if catch_clauses.is_empty() && finally_block.is_none() {
            return Err(RaccoonError::new(
                "Try statement must have at least one catch or finally block",
                position,
                self.file.clone(),
            ));
        }

        Ok(Stmt::TryStmt(TryStmt {
            try_block,
            catch_clauses,
            finally_block,
            position,
        }))
    }

    fn throw_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        let value = self.expression()?;
        self.optional_semicolon();
        Ok(Stmt::ThrowStmt(ThrowStmt { value, position }))
    }

    fn block_statements(&mut self) -> Result<Vec<Stmt>, RaccoonError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        Ok(statements)
    }
    fn if_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition")?;

        let then_branch = Box::new(self.statement()?);

        let mut else_branch = None;
        if self.match_token(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::IfStmt(IfStmt {
            condition,
            then_branch,
            else_branch,
            position,
        }))
    }

    fn while_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::WhileStmt(WhileStmt {
            condition,
            body,
            position,
        }))
    }

    fn for_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;

        if self.match_token(&[TokenType::Let, TokenType::Const]) {
            let is_const = self.previous().token_type == TokenType::Const;
            let var_name = self
                .consume(TokenType::Identifier, "Expected variable name")?
                .value
                .clone();

            let mut type_annotation = None;
            if self.match_token(&[TokenType::Colon]) {
                type_annotation = Some(self.parse_type()?);
            }

            if self.match_token(&[TokenType::In]) {
                let iterable = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after iterable")?;
                let body = Box::new(self.statement()?);
                return Ok(Stmt::ForInStmt(ForInStmt {
                    variable: var_name,
                    is_const,
                    type_annotation,
                    iterable,
                    body,
                    position,
                }));
            }

            self.consume(TokenType::Assign, "Expected '=' in variable declaration")?;
            let init_value = self.expression()?;

            let var_decl = Box::new(Stmt::VarDecl(VarDecl {
                pattern: VarPattern::Identifier(var_name),
                type_annotation: type_annotation.unwrap_or(PrimitiveType::any()),
                initializer: Some(init_value),
                is_constant: is_const,
                position,
            }));

            self.optional_semicolon();

            let mut condition = None;
            if !self.check(&TokenType::Semicolon) {
                condition = Some(self.expression()?);
            }
            self.optional_semicolon();

            let mut increment = None;
            if !self.check(&TokenType::RightParen) {
                increment = Some(self.expression()?);
            }
            self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;
            let body = Box::new(self.statement()?);

            return Ok(Stmt::ForStmt(ForStmt {
                initializer: Some(var_decl),
                condition,
                increment,
                body,
                position,
            }));
        }

        let mut initializer = None;
        if !self.match_token(&[TokenType::Semicolon]) {
            initializer = Some(Box::new(self.expression_statement()?));
        }

        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.optional_semicolon();

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::ForStmt(ForStmt {
            initializer,
            condition,
            increment,
            body,
            position,
        }))
    }

    fn return_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let position = self.previous().position;
        let mut value = None;

        if !self.check(&TokenType::Semicolon) && !self.can_insert_semicolon() {
            value = Some(self.expression()?);
        }

        self.optional_semicolon();
        Ok(Stmt::ReturnStmt(ReturnStmt { value, position }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RaccoonError> {
        let expr = self.expression()?;
        let position = expr.position();
        self.optional_semicolon();
        Ok(Stmt::ExprStmt(ExprStmt {
            expression: expr,
            position,
        }))
    }

    fn expression(&mut self) -> Result<Expr, RaccoonError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RaccoonError> {
        // Check for arrow function patterns
        // Case 1: async (params) => ...
        if self.check(&TokenType::Async) {
            let saved_pos = self.current;
            self.advance(); // consume async
            if self.check(&TokenType::LeftParen) {
                if let Ok(arrow) = self.try_parse_arrow_function(true) {
                    return Ok(Expr::ArrowFn(arrow));
                }
            }
            self.current = saved_pos;
        }

        // Case 2: (params) => ...
        if self.check(&TokenType::LeftParen) {
            let saved_pos = self.current;
            if let Ok(arrow) = self.try_parse_arrow_function(false) {
                return Ok(Expr::ArrowFn(arrow));
            }
            self.current = saved_pos;
        }

        let expr = self.conditional()?;

        if self.match_token(&[
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
        ]) {
            let operator = self.previous().token_type;
            let value = Box::new(self.assignment()?);
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

    fn conditional(&mut self) -> Result<Expr, RaccoonError> {
        let expr = self.null_coalescing()?;

        if self.match_token(&[TokenType::Question]) {
            let then_expr = Box::new(self.expression()?);
            self.consume(
                TokenType::Colon,
                "Expected ':' after then branch of conditional expression",
            )?;
            let else_expr = Box::new(self.conditional()?);
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

    fn null_coalescing(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.logical_or()?;

        while self.match_token(&[TokenType::QuestionQuestion]) {
            let right = Box::new(self.logical_or()?);
            let position = expr.position();
            expr = Expr::NullCoalescing(NullCoalescingExpr {
                left: Box::new(expr),
                right,
                position,
            });
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.logical_and()?;

        while self.match_token(&[TokenType::Or]) {
            let right = Box::new(self.logical_and()?);
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

    fn logical_and(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let right = Box::new(self.equality()?);
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

    fn equality(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::Eq, TokenType::Neq]) {
            let operator = if self.previous().token_type == TokenType::Eq {
                BinaryOperator::Equal
            } else {
                BinaryOperator::NotEqual
            };
            let right = Box::new(self.comparison()?);
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

    fn comparison(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.bitwise_or()?;

        while self.match_token(&[TokenType::Lt, TokenType::Gt, TokenType::Lte, TokenType::Gte]) {
            let operator = match self.previous().token_type {
                TokenType::Lt => BinaryOperator::LessThan,
                TokenType::Gt => BinaryOperator::GreaterThan,
                TokenType::Lte => BinaryOperator::LessEqual,
                TokenType::Gte => BinaryOperator::GreaterEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.bitwise_or()?);
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

    fn bitwise_or(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.bitwise_xor()?;

        while self.match_token(&[TokenType::BitwiseOr]) {
            let right = Box::new(self.bitwise_xor()?);
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

    fn bitwise_xor(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.bitwise_and()?;

        while self.match_token(&[TokenType::BitwiseXor]) {
            let right = Box::new(self.bitwise_and()?);
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

    fn bitwise_and(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.shift()?;

        while self.match_token(&[TokenType::Ampersand]) {
            let right = Box::new(self.shift()?);
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

    fn shift(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.range()?;

        while self.match_token(&[
            TokenType::LeftShift,
            TokenType::RightShift,
            TokenType::UnsignedRightShift,
        ]) {
            let operator = match self.previous().token_type {
                TokenType::LeftShift => BinaryOperator::LeftShift,
                TokenType::RightShift => BinaryOperator::RightShift,
                TokenType::UnsignedRightShift => BinaryOperator::UnsignedRightShift,
                _ => unreachable!(),
            };
            let right = Box::new(self.range()?);
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

    fn range(&mut self) -> Result<Expr, RaccoonError> {
        let expr = self.term()?;

        if self.match_token(&[TokenType::Range]) {
            let end = Box::new(self.term()?);
            let position = expr.position();
            return Ok(Expr::Range(RangeExpr {
                start: Box::new(expr),
                end,
                position,
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = if self.previous().token_type == TokenType::Plus {
                BinaryOperator::Add
            } else {
                BinaryOperator::Subtract
            };
            let right = Box::new(self.factor()?);
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

    fn factor(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.exponent()?;

        while self.match_token(&[TokenType::Multiply, TokenType::Divide, TokenType::Modulo]) {
            let operator = match self.previous().token_type {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                TokenType::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = Box::new(self.exponent()?);
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

    fn exponent(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.unary()?;

        // Exponentiation is right-associative
        if self.match_token(&[TokenType::Exponent]) {
            let right = Box::new(self.exponent()?); // Recursive for right-associativity
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

    fn unary(&mut self) -> Result<Expr, RaccoonError> {
        if self.match_token(&[TokenType::Typeof]) {
            let position = self.previous().position;
            let operand = Box::new(self.unary()?);
            return Ok(Expr::TypeOf(TypeOfExpr { operand, position }));
        }

        if self.match_token(&[TokenType::Minus, TokenType::Bang, TokenType::BitwiseNot]) {
            let operator = match self.previous().token_type {
                TokenType::Minus => UnaryOperator::Negate,
                TokenType::Bang => UnaryOperator::Not,
                TokenType::BitwiseNot => UnaryOperator::BitwiseNot,
                _ => unreachable!(),
            };
            let operand = Box::new(self.unary()?);
            let position = self.previous().position;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                operand,
                position,
            }));
        }

        if self.match_token(&[TokenType::Increment, TokenType::Decrement]) {
            let operator = if self.previous().token_type == TokenType::Increment {
                UpdateOperator::Increment
            } else {
                UpdateOperator::Decrement
            };
            let position = self.previous().position;
            let operand = Box::new(self.unary()?);
            return Ok(Expr::UnaryUpdate(UnaryUpdateExpr {
                operator,
                operand,
                is_prefix: true,
                position,
            }));
        }

        if self.match_token(&[TokenType::Await]) {
            let position = self.previous().position;
            let expression = Box::new(self.unary()?);
            return Ok(Expr::Await(AwaitExpr {
                expression,
                position,
            }));
        }

        if self.match_token(&[TokenType::New]) {
            return self.new_expression();
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, RaccoonError> {
        let expr = self.call()?;

        if self.match_token(&[TokenType::Increment, TokenType::Decrement]) {
            let operator = if self.previous().token_type == TokenType::Increment {
                UpdateOperator::Increment
            } else {
                UpdateOperator::Decrement
            };
            let position = self.previous().position;
            return Ok(Expr::UnaryUpdate(UnaryUpdateExpr {
                operator,
                operand: Box::new(expr),
                is_prefix: false,
                position,
            }));
        }

        if self.match_token(&[TokenType::Bang]) {
            let position = self.previous().position;
            return Ok(Expr::NullAssertion(NullAssertionExpr {
                operand: Box::new(expr),
                position,
            }));
        }

        Ok(expr)
    }

    fn new_expression(&mut self) -> Result<Expr, RaccoonError> {
        let position = self.previous().position;
        let class_name = self
            .consume(TokenType::Identifier, "Expected class name after new")?
            .value
            .clone();

        let mut type_args = Vec::new();
        if self.match_token(&[TokenType::Lt]) {
            loop {
                type_args.push(self.parse_type()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
            self.consume(TokenType::Gt, "Expected '>' after type arguments")?;
        }

        // Los paréntesis son opcionales si no hay argumentos
        let mut args = Vec::new();
        if self.match_token(&[TokenType::LeftParen]) {
            if !self.check(&TokenType::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
        }

        Ok(Expr::New(NewExpr {
            class_name,
            type_args,
            args,
            position,
        }))
    }

    fn call(&mut self) -> Result<Expr, RaccoonError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenType::QuestionDot]) {
                let name = self.consume_property_name()?;
                let position = expr.position();
                expr = Expr::OptionalChaining(OptionalChainingExpr {
                    object: Box::new(expr),
                    property: name,
                    position,
                });
            } else if self.match_token(&[TokenType::Dot]) {
                let name = self.consume_property_name()?;
                let position = expr.position();

                if self.check(&TokenType::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Expected ')' after method arguments")?;
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
            } else if self.match_token(&[TokenType::LeftBracket]) {
                let index = Box::new(self.expression()?);
                self.consume(TokenType::RightBracket, "Expected ']' after index")?;
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

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, RaccoonError> {
        let mut args = Vec::new();
        let mut named_args = HashMap::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if self.check(&TokenType::Identifier) && self.check_next(&[TokenType::Colon]) {
                    let name = self.advance().value.clone();
                    self.advance();
                    let value = self.expression()?;
                    named_args.insert(name, value);
                } else {
                    if !named_args.is_empty() {
                        return Err(RaccoonError::new(
                            "Positional arguments must come before named arguments",
                            self.peek().position,
                            self.file.clone(),
                        ));
                    }
                    args.push(self.expression()?);
                }

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
        let position = callee.position();

        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            args,
            named_args,
            position,
        }))
    }

    fn primary(&mut self) -> Result<Expr, RaccoonError> {
        if self.match_token(&[TokenType::This]) {
            return Ok(Expr::This(ThisExpr {
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::Super]) {
            return Ok(Expr::Super(SuperExpr {
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Identifier(Identifier {
                name: self.previous().value.clone(),
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::IntLiteral]) {
            let value = self.previous().value.parse::<i64>().unwrap_or(i64::MAX);
            return Ok(Expr::IntLiteral(IntLiteral {
                value,
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::FloatLiteral]) {
            let value = self.previous().value.parse::<f64>().unwrap();
            return Ok(Expr::FloatLiteral(FloatLiteral {
                value,
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::StrLiteral]) {
            return Ok(Expr::StrLiteral(StrLiteral {
                value: self.previous().value.clone(),
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::BoolLiteral(BoolLiteral {
                value: true,
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::BoolLiteral(BoolLiteral {
                value: false,
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::NullLiteral]) {
            return Ok(Expr::NullLiteral(NullLiteral {
                position: self.previous().position,
            }));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }

        if self.match_token(&[TokenType::LeftBracket]) {
            return self.list_literal();
        }

        if self.match_token(&[TokenType::LeftBrace]) {
            return self.object_literal();
        }

        if self.match_token(&[TokenType::TemplateStrStart]) {
            return self.template_string();
        }

        Err(RaccoonError::new(
            "Expected expression",
            self.peek().position,
            self.file.clone(),
        ))
    }

    fn list_literal(&mut self) -> Result<Expr, RaccoonError> {
        let position = self.previous().position;
        let mut elements = Vec::new();

        if !self.check(&TokenType::RightBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']'")?;
        Ok(Expr::ListLiteral(ListLiteral { elements, position }))
    }

    fn object_literal(&mut self) -> Result<Expr, RaccoonError> {
        let position = self.previous().position;
        let mut properties = HashMap::new();

        if !self.check(&TokenType::RightBrace) {
            loop {
                let key = self
                    .consume(TokenType::Identifier, "Expected property name")?
                    .value
                    .clone();
                self.consume(TokenType::Colon, "Expected ':' after property name")?;
                let value = self.expression()?;
                properties.insert(key, value);

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}'")?;
        Ok(Expr::ObjectLiteral(ObjectLiteral {
            properties,
            position,
        }))
    }

    fn template_string(&mut self) -> Result<Expr, RaccoonError> {
        let position = self.previous().position;
        let mut parts = Vec::new();

        while !self.check(&TokenType::TemplateStrEnd) {
            if self.match_token(&[TokenType::TemplateStrPart]) {
                parts.push(TemplateStrPart::String(StrLiteral {
                    value: self.previous().value.clone(),
                    position: self.previous().position,
                }));
            } else if self.match_token(&[TokenType::TemplateInterpolationStart]) {
                let expr = self.expression()?;
                self.consume(
                    TokenType::TemplateInterpolationEnd,
                    "Expected '}' after template interpolation",
                )?;
                parts.push(TemplateStrPart::Expr(expr));
            } else {
                return Err(RaccoonError::new(
                    "Expected template string part or interpolation",
                    self.peek().position,
                    self.file.clone(),
                ));
            }
        }

        self.consume(TokenType::TemplateStrEnd, "Expected end of template string")?;
        Ok(Expr::TemplateStr(TemplateStrExpr { parts, position }))
    }

    fn optional_semicolon(&mut self) {
        if self.match_token(&[TokenType::Semicolon]) || self.can_insert_semicolon() {
            return;
        }
    }

    fn can_insert_semicolon(&self) -> bool {
        self.check(&TokenType::RightBrace)
            || self.is_at_end()
            || self.previous_token_on_different_line()
    }

    fn previous_token_on_different_line(&self) -> bool {
        if self.current == 0 || self.current >= self.tokens.len() {
            return false;
        }
        self.tokens[self.current].position.0 > self.tokens[self.current - 1].position.0
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == *token_type
    }

    fn check_next(&self, types: &[TokenType]) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        types.contains(&self.tokens[self.current + 1].token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, RaccoonError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(RaccoonError::new(
            message,
            self.peek().position,
            self.file.clone(),
        ))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
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

            self.advance();
        }
    }

    // Helper method to consume a property/method name that might be a keyword
    fn consume_property_name(&mut self) -> Result<String, RaccoonError> {
        let token = self.peek();

        // Allow identifiers and most keywords as property names
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
            | TokenType::Super => {
                let name = self.advance().value.clone();
                Ok(name)
            }
            _ => Err(RaccoonError::new(
                "Expected property name after '.'",
                token.position,
                self.file.clone(),
            )),
        }
    }

    fn try_parse_arrow_function(&mut self, is_async: bool) -> Result<ArrowFnExpr, RaccoonError> {
        let position = self.peek().position;

        // Parse parameters
        self.consume(TokenType::LeftParen, "Expected '('")?;
        let parameters = self.function_parameters()?;
        self.consume(TokenType::RightParen, "Expected ')'")?;

        // Optional return type
        let mut return_type = None;
        if self.match_token(&[TokenType::Colon]) {
            return_type = Some(self.parse_type()?);
        }

        // Must have =>
        if !self.match_token(&[TokenType::Arrow]) {
            return Err(RaccoonError::new(
                "Expected '=>' for arrow function",
                self.peek().position,
                self.file.clone(),
            ));
        }

        // Parse body - either expression or block
        let body = if self.check(&TokenType::LeftBrace) {
            self.advance(); // consume {
            let stmts = self.block_statements()?;
            ArrowFnBody::Block(stmts)
        } else {
            let expr = self.conditional()?;
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
}

impl Expr {
    pub fn position(&self) -> Position {
        match self {
            Expr::Binary(e) => e.position,
            Expr::Unary(e) => e.position,
            Expr::Call(e) => e.position,
            Expr::New(e) => e.position,
            Expr::Member(e) => e.position,
            Expr::MethodCall(e) => e.position,
            Expr::Index(e) => e.position,
            Expr::Await(e) => e.position,
            Expr::This(e) => e.position,
            Expr::Super(e) => e.position,
            Expr::TypeOf(e) => e.position,
            Expr::InstanceOf(e) => e.position,
            Expr::ArrowFn(e) => e.position,
            Expr::Identifier(e) => e.position,
            Expr::Assignment(e) => e.position,
            Expr::Range(e) => e.position,
            Expr::Conditional(e) => e.position,
            Expr::NullCoalescing(e) => e.position,
            Expr::OptionalChaining(e) => e.position,
            Expr::NullAssertion(e) => e.position,
            Expr::UnaryUpdate(e) => e.position,
            Expr::TemplateStr(e) => e.position,
            Expr::TaggedTemplate(e) => e.position,
            Expr::IntLiteral(e) => e.position,
            Expr::FloatLiteral(e) => e.position,
            Expr::StrLiteral(e) => e.position,
            Expr::BoolLiteral(e) => e.position,
            Expr::NullLiteral(e) => e.position,
            Expr::ListLiteral(e) => e.position,
            Expr::ObjectLiteral(e) => e.position,
        }
    }
}
