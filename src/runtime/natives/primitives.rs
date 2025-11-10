use crate::runtime::values::NullValue;

use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

pub fn core_sqrt(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.sqrt().to_raccoon()
}

pub fn core_cbrt(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.cbrt().to_raccoon()
}

pub fn core_sin(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.sin().to_raccoon()
}

pub fn core_cos(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.cos().to_raccoon()
}

pub fn core_tan(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.tan().to_raccoon()
}

pub fn core_asin(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.asin().to_raccoon()
}

pub fn core_acos(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.acos().to_raccoon()
}

pub fn core_atan(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.atan().to_raccoon()
}

pub fn core_atan2(args: Vec<RuntimeValue>) -> RuntimeValue {
    let y = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    let x = f64::from_raccoon(&args[1]).unwrap_or(0.0);
    y.atan2(x).to_raccoon()
}

pub fn core_sinh(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.sinh().to_raccoon()
}

pub fn core_cosh(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.cosh().to_raccoon()
}

pub fn core_tanh(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.tanh().to_raccoon()
}

pub fn core_exp(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.exp().to_raccoon()
}

pub fn core_ln(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
    x.ln().to_raccoon()
}

pub fn core_log10(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
    x.log10().to_raccoon()
}

pub fn core_log(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
    let base = f64::from_raccoon(&args[1]).unwrap_or(std::f64::consts::E);
    x.log(base).to_raccoon()
}

pub fn core_floor(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.floor().to_raccoon()
}

pub fn core_ceil(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.ceil().to_raccoon()
}

pub fn core_round(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.round().to_raccoon()
}

pub fn core_trunc(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.trunc().to_raccoon()
}

pub fn core_abs(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.abs().to_raccoon()
}

pub fn core_sign(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    if x > 0.0 {
        1.0_f64.to_raccoon()
    } else if x < 0.0 {
        (-1.0_f64).to_raccoon()
    } else {
        0.0_f64.to_raccoon()
    }
}

pub fn core_pow(args: Vec<RuntimeValue>) -> RuntimeValue {
    let base = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    let exp = f64::from_raccoon(&args[1]).unwrap_or(0.0);
    base.powf(exp).to_raccoon()
}

pub fn core_file_read(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    match std::fs::read_to_string(&path) {
        Ok(content) => content.to_raccoon(),
        Err(_) => "".to_string().to_raccoon(),
    }
}

pub fn core_file_write(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    let content = String::from_raccoon(&args[1]).unwrap_or_default();
    std::fs::write(&path, content).is_ok().to_raccoon()
}

pub fn core_file_append(args: Vec<RuntimeValue>) -> RuntimeValue {
    use std::io::Write;
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    let content = String::from_raccoon(&args[1]).unwrap_or_default();
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        Ok(mut file) => file.write_all(content.as_bytes()).is_ok().to_raccoon(),
        Err(_) => false.to_raccoon(),
    }
}

pub fn core_file_exists(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    std::path::Path::new(&path).exists().to_raccoon()
}

pub fn core_file_delete(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    std::fs::remove_file(&path).is_ok().to_raccoon()
}

pub fn core_dir_create(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    std::fs::create_dir_all(&path).is_ok().to_raccoon()
}

pub fn core_dir_list(args: Vec<RuntimeValue>) -> RuntimeValue {
    let path = String::from_raccoon(&args[0]).unwrap_or_default();
    match std::fs::read_dir(&path) {
        Ok(entries) => {
            let names: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                .collect();
            serde_json::to_string(&names)
                .unwrap_or_else(|_| "[]".to_string())
                .to_raccoon()
        }
        Err(_) => "[]".to_string().to_raccoon(),
    }
}

pub fn core_http_get(args: Vec<RuntimeValue>) -> RuntimeValue {
    let url = String::from_raccoon(&args[0]).unwrap_or_default();
    match ureq::get(&url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => body.to_raccoon(),
            Err(_) => "".to_string().to_raccoon(),
        },
        Err(_) => "".to_string().to_raccoon(),
    }
}

pub fn core_http_post(args: Vec<RuntimeValue>) -> RuntimeValue {
    let url = String::from_raccoon(&args[0]).unwrap_or_default();
    let body = String::from_raccoon(&args[1]).unwrap_or_default();
    match ureq::post(&url).send_string(&body) {
        Ok(response) => match response.into_string() {
            Ok(body) => body.to_raccoon(),
            Err(_) => "".to_string().to_raccoon(),
        },
        Err(_) => "".to_string().to_raccoon(),
    }
}

pub fn core_http_request(args: Vec<RuntimeValue>) -> RuntimeValue {
    let method = String::from_raccoon(&args[0]).unwrap_or_default();
    let url = String::from_raccoon(&args[1]).unwrap_or_default();
    let body = String::from_raccoon(&args[2]).unwrap_or_default();
    let headers_json = String::from_raccoon(&args[3]).unwrap_or_else(|_| "{}".to_string());

    let agent = ureq::AgentBuilder::new().build();
    let mut request = agent.request(&method, &url);

    if let Ok(headers) =
        serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&headers_json)
    {
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
}

pub fn core_time_now(_args: Vec<RuntimeValue>) -> RuntimeValue {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64)
        .to_raccoon()
}

pub fn core_time_now_micros(_args: Vec<RuntimeValue>) -> RuntimeValue {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as i64)
        .to_raccoon()
}

pub fn core_sleep(args: Vec<RuntimeValue>) -> RuntimeValue {
    let ms = i64::from_raccoon(&args[0]).unwrap_or(0);
    if ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
    RuntimeValue::Null(NullValue)
}

pub fn core_string_len(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    (s.len() as i64).to_raccoon()
}

pub fn core_string_char_at(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let index = i64::from_raccoon(&args[1]).unwrap_or(0) as usize;
    s.chars()
        .nth(index)
        .map(|c| c.to_string())
        .unwrap_or_default()
        .to_raccoon()
}

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

pub fn core_string_to_upper(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    s.to_uppercase().to_raccoon()
}

pub fn core_string_to_lower(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    s.to_lowercase().to_raccoon()
}

pub fn core_string_trim(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    s.trim().to_string().to_raccoon()
}

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

pub fn core_string_replace(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let from = String::from_raccoon(&args[1]).unwrap_or_default();
    let to = String::from_raccoon(&args[2]).unwrap_or_default();
    s.replace(&from, &to).to_raccoon()
}

pub fn core_string_starts_with(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let prefix = String::from_raccoon(&args[1]).unwrap_or_default();
    s.starts_with(&prefix).to_raccoon()
}

pub fn core_string_ends_with(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let suffix = String::from_raccoon(&args[1]).unwrap_or_default();
    s.ends_with(&suffix).to_raccoon()
}

pub fn core_string_contains(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let substring = String::from_raccoon(&args[1]).unwrap_or_default();
    s.contains(&substring).to_raccoon()
}

pub fn core_string_index_of(args: Vec<RuntimeValue>) -> RuntimeValue {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    let substring = String::from_raccoon(&args[1]).unwrap_or_default();
    s.find(&substring)
        .map(|i| i as i64)
        .unwrap_or(-1)
        .to_raccoon()
}

pub fn core_array_join(args: Vec<RuntimeValue>) -> RuntimeValue {
    let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
    let separator = String::from_raccoon(&args[1]).unwrap_or_default();

    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
        let strings: Vec<String> = arr
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                _ => v.to_string(),
            })
            .collect();
        strings.join(&separator).to_raccoon()
    } else {
        "".to_string().to_raccoon()
    }
}

pub fn core_array_sort(args: Vec<RuntimeValue>) -> RuntimeValue {
    let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());

    if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
        arr.sort_by(|a, b| match (a, b) {
            (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => n1
                .as_f64()
                .partial_cmp(&n2.as_f64())
                .unwrap_or(std::cmp::Ordering::Equal),
            (serde_json::Value::String(s1), serde_json::Value::String(s2)) => s1.cmp(s2),
            _ => std::cmp::Ordering::Equal,
        });
        serde_json::to_string(&arr)
            .unwrap_or_else(|_| "[]".to_string())
            .to_raccoon()
    } else {
        "[]".to_string().to_raccoon()
    }
}

pub fn core_array_reverse(args: Vec<RuntimeValue>) -> RuntimeValue {
    let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());

    if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
        arr.reverse();
        serde_json::to_string(&arr)
            .unwrap_or_else(|_| "[]".to_string())
            .to_raccoon()
    } else {
        "[]".to_string().to_raccoon()
    }
}

pub fn core_json_parse(args: Vec<RuntimeValue>) -> RuntimeValue {
    let json = String::from_raccoon(&args[0]).unwrap_or_default();
    match serde_json::from_str::<serde_json::Value>(&json) {
        Ok(value) => serde_json::to_string(&value)
            .unwrap_or_else(|_| "null".to_string())
            .to_raccoon(),
        Err(_) => "null".to_string().to_raccoon(),
    }
}

pub fn core_json_stringify(args: Vec<RuntimeValue>) -> RuntimeValue {
    let value = String::from_raccoon(&args[0]).unwrap_or_default();
    value.to_raccoon()
}

pub fn core_print(args: Vec<RuntimeValue>) -> RuntimeValue {
    let message = String::from_raccoon(&args[0]).unwrap_or_default();
    print!("{}", message);
    RuntimeValue::Null(NullValue)
}

pub fn core_println(args: Vec<RuntimeValue>) -> RuntimeValue {
    let message = String::from_raccoon(&args[0]).unwrap_or_default();
    println!("{}", message);
    RuntimeValue::Null(NullValue)
}

pub fn core_eprint(args: Vec<RuntimeValue>) -> RuntimeValue {
    let message = String::from_raccoon(&args[0]).unwrap_or_default();
    eprint!("{}", message);
    RuntimeValue::Null(NullValue)
}

pub fn core_input(args: Vec<RuntimeValue>) -> RuntimeValue {
    use std::io::{self, Write};

    let prompt = if !args.is_empty() {
        String::from_raccoon(&args[0]).unwrap_or_default()
    } else {
        String::new()
    };

    if !prompt.is_empty() {
        print!("{}", prompt);
        let _ = io::stdout().flush();
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim_end_matches(&['\r', '\n'][..]).to_string();
            trimmed.to_raccoon()
        }
        Err(_) => "".to_string().to_raccoon(),
    }
}

pub fn core_len(args: Vec<RuntimeValue>) -> RuntimeValue {
    use crate::runtime::IntValue;

    if args.is_empty() {
        return RuntimeValue::Int(IntValue::new(0));
    }

    match &args[0] {
        RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
        RuntimeValue::Array(a) => RuntimeValue::Int(IntValue::new(a.elements.len() as i64)),
        RuntimeValue::Map(m) => RuntimeValue::Int(IntValue::new(m.entries.len() as i64)),
        _ => RuntimeValue::Int(IntValue::new(0)),
    }
}

pub fn core_env_get(args: Vec<RuntimeValue>) -> RuntimeValue {
    let name = String::from_raccoon(&args[0]).unwrap_or_default();
    std::env::var(&name).unwrap_or_default().to_raccoon()
}

pub fn core_env_set(args: Vec<RuntimeValue>) -> RuntimeValue {
    let name = String::from_raccoon(&args[0]).unwrap_or_default();
    let value = String::from_raccoon(&args[1]).unwrap_or_default();
    std::env::set_var(&name, &value);
    true.to_raccoon()
}

pub fn core_exit(args: Vec<RuntimeValue>) -> RuntimeValue {
    let code = i32::from_raccoon(&args[0]).unwrap_or(0);
    std::process::exit(code);
}

pub fn core_random(_args: Vec<RuntimeValue>) -> RuntimeValue {
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

    (hash as f64 / u64::MAX as f64).to_raccoon()
}

pub fn register_core_primitives(registrar: &mut Registrar) {
    registrar.register_fn("core_sqrt", None, core_sqrt, 1, Some(1));
    registrar.register_fn("core_cbrt", None, core_cbrt, 1, Some(1));
    registrar.register_fn("core_sin", None, core_sin, 1, Some(1));
    registrar.register_fn("core_cos", None, core_cos, 1, Some(1));
    registrar.register_fn("core_tan", None, core_tan, 1, Some(1));
    registrar.register_fn("core_asin", None, core_asin, 1, Some(1));
    registrar.register_fn("core_acos", None, core_acos, 1, Some(1));
    registrar.register_fn("core_atan", None, core_atan, 1, Some(1));
    registrar.register_fn("core_atan2", None, core_atan2, 2, Some(2));
    registrar.register_fn("core_sinh", None, core_sinh, 1, Some(1));
    registrar.register_fn("core_cosh", None, core_cosh, 1, Some(1));
    registrar.register_fn("core_tanh", None, core_tanh, 1, Some(1));
    registrar.register_fn("core_exp", None, core_exp, 1, Some(1));
    registrar.register_fn("core_ln", None, core_ln, 1, Some(1));
    registrar.register_fn("core_log10", None, core_log10, 1, Some(1));
    registrar.register_fn("core_log", None, core_log, 2, Some(2));
    registrar.register_fn("core_floor", None, core_floor, 1, Some(1));
    registrar.register_fn("core_ceil", None, core_ceil, 1, Some(1));
    registrar.register_fn("core_round", None, core_round, 1, Some(1));
    registrar.register_fn("core_trunc", None, core_trunc, 1, Some(1));
    registrar.register_fn("core_abs", None, core_abs, 1, Some(1));
    registrar.register_fn("core_sign", None, core_sign, 1, Some(1));
    registrar.register_fn("core_pow", None, core_pow, 2, Some(2));

    registrar.register_fn("core_file_read", None, core_file_read, 1, Some(1));
    registrar.register_fn("core_file_write", None, core_file_write, 2, Some(2));
    registrar.register_fn("core_file_append", None, core_file_append, 2, Some(2));
    registrar.register_fn("core_file_exists", None, core_file_exists, 1, Some(1));
    registrar.register_fn("core_file_delete", None, core_file_delete, 1, Some(1));
    registrar.register_fn("core_dir_create", None, core_dir_create, 1, Some(1));
    registrar.register_fn("core_dir_list", None, core_dir_list, 1, Some(1));

    registrar.register_fn("core_http_get", None, core_http_get, 1, Some(1));
    registrar.register_fn("core_http_post", None, core_http_post, 2, Some(2));
    registrar.register_fn("core_http_request", None, core_http_request, 4, Some(4));

    registrar.register_fn("core_time_now", None, core_time_now, 0, Some(0));
    registrar.register_fn(
        "core_time_now_micros",
        None,
        core_time_now_micros,
        0,
        Some(0),
    );
    registrar.register_fn("core_sleep", None, core_sleep, 1, Some(1));

    registrar.register_fn("core_string_len", None, core_string_len, 1, Some(1));
    registrar.register_fn("core_string_char_at", None, core_string_char_at, 2, Some(2));
    registrar.register_fn(
        "core_string_substring",
        None,
        core_string_substring,
        3,
        Some(3),
    );
    registrar.register_fn(
        "core_string_to_upper",
        None,
        core_string_to_upper,
        1,
        Some(1),
    );
    registrar.register_fn(
        "core_string_to_lower",
        None,
        core_string_to_lower,
        1,
        Some(1),
    );
    registrar.register_fn("core_string_trim", None, core_string_trim, 1, Some(1));
    registrar.register_fn("core_string_split", None, core_string_split, 2, Some(2));
    registrar.register_fn("core_string_replace", None, core_string_replace, 3, Some(3));
    registrar.register_fn(
        "core_string_starts_with",
        None,
        core_string_starts_with,
        2,
        Some(2),
    );
    registrar.register_fn(
        "core_string_ends_with",
        None,
        core_string_ends_with,
        2,
        Some(2),
    );
    registrar.register_fn(
        "core_string_contains",
        None,
        core_string_contains,
        2,
        Some(2),
    );
    registrar.register_fn(
        "core_string_index_of",
        None,
        core_string_index_of,
        2,
        Some(2),
    );

    registrar.register_fn("core_array_join", None, core_array_join, 2, Some(2));
    registrar.register_fn("core_array_sort", None, core_array_sort, 1, Some(1));
    registrar.register_fn("core_array_reverse", None, core_array_reverse, 1, Some(1));

    registrar.register_fn("core_json_parse", None, core_json_parse, 1, Some(1));
    registrar.register_fn("core_json_stringify", None, core_json_stringify, 1, Some(1));

    registrar.register_fn("core_print", None, core_print, 1, Some(1));
    registrar.register_fn("core_println", None, core_println, 1, Some(1));
    registrar.register_fn("core_eprint", None, core_eprint, 1, Some(1));
    registrar.register_fn("core_input", None, core_input, 0, None);
    registrar.register_fn("core_len", None, core_len, 0, None);
    registrar.register_fn("core_env_get", None, core_env_get, 1, Some(1));
    registrar.register_fn("core_env_set", None, core_env_set, 2, Some(2));
    registrar.register_fn("core_exit", None, core_exit, 1, Some(1));
    registrar.register_fn("core_random", None, core_random, 0, Some(0));
}
