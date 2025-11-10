use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveContext {
    Math,
    String,
    Array,
    IO,
    HTTP,
    Time,
    JSON,
    System,
    Builtins,
}

impl PrimitiveContext {
    pub fn all() -> &'static [PrimitiveContext] {
        &[
            PrimitiveContext::Math,
            PrimitiveContext::String,
            PrimitiveContext::Array,
            PrimitiveContext::IO,
            PrimitiveContext::HTTP,
            PrimitiveContext::Time,
            PrimitiveContext::JSON,
            PrimitiveContext::System,
            PrimitiveContext::Builtins,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            PrimitiveContext::Math => "math",
            PrimitiveContext::String => "string",
            PrimitiveContext::Array => "array",
            PrimitiveContext::IO => "io",
            PrimitiveContext::HTTP => "http",
            PrimitiveContext::Time => "time",
            PrimitiveContext::JSON => "json",
            PrimitiveContext::System => "system",
            PrimitiveContext::Builtins => "builtins",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "math" => Some(PrimitiveContext::Math),
            "string" => Some(PrimitiveContext::String),
            "array" => Some(PrimitiveContext::Array),
            "io" => Some(PrimitiveContext::IO),
            "http" => Some(PrimitiveContext::HTTP),
            "time" => Some(PrimitiveContext::Time),
            "json" => Some(PrimitiveContext::JSON),
            "system" => Some(PrimitiveContext::System),
            "builtins" => Some(PrimitiveContext::Builtins),
            _ => None,
        }
    }
}

impl fmt::Display for PrimitiveContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
