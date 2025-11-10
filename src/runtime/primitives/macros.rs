//! Macros for defining primitives and builtins
//!
//! This module provides a unified macro system for declaring both primitives
//! and builtins with minimal boilerplate.

/// Macro to define a primitive function
///
/// Usage:
/// ```
/// primitive! {
///     math::sqrt(x: f64) -> f64 {
///         x.sqrt()
///     }
/// }
/// ```
#[macro_export]
macro_rules! primitive {
    // Math context - single f64 argument
    (math::$name:ident($arg:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // Math context - two f64 arguments
    (math::$name:ident($arg1:ident: f64, $arg2:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let $arg2 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // String context - single String argument -> String
    (string::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // String context - two String arguments -> String
    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // String context - String argument -> bool
    (string::$name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // String context - two String arguments -> bool
    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // String context - String argument -> i64
    (string::$name:ident($arg:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // String context - two String arguments -> i64
    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // Time context - no arguments -> i64
    (time::$name:ident() -> i64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // System context - i64 argument -> void
    (system::$name:ident($arg:ident: i64) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = i64::from_raccoon(&args[0]).unwrap_or(0);
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };

    // System context - String argument -> void
    (system::$name:ident($arg:ident: String) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };

    // System context - String argument -> String
    (system::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // System context - no arguments -> f64
    (system::$name:ident() -> f64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // IO context - String argument -> String
    (io::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // IO context - two String arguments -> bool
    (io::$name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // IO context - String argument -> bool
    (io::$name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // HTTP context - String argument -> String
    (http::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // HTTP context - two String arguments -> String
    (http::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // Array context - two String arguments -> String
    (array::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // Array context - String argument -> String
    (array::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // JSON context - String argument -> String
    (json::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };
}

/// Macro to define a builtin function
///
/// Usage:
/// ```
/// builtin! {
///     print(args: &[RuntimeValue]) -> RuntimeValue {
///         // implementation
///     }
/// }
/// ```
#[macro_export]
macro_rules! builtin {
    ($name:ident($args:ident: &[RuntimeValue]) -> RuntimeValue $body:block) => {
        pub fn $name($args: &[RuntimeValue]) -> $crate::runtime::RuntimeValue {
            $body
        }
    };

    ($name:ident($args:ident: Vec<RuntimeValue>) -> RuntimeValue $body:block) => {
        pub fn $name($args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            $body
        }
    };
}

/// Macro to register a batch of primitives in a context
///
/// Usage:
/// ```
/// register_context_primitives!(registrar, math, {
///     sqrt: 1..=1,
///     pow: 2..=2,
/// });
/// ```
#[macro_export]
macro_rules! register_context_primitives {
    ($registrar:expr, $context:ident, { $( $name:ident: $min:literal..=$max:literal ),* $(,)? }) => {
        $(
            $registrar.register_fn(
                stringify!($name),
                None,
                $name,
                $min,
                Some($max),
            );
        )*
    };
}
