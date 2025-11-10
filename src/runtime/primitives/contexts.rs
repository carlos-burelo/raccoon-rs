//! Primitive operation contexts
//!
//! This module defines the different contexts in which primitives can operate.
//! Primitives are organized by their operational context rather than by std modules.

use std::fmt;

/// Operational contexts for primitives
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveContext {
    /// Mathematical operations: sqrt, sin, cos, pow, abs, floor, ceil, etc.
    Math,
    /// String operations: len, charAt, substring, toUpper, toLower, trim, etc.
    String,
    /// Array operations: join, sort, reverse, etc.
    Array,
    /// File I/O operations: fileRead, fileWrite, fileAppend, fileExists, etc.
    IO,
    /// HTTP operations: httpGet, httpPost, httpRequest, etc.
    HTTP,
    /// Time operations: timeNow, timeNowMicros, sleep, etc.
    Time,
    /// JSON operations: jsonParse, jsonStringify, etc.
    JSON,
    /// System operations: print, println, envGet, envSet, exit, random, etc.
    System,
    /// Global built-in functions: print, println, eprint, input, len, etc.
    Builtins,
}

impl PrimitiveContext {
    /// Get all available contexts
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

    /// Get the context name as a string
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

    /// Parse a context from a string
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
