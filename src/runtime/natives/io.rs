use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};
use std::fs;

pub fn register_io_module(registrar: &mut Registrar) {
    // read_file(path: string) -> string
    registrar.register_fn(
        "read_file",
        Some("io"),
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            match fs::read_to_string(&path) {
                Ok(content) => content.to_raccoon(),
                Err(_) => RuntimeValue::Null(crate::runtime::NullValue::new()),
            }
        },
        1,
        Some(1),
    );

    // write_file(path: string, content: string) -> bool
    registrar.register_fn(
        "write_file",
        Some("io"),
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            let content = String::from_raccoon(&args[1]).unwrap_or_default();
            match fs::write(&path, &content) {
                Ok(_) => true.to_raccoon(),
                Err(_) => false.to_raccoon(),
            }
        },
        2,
        Some(2),
    );

    // file_exists(path: string) -> bool
    registrar.register_fn(
        "file_exists",
        Some("io"),
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            std::path::Path::new(&path).exists().to_raccoon()
        },
        1,
        Some(1),
    );

    // delete_file(path: string) -> bool
    registrar.register_fn(
        "delete_file",
        Some("io"),
        |args| {
            let path = String::from_raccoon(&args[0]).unwrap_or_default();
            match fs::remove_file(&path) {
                Ok(_) => true.to_raccoon(),
                Err(_) => false.to_raccoon(),
            }
        },
        1,
        Some(1),
    );
}
