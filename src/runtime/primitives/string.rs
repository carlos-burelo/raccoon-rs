//! String context primitives
//! Low-level string operations

use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

// Get string length
primitive! {
    string::core_string_len(s: String) -> i64 {
        s.len() as i64
    }
}

// Get character at index
pub fn core_string_char_at(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let index = i64::from_raccoon(&args[1]).unwrap_or(0) as usize;
    s.chars()
        .nth(index)
        .map(|c| c.to_string())
        .unwrap_or_default()
        .to_raccoon()
}

// Get substring
pub fn core_string_substring(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let start = i64::from_raccoon(&args[1]).unwrap_or(0) as usize;
    let end = i64::from_raccoon(&args[2]).unwrap_or(s.len() as i64) as usize;
    s.chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect::<String>()
        .to_raccoon()
}

// Convert to uppercase
primitive! {
    string::core_string_to_upper(s: String) -> String {
        s.to_uppercase()
    }
}

// Convert to lowercase
primitive! {
    string::core_string_to_lower(s: String) -> String {
        s.to_lowercase()
    }
}

// Trim whitespace
primitive! {
    string::core_string_trim(s: String) -> String {
        s.trim().to_string()
    }
}

// Split string
pub fn core_string_split(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let delimiter = String::from_raccoon(&args[1]).unwrap_or_default();
    let parts: Vec<String> = if delimiter.is_empty() {
        s.chars().map(|c| c.to_string()).collect()
    } else {
        s.split(&delimiter).map(|s| s.to_string()).collect()
    };
    serde_json::to_string(&parts)
        .unwrap_or_else(|_| "[]".to_string())
        .to_raccoon()
}

// Replace all occurrences
pub fn core_string_replace(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let from = String::from_raccoon(&args[1]).unwrap_or_default();
    let to = String::from_raccoon(&args[2]).unwrap_or_default();
    s.replace(&from, &to).to_raccoon()
}

// Check if string starts with prefix
primitive! {
    string::core_string_starts_with(s: String, prefix: String) -> bool {
        s.starts_with(&prefix)
    }
}

// Check if string ends with suffix
primitive! {
    string::core_string_ends_with(s: String, suffix: String) -> bool {
        s.ends_with(&suffix)
    }
}

// Check if string contains substring
primitive! {
    string::core_string_contains(s: String, substring: String) -> bool {
        s.contains(&substring)
    }
}

// Find index of substring
primitive! {
    string::core_string_index_of(s: String, substring: String) -> i64 {
        s.find(&substring)
            .map(|i| i as i64)
            .unwrap_or(-1)
    }
}

/// Register all string primitives
pub fn register_string_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, string, {
        core_string_len: 1..=1,
        core_string_char_at: 2..=2,
        core_string_substring: 3..=3,
        core_string_to_upper: 1..=1,
        core_string_to_lower: 1..=1,
        core_string_trim: 1..=1,
        core_string_split: 2..=2,
        core_string_replace: 3..=3,
        core_string_starts_with: 2..=2,
        core_string_ends_with: 2..=2,
        core_string_contains: 2..=2,
        core_string_index_of: 2..=2,
    });
}
