use crate::runtime::{Registrar, FromRaccoon, ToRaccoon};

pub fn register_http_module(registrar: &mut Registrar) {
    // fetch(url: string) -> string (simplified - returns placeholder)
    registrar.register_fn(
        "fetch",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            // Simplified: just return the URL as a response
            format!("Fetched from: {}", url).to_raccoon()
        },
        1,
        Some(1),
    );

    // get(url: string) -> string
    registrar.register_fn(
        "get",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            format!("GET {}", url).to_raccoon()
        },
        1,
        Some(1),
    );

    // post(url: string, body: string) -> string
    registrar.register_fn(
        "post",
        Some("http"),
        |args| {
            let url = String::from_raccoon(&args[0]).unwrap_or_default();
            let body = String::from_raccoon(&args[1]).unwrap_or_default();
            format!("POST {} with body: {}", url, body).to_raccoon()
        },
        2,
        Some(2),
    );
}
