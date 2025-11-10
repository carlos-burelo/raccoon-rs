//! Macros for defining built-in functions
//!
//! This module provides macros to easily define built-in functions
//! with minimal boilerplate, similar to the primitive macro system.

/// Macro to define a builtin function that returns a RuntimeValue
///
/// Usage:
/// ```
/// builtin_fn! {
///     print(args) {
///         // implementation
///         RuntimeValue::Null(NullValue::new())
///     }
/// }
/// ```
#[macro_export]
macro_rules! builtin_fn {
    ($name:ident($args:ident) -> $ret:ty $body:block) => {
        pub fn $name($args: Vec<$crate::runtime::RuntimeValue>) -> $ret {
            $body
        }
    };
}

/// Macro to create a NativeFunction RuntimeValue from a closure
///
/// Usage:
/// ```
/// builtin_native_fn!(|args| {
///     // implementation
///     RuntimeValue::Null(NullValue::new())
/// }, fn_type)
/// ```
#[macro_export]
macro_rules! builtin_native_fn {
    ($closure:expr, $fn_type:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction(
            $crate::runtime::NativeFunctionValue::new($closure, $fn_type)
        )
    };
}

/// Macro to register a builtin in the environment
///
/// Usage:
/// ```
/// register_builtin!(env, "print", print_fn());
/// ```
#[macro_export]
macro_rules! register_builtin {
    ($env:expr, $name:expr, $value:expr) => {
        let _ = $env.declare($name.to_string(), $value);
    };
}

/// Macro to define and register multiple builtins at once
///
/// Usage:
/// ```
/// builtins! {
///     env => {
///         "print" => print_fn(),
///         "println" => println_fn(),
///     }
/// }
/// ```
#[macro_export]
macro_rules! builtins {
    ($env:expr => { $( $name:expr => $value:expr ),* $(,)? }) => {
        $(
            register_builtin!($env, $name, $value);
        )*
    };
}
