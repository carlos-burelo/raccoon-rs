/// I/O functions: File operations and input
use crate::ast::types::{FunctionType, ListType, PrimitiveType, Type};
use crate::runtime::values::{ListValue, NativeFunctionValue, RuntimeValue, StrValue, BoolValue};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // Read file
    functions.insert(
        "native_io_read_file".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(path)) = args.first() {
                    match fs::read_to_string(&path.value) {
                        Ok(content) => RuntimeValue::Str(StrValue::new(content)),
                        Err(_) => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                    }
                } else {
                    RuntimeValue::Null(crate::runtime::values::NullValue::new())
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );

    // Write file
    functions.insert(
        "native_io_write_file".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }

                let path = match &args[0] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                };

                let content = match &args[1] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                };

                match fs::write(&path, &content) {
                    Ok(_) => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                    Err(_) => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ),
    );

    // Append file
    functions.insert(
        "native_io_append_file".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }

                let path = match &args[0] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                };

                let content = match &args[1] {
                    RuntimeValue::Str(s) => s.value.clone(),
                    _ => return RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                };

                use std::fs::OpenOptions;

                match OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                {
                    Ok(mut file) => {
                        let _ = file.write_all(content.as_bytes());
                        RuntimeValue::Null(crate::runtime::values::NullValue::new())
                    }
                    Err(_) => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ),
    );

    // File exists
    functions.insert(
        "native_io_file_exists".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(path)) = args.first() {
                    let exists = Path::new(&path.value).exists();
                    RuntimeValue::Bool(BoolValue::new(exists))
                } else {
                    RuntimeValue::Bool(BoolValue::new(false))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ),
    );

    // Delete file
    functions.insert(
        "native_io_delete_file".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(path)) = args.first() {
                    let _ = fs::remove_file(&path.value);
                }
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ),
    );

    // Read directory
    functions.insert(
        "native_io_read_dir".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(path)) = args.first() {
                    match fs::read_dir(&path.value) {
                        Ok(entries) => {
                            let files: Vec<RuntimeValue> = entries
                                .filter_map(|entry| {
                                    entry.ok().and_then(|e| {
                                        e.file_name()
                                            .into_string()
                                            .ok()
                                            .map(|name| RuntimeValue::Str(StrValue::new(name)))
                                    })
                                })
                                .collect();
                            RuntimeValue::List(ListValue::new(files, PrimitiveType::str()))
                        }
                        Err(_) => RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str())),
                    }
                } else {
                    RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::str(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // Create directory
    functions.insert(
        "native_io_create_dir".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::Str(path)) = args.first() {
                    let _ = fs::create_dir_all(&path.value);
                }
                RuntimeValue::Null(crate::runtime::values::NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ),
    );

    // Read input from stdin
    functions.insert(
        "native_io_input".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                // Print prompt if provided
                if let Some(RuntimeValue::Str(prompt)) = args.first() {
                    print!("{}", prompt.value);
                    let _ = io::stdout().flush();
                }

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        // Remove trailing newline
                        if input.ends_with('\n') {
                            input.pop();
                        }
                        if input.ends_with('\r') {
                            input.pop();
                        }
                        RuntimeValue::Str(StrValue::new(input))
                    }
                    Err(_) => RuntimeValue::Str(StrValue::new("".to_string())),
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );
}
