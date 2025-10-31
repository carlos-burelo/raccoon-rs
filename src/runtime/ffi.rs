/// FFI Module - DEPRECATED
///
/// This module is deprecated and will be refactored with the new plugin system.
/// The match statement explosion here (lines 100-200) is a perfect example of
/// code that the plugin system eliminates.
///
/// See: src/runtime/plugin_system.rs and src/runtime/builtin_plugins.rs

use crate::error::RaccoonError;
use crate::runtime::values::{BoolValue, FloatValue, IntValue, NullValue, RuntimeValue, StrValue};
use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub enum FFIType {
    Int,
    Float,
    Bool,
    Str,
    Void,
}

#[derive(Debug, Clone)]
pub enum FFIValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Null,
}

impl FFIValue {
    pub fn get_type_name(&self) -> &'static str {
        match self {
            FFIValue::Int(_) => "int",
            FFIValue::Float(_) => "float",
            FFIValue::Bool(_) => "bool",
            FFIValue::Str(_) => "str",
            FFIValue::Null => "null",
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, FFIValue::Int(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, FFIValue::Float(_))
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, FFIValue::Bool(_))
    }
    pub fn is_str(&self) -> bool {
        matches!(self, FFIValue::Str(_))
    }

    pub unsafe fn as_int(&self) -> i64 {
        match self {
            FFIValue::Int(val) => *val,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub unsafe fn as_float(&self) -> f64 {
        match self {
            FFIValue::Float(val) => *val,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub unsafe fn as_bool(&self) -> bool {
        match self {
            FFIValue::Bool(val) => *val,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub unsafe fn as_str(&self) -> &String {
        match self {
            FFIValue::Str(val) => val,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

pub struct FFIHost;

impl FFIHost {
    pub unsafe fn call_native_function(
        &self,
        library: &Library,
        func_name: &str,
        args: &[FFIValue],
        return_type: &FFIType,
    ) -> Result<RuntimeValue, RaccoonError> {
        let symbol: Symbol<*mut c_void> =
            unsafe { library.get(func_name.as_bytes()) }.map_err(|e| {
                RaccoonError::new(
                    format!(
                        "Error FFI: No se pudo cargar el símbolo '{}': {}",
                        func_name, e
                    ),
                    (0, 0),
                    None::<String>,
                )
            })?;

        match (args.len(), return_type) {
            (0, FFIType::Int) => {
                let result = unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn() -> i64>(*symbol);
                    func()
                };
                Ok(RuntimeValue::Int(IntValue::new(result)))
            }
            (0, FFIType::Float) => {
                let result = unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn() -> f64>(*symbol);
                    func()
                };
                Ok(RuntimeValue::Float(FloatValue::new(result)))
            }
            (0, FFIType::Bool) => {
                let result = unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn() -> bool>(*symbol);
                    func()
                };
                Ok(RuntimeValue::Bool(BoolValue::new(result)))
            }
            (0, FFIType::Void) => {
                unsafe {
                    let func = mem::transmute::<*mut c_void, unsafe extern "Rust" fn()>(*symbol);
                    func();
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }

            (1, FFIType::Int) if args[0].is_int() => {
                let result = unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn(i64) -> i64>(*symbol);
                    func(args[0].as_int())
                };
                Ok(RuntimeValue::Int(IntValue::new(result)))
            }
            (1, FFIType::Float) if args[0].is_float() => {
                let result = unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn(f64) -> f64>(*symbol);
                    func(args[0].as_float())
                };
                Ok(RuntimeValue::Float(FloatValue::new(result)))
            }
            (1, FFIType::Void) if args[0].is_str() => {
                unsafe {
                    let func =
                        mem::transmute::<*mut c_void, unsafe extern "Rust" fn(&str)>(*symbol);
                    func(args[0].as_str().as_str());
                }
                Ok(RuntimeValue::Null(NullValue::new()))
            }
            (1, FFIType::Str) if args[0].is_str() => {
                let result = unsafe {
                    let func = mem::transmute::<*mut c_void, unsafe extern "Rust" fn(&str) -> String>(
                        *symbol,
                    );
                    func(args[0].as_str().as_str())
                };
                Ok(RuntimeValue::Str(StrValue::new(result)))
            }

            (2, FFIType::Int) if args[0].is_int() && args[1].is_int() => {
                let result = unsafe {
                    let func = mem::transmute::<
                        *mut c_void,
                        unsafe extern "Rust" fn(i64, i64) -> i64,
                    >(*symbol);
                    func(args[0].as_int(), args[1].as_int())
                };
                Ok(RuntimeValue::Int(IntValue::new(result)))
            }
            (2, FFIType::Float) if args[0].is_float() && args[1].is_float() => {
                let result = unsafe {
                    let func = mem::transmute::<
                        *mut c_void,
                        unsafe extern "Rust" fn(f64, f64) -> f64,
                    >(*symbol);
                    func(args[0].as_float(), args[1].as_float())
                };
                Ok(RuntimeValue::Float(FloatValue::new(result)))
            }

            _ => {
                let arg_types: Vec<&str> = args.iter().map(|a| a.get_type_name()).collect();

                Err(RaccoonError::new(
                    format!(
                        "Error FFI: Firma no soportada para '{}': Args: {:?}, Return: {:?}",
                        func_name, arg_types, return_type
                    ),
                    (0, 0),
                    None::<String>,
                ))
            }
        }
    }
}
