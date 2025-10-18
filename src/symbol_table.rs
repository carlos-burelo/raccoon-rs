use crate::{
    ast::{nodes::*, types::*},
    error::RaccoonError,
    runtime::RuntimeValue,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Class,
    Interface,
    Enum,
    TypeAlias,
    Parameter,
    Property,
    Method,
}

#[derive(Debug, Clone)]
pub struct SymbolItem {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Type,
    pub is_constant: bool,
    pub declaration: Option<Box<Stmt>>,
    pub value: Option<RuntimeValue>, // Añadido para almacenar valores en runtime
}

impl SymbolItem {
    pub fn new(
        name: String,
        kind: SymbolKind,
        symbol_type: Type,
        is_constant: bool,
        declaration: Option<Box<Stmt>>,
    ) -> Self {
        Self {
            name,
            kind,
            symbol_type,
            is_constant,
            declaration,
            value: None,
        }
    }

    pub fn with_value(mut self, value: RuntimeValue) -> Self {
        self.value = Some(value);
        self
    }
}

pub struct SymbolTable {
    file: Option<String>,
    scopes: Vec<HashMap<String, SymbolItem>>,
}

impl SymbolTable {
    pub fn new(file: Option<String>) -> Self {
        let mut table = Self {
            file,
            scopes: Vec::new(),
        };

        table.scopes.push(HashMap::new());
        table.initialize_builtin_types();
        table
    }

    fn initialize_builtin_types(&mut self) {
        let builtin_types = vec![
            ("int", PrimitiveType::int()),
            ("float", PrimitiveType::float()),
            (
                "i8",
                Type::Primitive(PrimitiveType::new(TypeKind::I8, "i8")),
            ),
            (
                "i16",
                Type::Primitive(PrimitiveType::new(TypeKind::I16, "i16")),
            ),
            (
                "i32",
                Type::Primitive(PrimitiveType::new(TypeKind::I32, "i32")),
            ),
            (
                "i64",
                Type::Primitive(PrimitiveType::new(TypeKind::I64, "i64")),
            ),
            (
                "u8",
                Type::Primitive(PrimitiveType::new(TypeKind::U8, "u8")),
            ),
            (
                "u16",
                Type::Primitive(PrimitiveType::new(TypeKind::U16, "u16")),
            ),
            (
                "u32",
                Type::Primitive(PrimitiveType::new(TypeKind::U32, "u32")),
            ),
            (
                "u64",
                Type::Primitive(PrimitiveType::new(TypeKind::U64, "u64")),
            ),
            (
                "f32",
                Type::Primitive(PrimitiveType::new(TypeKind::F32, "f32")),
            ),
            (
                "f64",
                Type::Primitive(PrimitiveType::new(TypeKind::F64, "f64")),
            ),
            (
                "decimal",
                Type::Primitive(PrimitiveType::new(TypeKind::Decimal, "decimal")),
            ),
            ("str", PrimitiveType::str()),
            ("bool", PrimitiveType::bool()),
            ("null", PrimitiveType::null()),
            ("void", PrimitiveType::void()),
            ("any", PrimitiveType::any()),
            ("unknown", PrimitiveType::unknown()),
            (
                "symbol",
                Type::Primitive(PrimitiveType::new(TypeKind::Symbol, "symbol")),
            ),
            (
                "Date",
                Type::Primitive(PrimitiveType::new(TypeKind::Date, "Date")),
            ),
            (
                "Regex",
                Type::Primitive(PrimitiveType::new(TypeKind::Regex, "Regex")),
            ),
            (
                "Error",
                Type::Primitive(PrimitiveType::new(TypeKind::Error, "Error")),
            ),
        ];

        for (name, type_) in builtin_types {
            self.define(name.to_string(), SymbolKind::TypeAlias, type_, false, None);
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(
        &mut self,
        name: String,
        kind: SymbolKind,
        symbol_type: Type,
        is_constant: bool,
        declaration: Option<Box<Stmt>>,
    ) -> SymbolItem {
        let symbol = SymbolItem::new(name.clone(), kind, symbol_type, is_constant, declaration);

        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, symbol.clone());
        }

        symbol
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolItem> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn lookup_current_scope(&self, name: &str) -> Option<&SymbolItem> {
        self.scopes.last().and_then(|scope| scope.get(name))
    }

    pub fn update(&mut self, name: &str, new_type: Type) -> Result<(), RaccoonError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(name) {
                if symbol.is_constant {
                    return Err(RaccoonError::new(
                        format!("Cannot reassign constant '{}'", name),
                        (0, 0),
                        self.file.clone(),
                    ));
                }
                symbol.symbol_type = new_type;
                return Ok(());
            }
        }
        Err(RaccoonError::new(
            format!("Undefined variable '{}'", name),
            (0, 0),
            self.file.clone(),
        ))
    }

    pub fn update_value(&mut self, name: &str, value: RuntimeValue) -> Result<(), RaccoonError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(name) {
                if symbol.is_constant {
                    return Err(RaccoonError::new(
                        format!("Cannot reassign constant '{}'", name),
                        (0, 0),
                        self.file.clone(),
                    ));
                }
                symbol.value = Some(value);
                return Ok(());
            }
        }
        Err(RaccoonError::new(
            format!("Undefined variable '{}'", name),
            (0, 0),
            self.file.clone(),
        ))
    }

    pub fn define_with_value(
        &mut self,
        name: String,
        kind: SymbolKind,
        symbol_type: Type,
        value: RuntimeValue,
        is_constant: bool,
        declaration: Option<Box<Stmt>>,
    ) -> SymbolItem {
        let mut symbol = SymbolItem::new(name.clone(), kind, symbol_type, is_constant, declaration);
        symbol.value = Some(value);

        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, symbol.clone());
        }

        symbol
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut SymbolItem> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                return scope.get_mut(name);
            }
        }
        None
    }

    pub fn is_defined_in_current_scope(&self, name: &str) -> bool {
        self.lookup_current_scope(name).is_some()
    }

    pub fn get_scope_depth(&self) -> usize {
        self.scopes.len()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new(Option::None)
    }
}
