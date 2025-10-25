/// FFI (Foreign Function Interface) stubs
use crate::ast::types::{FunctionType, ListType, PrimitiveType, Type};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue, StrValue};
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // FFI stubs - these are placeholders

    // native_ffi_load_library - stub
    functions.insert(
        "native_ffi_load_library".to_string(),
        NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::str()],
                return_type: PrimitiveType::void(),
                is_variadic: false,
            })),
        ),
    );

    // native_ffi_register_function - stub
    functions.insert(
        "native_ffi_register_function".to_string(),
        NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
            Type::Function(Box::new(FunctionType {
                params: vec![PrimitiveType::str(), PrimitiveType::any()],
                return_type: PrimitiveType::void(),
                is_variadic: false,
            })),
        ),
    );

    // native_ffi_call - stub
    functions.insert(
        "native_ffi_call".to_string(),
        NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| RuntimeValue::Null(NullValue::new()),
            Type::Function(Box::new(FunctionType {
                params: vec![
                    PrimitiveType::str(),
                    PrimitiveType::str(),
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                ],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ),
    );

    // getOS - actual implementation
    functions.insert(
        "getOS".to_string(),
        NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let os = if cfg!(target_os = "windows") {
                    "windows"
                } else if cfg!(target_os = "linux") {
                    "linux"
                } else if cfg!(target_os = "macos") {
                    "darwin"
                } else {
                    "unknown"
                };
                RuntimeValue::Str(StrValue::new(os.to_string()))
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ),
    );
}
