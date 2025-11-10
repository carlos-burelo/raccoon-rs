#[macro_export]
macro_rules! define_native {
    ($name:ident($arg:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg1:ident: f64, $arg2:ident: f64) -> f64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let $arg2 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg1:ident: String, $arg2:ident: String) -> String $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: String = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg1:ident: String, $arg2:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg:ident: String) -> bool $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: bool = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg = String::from_raccoon(&args[0]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg1:ident: String, $arg2:ident: String) -> i64 $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::{FromRaccoon, ToRaccoon};
            let $arg1 = String::from_raccoon(&args[0]).unwrap_or_default();
            let $arg2 = String::from_raccoon(&args[1]).unwrap_or_default();
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident() -> i64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: i64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident() -> f64 $body:block) => {
        pub fn $name(_args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::ToRaccoon;
            let result: f64 = $body;
            result.to_raccoon()
        }
    };

    ($name:ident($arg:ident: i64) -> () $body:block) => {
        pub fn $name(args: Vec<$crate::runtime::RuntimeValue>) -> $crate::runtime::RuntimeValue {
            use $crate::runtime::FromRaccoon;
            let $arg = i64::from_raccoon(&args[0]).unwrap_or(0);
            $body;
            $crate::runtime::RuntimeValue::Null($crate::runtime::values::NullValue)
        }
    };
}

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
