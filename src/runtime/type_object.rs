use crate::ast::types::Type;
use crate::runtime::values::RuntimeValue;
use std::collections::HashMap;
use std::fmt;

/// Representa cualquier tipo como un objeto de primera clase en runtime
#[derive(Debug, Clone)]
pub struct TypeObject {
    /// La definición del tipo (compile-time)
    pub type_def: Type,

    /// Clasificación del tipo
    pub kind: TypeKind,

    /// Métodos estáticos (Future.resolve, Object.keys, int.parse, etc.)
    pub static_methods: HashMap<String, RuntimeValue>,

    /// Propiedades estáticas (int.MAX_VALUE, str.empty, etc.)
    pub static_properties: HashMap<String, RuntimeValue>,

    /// Constructor del tipo (new MyClass())
    pub constructor: Option<Box<RuntimeValue>>,

    /// Metadata adicional (documentación, anotaciones, etc.)
    pub metadata: TypeMetadata,
}

impl TypeObject {
    /// Crea un nuevo TypeObject
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

    /// Obtiene el nombre del tipo
    pub fn name(&self) -> String {
        self.kind.name()
    }

    /// Obtiene un método estático por nombre
    pub fn get_static_method(&self, name: &str) -> Option<&RuntimeValue> {
        self.static_methods.get(name)
    }

    /// Obtiene una propiedad estática por nombre
    pub fn get_static_property(&self, name: &str) -> Option<&RuntimeValue> {
        self.static_properties.get(name)
    }

    /// Verifica si tiene un método estático
    pub fn has_static_method(&self, name: &str) -> bool {
        self.static_methods.contains_key(name)
    }

    /// Verifica si tiene una propiedad estática
    pub fn has_static_property(&self, name: &str) -> bool {
        self.static_properties.contains_key(name)
    }

    /// Obtiene el constructor
    pub fn get_constructor(&self) -> Option<&RuntimeValue> {
        self.constructor.as_ref().map(|b| b.as_ref())
    }
}

/// Clasificación de tipos
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// Tipos primitivos (int, str, bool, etc.)
    Primitive(PrimitiveKind),

    /// Clase definida por usuario
    Class {
        name: String,
        superclass: Option<String>,
    },

    /// Interface
    Interface {
        name: String,
    },

    /// Enum
    Enum {
        name: String,
        variants: Vec<String>,
    },

    /// Tipo función
    Function,

    /// Tipo genérico no instanciado
    Generic {
        name: String,
        constraints: Vec<Type>,
    },

    /// Alias de tipo
    Alias {
        name: String,
        target: Box<Type>,
    },

    /// Tipo módulo
    Module {
        name: String,
    },

    /// Tipo unknown/any
    Unknown,
}

impl TypeKind {
    /// Obtiene el nombre del tipo
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

    /// Obtiene una descripción legible del kind
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

/// Tipos primitivos del lenguaje
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
    /// Obtiene el nombre del tipo primitivo
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

/// Metadata asociada a un tipo
#[derive(Debug, Clone, Default)]
pub struct TypeMetadata {
    /// Documentación del tipo
    pub documentation: Option<String>,

    /// Ubicación en el código fuente
    pub source_location: Option<SourceLocation>,

    /// Decoradores aplicados al tipo
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

/// Ubicación en el código fuente
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
