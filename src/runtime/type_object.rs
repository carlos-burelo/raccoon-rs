use crate::ast::types::Type;
use crate::runtime::values::RuntimeValue;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TypeObject {
    pub type_def: Type,
    pub kind: TypeKind,
    pub static_methods: HashMap<String, RuntimeValue>,
    pub static_properties: HashMap<String, RuntimeValue>,
    pub constructor: Option<Box<RuntimeValue>>,
    pub metadata: TypeMetadata,
}

impl TypeObject {
    pub fn new(
        type_def: Type,
        kind: TypeKind,
        static_methods: HashMap<String, RuntimeValue>,
        static_properties: HashMap<String, RuntimeValue>,
        constructor: Option<Box<RuntimeValue>>,
        metadata: TypeMetadata,
    ) -> Self {
        Self {
            type_def,
            kind,
            static_methods,
            static_properties,
            constructor,
            metadata,
        }
    }

    pub fn name(&self) -> String {
        self.kind.name()
    }

    pub fn get_static_method(&self, name: &str) -> Option<&RuntimeValue> {
        self.static_methods.get(name)
    }

    pub fn get_static_property(&self, name: &str) -> Option<&RuntimeValue> {
        self.static_properties.get(name)
    }

    pub fn has_static_method(&self, name: &str) -> bool {
        self.static_methods.contains_key(name)
    }

    pub fn has_static_property(&self, name: &str) -> bool {
        self.static_properties.contains_key(name)
    }

    pub fn get_constructor(&self) -> Option<&RuntimeValue> {
        self.constructor.as_ref().map(|b| b.as_ref())
    }

    pub fn get_kind(&self) -> &TypeKind {
        &self.kind
    }

    pub fn get_all_static_methods(&self) -> Vec<String> {
        self.static_methods.keys().cloned().collect()
    }

    pub fn get_all_static_properties(&self) -> Vec<String> {
        self.static_properties.keys().cloned().collect()
    }

    pub fn get_type_def(&self) -> &Type {
        &self.type_def
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self.kind, TypeKind::Primitive(_))
    }

    pub fn is_class(&self) -> bool {
        matches!(self.kind, TypeKind::Class { .. })
    }

    pub fn is_interface(&self) -> bool {
        matches!(self.kind, TypeKind::Interface { .. })
    }

    pub fn is_enum(&self) -> bool {
        matches!(self.kind, TypeKind::Enum { .. })
    }

    pub fn is_function(&self) -> bool {
        matches!(self.kind, TypeKind::Function)
    }

    pub fn get_documentation(&self) -> Option<&String> {
        self.metadata.documentation.as_ref()
    }

    pub fn get_decorators(&self) -> &Vec<String> {
        &self.metadata.decorators
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Primitive(PrimitiveKind),

    Class {
        name: String,
        superclass: Option<String>,
    },

    Interface {
        name: String,
    },

    Enum {
        name: String,
        variants: Vec<String>,
    },

    Function,

    Generic {
        name: String,
        constraints: Vec<Type>,
    },

    Alias {
        name: String,
        target: Box<Type>,
    },

    Module {
        name: String,
    },

    Unknown,
}

impl TypeKind {
    pub fn name(&self) -> String {
        match self {
            TypeKind::Primitive(p) => p.name().to_string(),
            TypeKind::Class { name, .. } => name.clone(),
            TypeKind::Interface { name } => name.clone(),
            TypeKind::Enum { name, .. } => name.clone(),
            TypeKind::Function => "Function".to_string(),
            TypeKind::Generic { name, .. } => name.clone(),
            TypeKind::Alias { name, .. } => name.clone(),
            TypeKind::Module { name } => name.clone(),
            TypeKind::Unknown => "unknown".to_string(),
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            TypeKind::Primitive(_) => "primitive",
            TypeKind::Class { .. } => "class",
            TypeKind::Interface { .. } => "interface",
            TypeKind::Enum { .. } => "enum",
            TypeKind::Function => "function",
            TypeKind::Generic { .. } => "generic",
            TypeKind::Alias { .. } => "alias",
            TypeKind::Module { .. } => "module",
            TypeKind::Unknown => "unknown",
        }
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveKind {
    Int,
    BigInt,
    Float,
    Decimal,
    String,
    Bool,
    Null,
    Void,
    Any,
    Unknown,
    Never,
}

impl PrimitiveKind {
    pub fn name(&self) -> &'static str {
        match self {
            PrimitiveKind::Int => "int",
            PrimitiveKind::BigInt => "bigint",
            PrimitiveKind::Float => "float",
            PrimitiveKind::Decimal => "decimal",
            PrimitiveKind::String => "str",
            PrimitiveKind::Bool => "bool",
            PrimitiveKind::Null => "null",
            PrimitiveKind::Void => "void",
            PrimitiveKind::Any => "any",
            PrimitiveKind::Unknown => "unknown",
            PrimitiveKind::Never => "never",
        }
    }
}

impl fmt::Display for PrimitiveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeMetadata {
    pub documentation: Option<String>,

    pub source_location: Option<SourceLocation>,

    pub decorators: Vec<String>,
}

impl TypeMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_documentation(mut self, doc: String) -> Self {
        self.documentation = Some(doc);
        self
    }

    pub fn with_source_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_decorator(mut self, decorator: String) -> Self {
        self.decorators.push(decorator);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
