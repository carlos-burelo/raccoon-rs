/// Macro system for defining native primitives
/// Simplifies registration of native functions without repetitive boilerplate

/// Macro to define a native primitive function
///
/// Usage:
/// ```
/// define_native!(core_sqrt(x: f64) -> f64 {
///     x.sqrt()
/// });
/// ```
#[macro_export]
macro_rules! define_native {
    // Single argument function
    ($name:ident($arg:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // Two argument function (f64, f64) -> f64
    ($name:ident($arg1:ident: f64, $arg2:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let $arg2 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // String argument function
    ($name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // Two string arguments
    ($name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    // Two string arguments -> bool
    ($name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // String argument -> bool
    ($name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    // String argument -> i64
    ($name:ident($arg:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // String, String -> i64
    ($name:ident($arg1:ident: String, $arg2:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // No arguments -> i64
    ($name:ident() -> i64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    // No arguments -> f64
    ($name:ident() -> f64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    // i64 argument
    ($name:ident($arg:ident: i64) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = i64::from_raccoon(&args[0]).unwrap_or(0);
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };
}

/// Macro to register a batch of native functions with the registrar
///
/// Usage:
/// ```
/// register_natives!(registrar, {
///     core_sqrt: 1..=1,
///     core_sin: 1..=1,
///     core_file_read: 1..=1,
/// });
/// ```
#[macro_export]
macro_rules! register_natives {
    ($registrar:expr, { $( $name:ident: $min:literal..=$max:literal ),* $(,)? }) => {
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

/// Convenience macro for functions that take no arguments
#[macro_export]
macro_rules! native_nullary {
    ($name:ident -> $ret:ty $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: $ret = $body;
            result.to_raccoon()
        }
    };
}

/// Convenience macro for functions that take one argument
#[macro_export]
macro_rules! native_unary {
    ($name:ident($arg:ident: $arg_ty:ty) -> $ret:ty $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = <$arg_ty>::from_raccoon(&args[0]).unwrap_or_default();
            let result: $ret = $body;
            result.to_raccoon()
        }
    };
}

/// Convenience macro for functions that take two arguments
#[macro_export]
macro_rules! native_binary {
    ($name:ident($arg1:ident: $arg1_ty:ty, $arg2:ident: $arg2_ty:ty) -> $ret:ty $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = <$arg1_ty>::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = <$arg2_ty>::from_raccoon(&args[1]).unwrap_or_default();
            let result: $ret = $body;
            result.to_raccoon()
        }
    };
}
