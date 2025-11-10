use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

pub fn register_string_module(registrar: &mut Registrar) {
    // length(s: string) -> i32
    registrar.register_fn(
        "length",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            (s.len() as i32).to_raccoon()
        },
        1,
        Some(1),
    );

    // upper(s: string) -> string
    registrar.register_fn(
        "upper",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.to_uppercase().to_raccoon()
        },
        1,
        Some(1),
    );

    // lower(s: string) -> string
    registrar.register_fn(
        "lower",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.to_lowercase().to_raccoon()
        },
        1,
        Some(1),
    );

    // trim(s: string) -> string
    registrar.register_fn(
        "trim",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.trim().to_string().to_raccoon()
        },
        1,
        Some(1),
    );

    // substring(s: string, start: i32, end: i32) -> string
    registrar.register_fn(
        "substring",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let start = i32::from_raccoon(&args[1]).unwrap_or(0) as usize;
            let end = i32::from_raccoon(&args[2]).unwrap_or(s.len() as i32) as usize;

            let start = start.min(s.len());
            let end = end.min(s.len());

            if start <= end {
                s[start..end].to_string().to_raccoon()
            } else {
                String::new().to_raccoon()
            }
        },
        3,
        Some(3),
    );

    // split(s: string, delimiter: string) -> list<string>
    registrar.register_fn(
        "split",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let delimiter = String::from_raccoon(&args[1]).unwrap_or_default();

            let parts: Vec<RuntimeValue> = s
                .split(&delimiter)
                .map(|part| part.to_string().to_raccoon())
                .collect();

            RuntimeValue::Array(crate::runtime::ArrayValue::new(
                parts,
                crate::ast::types::PrimitiveType::str(),
            ))
        },
        2,
        Some(2),
    );

    // replace(s: string, from: string, to: string) -> string
    registrar.register_fn(
        "replace",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let from = String::from_raccoon(&args[1]).unwrap_or_default();
            let to = String::from_raccoon(&args[2]).unwrap_or_default();
            s.replace(&from, &to).to_raccoon()
        },
        3,
        Some(3),
    );

    // contains(s: string, needle: string) -> bool
    registrar.register_fn(
        "contains",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let needle = String::from_raccoon(&args[1]).unwrap_or_default();
            s.contains(&needle).to_raccoon()
        },
        2,
        Some(2),
    );

    // starts_with(s: string, prefix: string) -> bool
    registrar.register_fn(
        "starts_with",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let prefix = String::from_raccoon(&args[1]).unwrap_or_default();
            s.starts_with(&prefix).to_raccoon()
        },
        2,
        Some(2),
    );

    // ends_with(s: string, suffix: string) -> bool
    registrar.register_fn(
        "ends_with",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let suffix = String::from_raccoon(&args[1]).unwrap_or_default();
            s.ends_with(&suffix).to_raccoon()
        },
        2,
        Some(2),
    );

    // reverse(s: string) -> string
    registrar.register_fn(
        "reverse",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.chars().rev().collect::<String>().to_raccoon()
        },
        1,
        Some(1),
    );

    // repeat(s: string, count: i32) -> string
    registrar.register_fn(
        "repeat",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let count = i32::from_raccoon(&args[1]).unwrap_or(0) as usize;
            s.repeat(count).to_raccoon()
        },
        2,
        Some(2),
    );

    // char_at(s: string, index: i32) -> string
    registrar.register_fn(
        "char_at",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let index = i32::from_raccoon(&args[1]).unwrap_or(0) as usize;

            match s.chars().nth(index) {
                Some(ch) => ch.to_string().to_raccoon(),
                None => String::new().to_raccoon(),
            }
        },
        2,
        Some(2),
    );

    // index_of(s: string, needle: string) -> i32
    registrar.register_fn(
        "index_of",
        Some("string"),
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let needle = String::from_raccoon(&args[1]).unwrap_or_default();

            match s.find(&needle) {
                Some(idx) => (idx as i32).to_raccoon(),
                None => (-1i32).to_raccoon(),
            }
        },
        2,
        Some(2),
    );
}
