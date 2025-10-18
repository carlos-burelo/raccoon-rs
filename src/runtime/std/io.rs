use crate::ast::types::{ListType, PrimitiveType, Type};
use crate::runtime::values::{BoolValue, IntValue, ListValue, NullValue, RuntimeValue, StrValue};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct IOModule;

impl IOModule {
    pub fn name() -> &'static str {
        "std:io"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        // File operations
        exports.insert("readFile".to_string(), Self::create_read_file_fn());
        exports.insert("writeFile".to_string(), Self::create_write_file_fn());
        exports.insert("appendFile".to_string(), Self::create_append_file_fn());
        exports.insert("deleteFile".to_string(), Self::create_delete_file_fn());
        exports.insert("fileExists".to_string(), Self::create_file_exists_fn());
        exports.insert("copyFile".to_string(), Self::create_copy_file_fn());
        exports.insert("moveFile".to_string(), Self::create_move_file_fn());
        exports.insert("fileSize".to_string(), Self::create_file_size_fn());

        // Directory operations
        exports.insert("createDir".to_string(), Self::create_create_dir_fn());
        exports.insert("removeDir".to_string(), Self::create_remove_dir_fn());
        exports.insert("dirExists".to_string(), Self::create_dir_exists_fn());
        exports.insert("listDir".to_string(), Self::create_list_dir_fn());

        // Path operations
        exports.insert("joinPath".to_string(), Self::create_join_path_fn());
        exports.insert("basename".to_string(), Self::create_basename_fn());
        exports.insert("dirname".to_string(), Self::create_dirname_fn());
        exports.insert("extension".to_string(), Self::create_extension_fn());
        exports.insert("absolutePath".to_string(), Self::create_absolute_path_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "readFile" => Some(Self::create_read_file_fn()),
            "writeFile" => Some(Self::create_write_file_fn()),
            "appendFile" => Some(Self::create_append_file_fn()),
            "deleteFile" => Some(Self::create_delete_file_fn()),
            "fileExists" => Some(Self::create_file_exists_fn()),
            "copyFile" => Some(Self::create_copy_file_fn()),
            "moveFile" => Some(Self::create_move_file_fn()),
            "fileSize" => Some(Self::create_file_size_fn()),
            "createDir" => Some(Self::create_create_dir_fn()),
            "removeDir" => Some(Self::create_remove_dir_fn()),
            "dirExists" => Some(Self::create_dir_exists_fn()),
            "listDir" => Some(Self::create_list_dir_fn()),
            "joinPath" => Some(Self::create_join_path_fn()),
            "basename" => Some(Self::create_basename_fn()),
            "dirname" => Some(Self::create_dirname_fn()),
            "extension" => Some(Self::create_extension_fn()),
            "absolutePath" => Some(Self::create_absolute_path_fn()),
            _ => None,
        }
    }

    // File operations

    fn create_read_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::read_to_string(&path.value) {
                        Ok(content) => RuntimeValue::Str(StrValue::new(content)),
                        Err(_) => RuntimeValue::Null(NullValue::new()),
                    },
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_write_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match (&args[0], &args[1]) {
                    (RuntimeValue::Str(path), RuntimeValue::Str(content)) => {
                        match fs::write(&path.value, &content.value) {
                            Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                            Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                        }
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_append_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match (&args[0], &args[1]) {
                    (RuntimeValue::Str(path), RuntimeValue::Str(content)) => {
                        match std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path.value)
                        {
                            Ok(mut file) => match file.write_all(content.value.as_bytes()) {
                                Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                                Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                            },
                            Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                        }
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_delete_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::remove_file(&path.value) {
                        Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                        Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                    },
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_file_exists_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => {
                        let exists = Path::new(&path.value).is_file();
                        RuntimeValue::Bool(BoolValue::new(exists))
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_copy_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match (&args[0], &args[1]) {
                    (RuntimeValue::Str(src), RuntimeValue::Str(dst)) => {
                        match fs::copy(&src.value, &dst.value) {
                            Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                            Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                        }
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_move_file_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match (&args[0], &args[1]) {
                    (RuntimeValue::Str(src), RuntimeValue::Str(dst)) => {
                        match fs::rename(&src.value, &dst.value) {
                            Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                            Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                        }
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_file_size_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Int(IntValue::new(-1));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::metadata(&path.value) {
                        Ok(metadata) => RuntimeValue::Int(IntValue::new(metadata.len() as i64)),
                        Err(_) => RuntimeValue::Int(IntValue::new(-1)),
                    },
                    _ => RuntimeValue::Int(IntValue::new(-1)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    // Directory operations

    fn create_create_dir_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::create_dir_all(&path.value) {
                        Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                        Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                    },
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_remove_dir_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::remove_dir_all(&path.value) {
                        Ok(_) => RuntimeValue::Bool(BoolValue::new(true)),
                        Err(_) => RuntimeValue::Bool(BoolValue::new(false)),
                    },
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_dir_exists_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }
                match &args[0] {
                    RuntimeValue::Str(path) => {
                        let exists = Path::new(&path.value).is_dir();
                        RuntimeValue::Bool(BoolValue::new(exists))
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_list_dir_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path) => match fs::read_dir(&path.value) {
                        Ok(entries) => {
                            let files: Vec<RuntimeValue> = entries
                                .filter_map(|entry| entry.ok())
                                .filter_map(|entry| {
                                    entry
                                        .file_name()
                                        .to_str()
                                        .map(|s| RuntimeValue::Str(StrValue::new(s.to_string())))
                                })
                                .collect();
                            RuntimeValue::List(ListValue::new(files, PrimitiveType::str()))
                        }
                        Err(_) => RuntimeValue::Null(NullValue::new()),
                    },
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::str(),
                })),
                is_variadic: false,
            })),
        ))
    }

    // Path operations

    fn create_join_path_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let parts: Vec<String> = args
                    .iter()
                    .filter_map(|arg| match arg {
                        RuntimeValue::Str(s) => Some(s.value.clone()),
                        _ => None,
                    })
                    .collect();

                let mut path = std::path::PathBuf::new();
                for part in parts {
                    path.push(part);
                }

                RuntimeValue::Str(StrValue::new(path.to_string_lossy().to_string()))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: true,
            })),
        ))
    }

    fn create_basename_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path_str) => {
                        let path = Path::new(&path_str.value);
                        match path.file_name() {
                            Some(name) => {
                                RuntimeValue::Str(StrValue::new(name.to_string_lossy().to_string()))
                            }
                            None => RuntimeValue::Null(NullValue::new()),
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_dirname_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path_str) => {
                        let path = Path::new(&path_str.value);
                        match path.parent() {
                            Some(parent) => RuntimeValue::Str(StrValue::new(
                                parent.to_string_lossy().to_string(),
                            )),
                            None => RuntimeValue::Null(NullValue::new()),
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_extension_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path_str) => {
                        let path = Path::new(&path_str.value);
                        match path.extension() {
                            Some(ext) => {
                                RuntimeValue::Str(StrValue::new(ext.to_string_lossy().to_string()))
                            }
                            None => RuntimeValue::Null(NullValue::new()),
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_absolute_path_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Str(path_str) => {
                        let path = Path::new(&path_str.value);
                        match path.canonicalize() {
                            Ok(abs_path) => RuntimeValue::Str(StrValue::new(
                                abs_path.to_string_lossy().to_string(),
                            )),
                            Err(_) => RuntimeValue::Null(NullValue::new()),
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }
}
