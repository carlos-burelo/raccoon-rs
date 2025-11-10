/// Core primitives for the Raccoon standard library
/// These are atomic operations that can only be implemented in Rust
/// All stdlib logic should be implemented in .rcc files using these primitives
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

/// Register all core primitives in the registrar
/// These functions are registered without namespace and with "core_" prefix
/// They can be accessed from "internal:core" module in Raccoon code
pub fn register_core_primitives(registrar: &mut Registrar) {
    // Math primitives
    register_math_primitives(registrar);

    // File I/O primitives
    register_io_primitives(registrar);

    // HTTP primitives
    register_http_primitives(registrar);

    // Time primitives
    register_time_primitives(registrar);

    // String primitives
    register_string_primitives(registrar);

    // Array primitives
    register_array_primitives(registrar);

    // JSON primitives
    register_json_primitives(registrar);

    // System primitives
    register_system_primitives(registrar);
}

// ============================================================================
// Math Primitives
// ============================================================================

fn register_math_primitives(registrar: &mut Registrar) {
    // core_sqrt(x: f64) -> f64
    registrar.register_fn(
        "core_sqrt",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sqrt().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_cbrt(x: f64) -> f64
    registrar.register_fn(
        "core_cbrt",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.cbrt().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_sin(x: f64) -> f64
    registrar.register_fn(
        "core_sin",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sin().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_cos(x: f64) -> f64
    registrar.register_fn(
        "core_cos",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.cos().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_tan(x: f64) -> f64
    registrar.register_fn(
        "core_tan",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.tan().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_asin(x: f64) -> f64
    registrar.register_fn(
        "core_asin",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.asin().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_acos(x: f64) -> f64
    registrar.register_fn(
        "core_acos",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.acos().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_atan(x: f64) -> f64
    registrar.register_fn(
        "core_atan",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.atan().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_atan2(y: f64, x: f64) -> f64
    registrar.register_fn(
        "core_atan2",
        None,
        |args| {
            let y = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let x = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            y.atan2(x).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_sinh(x: f64) -> f64
    registrar.register_fn(
        "core_sinh",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sinh().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_cosh(x: f64) -> f64
    registrar.register_fn(
        "core_cosh",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.cosh().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_tanh(x: f64) -> f64
    registrar.register_fn(
        "core_tanh",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.tanh().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_exp(x: f64) -> f64
    registrar.register_fn(
        "core_exp",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.exp().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_ln(x: f64) -> f64 (natural logarithm)
    registrar.register_fn(
        "core_ln",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
            x.ln().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_log10(x: f64) -> f64
    registrar.register_fn(
        "core_log10",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
            x.log10().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_log(x: f64, base: f64) -> f64
    registrar.register_fn(
        "core_log",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
            let base = f64::from_raccoon(&args[1]).unwrap_or(std::f64::consts::E);
            x.log(base).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_floor(x: f64) -> f64
    registrar.register_fn(
        "core_floor",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.floor().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_ceil(x: f64) -> f64
    registrar.register_fn(
        "core_ceil",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.ceil().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_round(x: f64) -> f64
    registrar.register_fn(
        "core_round",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.round().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_trunc(x: f64) -> f64
    registrar.register_fn(
        "core_trunc",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.trunc().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_abs(x: f64) -> f64
    registrar.register_fn(
        "core_abs",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.abs().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_sign(x: f64) -> f64
    registrar.register_fn(
        "core_sign",
        None,
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            if x > 0.0 {
                1.0_f64.to_raccoon()
            } else if x < 0.0 {
                (-1.0_f64).to_raccoon()
            } else {
                0.0_f64.to_raccoon()
            }
        },
        1,
        Some(1),
    );
}

// ============================================================================
// File I/O Primitives
// ============================================================================

fn register_io_primitives(registrar: &mut Registrar) {
    // core_file_read(path: string) -> string
    registrar.register_fn(
        "core_file_read",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            match std::fs::read_to_string(&path) {
                Ok(content) => content.to_raccoon(),
                Err(_) => "".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );

    // core_file_write(path: string, content: string) -> bool
    registrar.register_fn(
        "core_file_write",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            let content = String::from_raccoon(&args[1]).unwrap_or_default();
            std::fs::write(&path, content).is_ok().to_raccoon()
        },
        2,
        Some(2),
    );

    // core_file_append(path: string, content: string) -> bool
    registrar.register_fn(
        "core_file_append",
        None,
        |args| {
            use std::io::Write;
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            let content = String::from_raccoon(&args[1]).unwrap_or_default();
            match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                Ok(mut file) => file.write_all(content.as_bytes()).is_ok().to_raccoon(),
                Err(_) => false.to_raccoon(),
            }
        },
        2,
        Some(2),
    );

    // core_file_exists(path: string) -> bool
    registrar.register_fn(
        "core_file_exists",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            std::path::Path::new(&path).exists().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_file_delete(path: string) -> bool
    registrar.register_fn(
        "core_file_delete",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            std::fs::remove_file(&path).is_ok().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_dir_create(path: string) -> bool
    registrar.register_fn(
        "core_dir_create",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            std::fs::create_dir_all(&path).is_ok().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_dir_list(path: string) -> string (JSON array)
    registrar.register_fn(
        "core_dir_list",
        None,
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            match std::fs::read_dir(&path) {
                Ok(entries) => {
                    let names: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                        .collect();
                    serde_json::to_string(&names).unwrap_or_else(|_| "[]".to_string()).to_raccoon()
                }
                Err(_) => "[]".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );
}

// ============================================================================
// HTTP Primitives
// ============================================================================

fn register_http_primitives(registrar: &mut Registrar) {
    // core_http_get(url: string) -> string
    registrar.register_fn(
        "core_http_get",
        None,
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();

            // Use ureq for synchronous HTTP requests
            match ureq::get(&url).call() {
                Ok(response) => match response.into_string() {
                    Ok(body) => body.to_raccoon(),
                    Err(_) => "".to_string().to_raccoon(),
                },
                Err(_) => "".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );

    // core_http_post(url: string, body: string) -> string
    registrar.register_fn(
        "core_http_post",
        None,
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            let body = String::from_raccoon(&args[1]).unwrap_or_default();

            match ureq::post(&url).send_string(&body) {
                Ok(response) => match response.into_string() {
                    Ok(body) => body.to_raccoon(),
                    Err(_) => "".to_string().to_raccoon(),
                },
                Err(_) => "".to_string().to_raccoon(),
            }
        },
        2,
        Some(2),
    );

    // core_http_request(method: string, url: string, body: string, headers: string) -> string
    // headers is a JSON string of key-value pairs
    registrar.register_fn(
        "core_http_request",
        None,
        |args| {
            let method = String::from_raccoon(&args[0]).unwrap_or_default();
            let url = String::from_raccoon(&args[1]).unwrap_or_default();
            let body = String::from_raccoon(&args[2]).unwrap_or_default();
            let headers_json = String::from_raccoon(&args[3]).unwrap_or_else(|_| "{}".to_string());

            let agent = ureq::AgentBuilder::new().build();
            let mut request = agent.request(&method, &url);

            // Parse headers
            if let Ok(headers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&headers_json) {
                for (key, value) in headers {
                    if let Some(val_str) = value.as_str() {
                        request = request.set(&key, val_str);
                    }
                }
            }

            let result = if body.is_empty() {
                request.call()
            } else {
                request.send_string(&body)
            };

            match result {
                Ok(response) => match response.into_string() {
                    Ok(body) => body.to_raccoon(),
                    Err(_) => "".to_string().to_raccoon(),
                },
                Err(_) => "".to_string().to_raccoon(),
            }
        },
        4,
        Some(4),
    );
}

// ============================================================================
// Time Primitives
// ============================================================================

fn register_time_primitives(registrar: &mut Registrar) {
    // core_time_now() -> i64 (milliseconds since UNIX epoch)
    registrar.register_fn(
        "core_time_now",
        None,
        |_args| {
            (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64)
                .to_raccoon()
        },
        0,
        Some(0),
    );

    // core_time_now_micros() -> i64 (microseconds since UNIX epoch)
    registrar.register_fn(
        "core_time_now_micros",
        None,
        |_args| {
            (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as i64)
                .to_raccoon()
        },
        0,
        Some(0),
    );

    // core_sleep(ms: i64) -> void
    registrar.register_fn(
        "core_sleep",
        None,
        |args| {
            let ms = i64::from_raccoon(&args[0]).unwrap_or(0);
            if ms > 0 {
                std::thread::sleep(std::time::Duration::from_millis(ms as u64));
            }
            RuntimeValue::Null(crate::runtime::values::NullValue)
        },
        1,
        Some(1),
    );
}

// ============================================================================
// String Primitives
// ============================================================================

fn register_string_primitives(registrar: &mut Registrar) {
    // core_string_len(s: string) -> int
    registrar.register_fn(
        "core_string_len",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            (s.len() as i64).to_raccoon()
        },
        1,
        Some(1),
    );

    // core_string_char_at(s: string, index: int) -> string
    registrar.register_fn(
        "core_string_char_at",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let index = i64::from_raccoon(&args[1]).unwrap_or(0) as usize;
            s.chars().nth(index).map(|c| c.to_string()).unwrap_or_default().to_raccoon()
        },
        2,
        Some(2),
    );

    // core_string_substring(s: string, start: int, end: int) -> string
    registrar.register_fn(
        "core_string_substring",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let start = i64::from_raccoon(&args[1]).unwrap_or(0) as usize;
            let end = i64::from_raccoon(&args[2]).unwrap_or(s.len() as i64) as usize;
            s.chars().skip(start).take(end.saturating_sub(start)).collect::<String>().to_raccoon()
        },
        3,
        Some(3),
    );

    // core_string_to_upper(s: string) -> string
    registrar.register_fn(
        "core_string_to_upper",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.to_uppercase().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_string_to_lower(s: string) -> string
    registrar.register_fn(
        "core_string_to_lower",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.to_lowercase().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_string_trim(s: string) -> string
    registrar.register_fn(
        "core_string_trim",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            s.trim().to_string().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_string_split(s: string, delimiter: string) -> string (JSON array)
    registrar.register_fn(
        "core_string_split",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let delimiter = String::from_raccoon(&args[1]).unwrap_or_default();
            let parts: Vec<String> = if delimiter.is_empty() {
                s.chars().map(|c| c.to_string()).collect()
            } else {
                s.split(&delimiter).map(|s| s.to_string()).collect()
            };
            serde_json::to_string(&parts).unwrap_or_else(|_| "[]".to_string()).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_string_replace(s: string, from: string, to: string) -> string
    registrar.register_fn(
        "core_string_replace",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let from = String::from_raccoon(&args[1]).unwrap_or_default();
            let to = String::from_raccoon(&args[2]).unwrap_or_default();
            s.replace(&from, &to).to_raccoon()
        },
        3,
        Some(3),
    );

    // core_string_starts_with(s: string, prefix: string) -> bool
    registrar.register_fn(
        "core_string_starts_with",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let prefix = String::from_raccoon(&args[1]).unwrap_or_default();
            s.starts_with(&prefix).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_string_ends_with(s: string, suffix: string) -> bool
    registrar.register_fn(
        "core_string_ends_with",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let suffix = String::from_raccoon(&args[1]).unwrap_or_default();
            s.ends_with(&suffix).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_string_contains(s: string, substring: string) -> bool
    registrar.register_fn(
        "core_string_contains",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let substring = String::from_raccoon(&args[1]).unwrap_or_default();
            s.contains(&substring).to_raccoon()
        },
        2,
        Some(2),
    );

    // core_string_index_of(s: string, substring: string) -> int
    registrar.register_fn(
        "core_string_index_of",
        None,
        |args| {
            let s = String::from_raccoon(&args[0]).unwrap_or_default();
            let substring = String::from_raccoon(&args[1]).unwrap_or_default();
            s.find(&substring).map(|i| i as i64).unwrap_or(-1).to_raccoon()
        },
        2,
        Some(2),
    );
}

// ============================================================================
// Array Primitives
// ============================================================================

fn register_array_primitives(registrar: &mut Registrar) {
    // core_array_join(array: string, separator: string) -> string
    // array is a JSON string
    registrar.register_fn(
        "core_array_join",
        None,
        |args| {
            let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
            let separator = String::from_raccoon(&args[1]).unwrap_or_default();

            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
                let strings: Vec<String> = arr.iter().map(|v| {
                    match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    }
                }).collect();
                strings.join(&separator).to_raccoon()
            } else {
                "".to_string().to_raccoon()
            }
        },
        2,
        Some(2),
    );

    // core_array_sort(array: string) -> string (JSON array)
    registrar.register_fn(
        "core_array_sort",
        None,
        |args| {
            let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());

            if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
                arr.sort_by(|a, b| {
                    // Simple comparison for numbers and strings
                    match (a, b) {
                        (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => {
                            n1.as_f64().partial_cmp(&n2.as_f64()).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (serde_json::Value::String(s1), serde_json::Value::String(s2)) => s1.cmp(s2),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string()).to_raccoon()
            } else {
                "[]".to_string().to_raccoon()
            }
        },
        1,
        Some(1),
    );

    // core_array_reverse(array: string) -> string (JSON array)
    registrar.register_fn(
        "core_array_reverse",
        None,
        |args| {
            let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());

            if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
                arr.reverse();
                serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string()).to_raccoon()
            } else {
                "[]".to_string().to_raccoon()
            }
        },
        1,
        Some(1),
    );
}

// ============================================================================
// JSON Primitives
// ============================================================================

fn register_json_primitives(registrar: &mut Registrar) {
    // core_json_parse(json: string) -> string
    registrar.register_fn(
        "core_json_parse",
        None,
        |args| {
            let json = String::from_raccoon(&args[0]).unwrap_or_default();
            // Validate JSON by parsing and re-stringifying
            match serde_json::from_str::<serde_json::Value>(&json) {
                Ok(value) => serde_json::to_string(&value).unwrap_or_else(|_| "null".to_string()).to_raccoon(),
                Err(_) => "null".to_string().to_raccoon(),
            }
        },
        1,
        Some(1),
    );

    // core_json_stringify(value: string) -> string
    // Since we can't easily convert RuntimeValue to JSON here, we assume input is already JSON-compatible
    registrar.register_fn(
        "core_json_stringify",
        None,
        |args| {
            // Just return the string representation
            let value = String::from_raccoon(&args[0]).unwrap_or_default();
            value.to_raccoon()
        },
        1,
        Some(1),
    );
}

// ============================================================================
// System Primitives
// ============================================================================

fn register_system_primitives(registrar: &mut Registrar) {
    // core_print(message: string) -> void
    registrar.register_fn(
        "core_print",
        None,
        |args| {
            let message = String::from_raccoon(&args[0]).unwrap_or_default();
            print!("{}", message);
            RuntimeValue::Null(crate::runtime::values::NullValue)
        },
        1,
        Some(1),
    );

    // core_println(message: string) -> void
    registrar.register_fn(
        "core_println",
        None,
        |args| {
            let message = String::from_raccoon(&args[0]).unwrap_or_default();
            println!("{}", message);
            RuntimeValue::Null(crate::runtime::values::NullValue)
        },
        1,
        Some(1),
    );

    // core_env_get(name: string) -> string
    registrar.register_fn(
        "core_env_get",
        None,
        |args| {
            let name = String::from_raccoon(&args[0]).unwrap_or_default();
            std::env::var(&name).unwrap_or_default().to_raccoon()
        },
        1,
        Some(1),
    );

    // core_env_set(name: string, value: string) -> bool
    registrar.register_fn(
        "core_env_set",
        None,
        |args| {
            let name = String::from_raccoon(&args[0]).unwrap_or_default();
            let value = String::from_raccoon(&args[1]).unwrap_or_default();
            std::env::set_var(&name, &value);
            true.to_raccoon()
        },
        2,
        Some(2),
    );

    // core_exit(code: int) -> void
    registrar.register_fn(
        "core_exit",
        None,
        |args| {
            let code = i32::from_raccoon(&args[0]).unwrap_or(0);
            std::process::exit(code);
        },
        1,
        Some(1),
    );

    // core_random() -> f64 (random number between 0 and 1)
    registrar.register_fn(
        "core_random",
        None,
        |_args| {
            use std::collections::hash_map::RandomState;
            use std::hash::{BuildHasher, Hasher};

            let random_state = RandomState::new();
            let mut hasher = random_state.build_hasher();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            hasher.write_u128(now);
            let hash = hasher.finish();

            // Convert to f64 between 0 and 1
            (hash as f64 / u64::MAX as f64).to_raccoon()
        },
        0,
        Some(0),
    );
}
