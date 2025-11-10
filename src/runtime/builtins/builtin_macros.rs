#[macro_export]
macro_rules! builtin_fn {
    ($name:ident($args:ident) -> $ret:ty $body:block) => {
        pub fn $name($args: Vec<$crate::runtime::RuntimeValue>) -> $ret {
            $body
        }
    };
}

#[macro_export]
macro_rules! builtin_native_fn {
    ($closure:expr, $fn_type:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction($crate::runtime::NativeFunctionValue::new(
            $closure, $fn_type,
        ))
    };
}

#[macro_export]
macro_rules! register_builtin {
    ($env:expr, $name:expr, $value:expr) => {
        let _ = $env.declare($name.to_string(), $value);
    };
}

#[macro_export]
macro_rules! builtins {
    ($env:expr => { $( $name:expr => $value:expr ),* $(,)? }) => {
        $(
            register_builtin!($env, $name, $value);
        )*
    };
}
