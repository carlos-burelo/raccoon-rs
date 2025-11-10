#[macro_export]
macro_rules! primitive {
    (math::$name:ident($arg:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    (math::$name:ident($arg1:ident: f64, $arg2:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let $arg2 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    (string::$name:ident($arg1:ident: String, $arg2:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    (time::$name:ident() -> i64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    (system::$name:ident($arg:ident: i64) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = i64::from_raccoon(&args[0]).unwrap_or(0);
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };

    (system::$name:ident($arg:ident: String) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };

    (system::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (system::$name:ident() -> f64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    (io::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (io::$name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    (io::$name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    (http::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (http::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (array::$name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (array::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
            let result: String = $body;
            result.to_raccoon()
        }
    };

    (json::$name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };
}

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
