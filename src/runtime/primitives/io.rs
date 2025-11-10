use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

primitive! {
    io::core_file_read(path: String) -> String {
        std::fs::read_to_string(&path).unwrap_or_default()
    }
}

primitive! {
    io::core_file_write(path: String, content: String) -> bool {
        std::fs::write(&path, content).is_ok()
    }
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

primitive! {
    io::core_file_exists(path: String) -> bool {
        std::path::Path::new(&path).exists()
    }
}

primitive! {
    io::core_file_delete(path: String) -> bool {
        std::fs::remove_file(&path).is_ok()
    }
}

primitive! {
    io::core_dir_create(path: String) -> bool {
        std::fs::create_dir_all(&path).is_ok()
    }
}

primitive! {
    io::core_dir_list(path: String) -> String {
        match std::fs::read_dir(&path) {
            Ok(entries) => {
                let names: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                    .collect();
                serde_json::to_string(&names)
                    .unwrap_or_else(|_| "[]".to_string())
            }
            Err(_) => "[]".to_string(),
        }
    }
}

pub fn register_io_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, io, {
        core_file_read: 1..=1,
        core_file_write: 2..=2,
        core_file_append: 2..=2,
        core_file_exists: 1..=1,
        core_file_delete: 1..=1,
        core_dir_create: 1..=1,
        core_dir_list: 1..=1,
    });
}
