#[macro_export]
macro_rules! fn_type {

    (void) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $crate::ast::types::PrimitiveType::void(),
                is_variadic: false,
            }
        ))
    };


    ($return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $return,
                is_variadic: false,
            }
        ))
    };


    (variadic, $return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $return,
                is_variadic: true,
            }
        ))
    };


    ($param:expr, $return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![$param],
                return_type: $return,
                is_variadic: false,
            }
        ))
    };


    ([$($param:expr),+], $return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![$($param),+],
                return_type: $return,
                is_variadic: false,
            }
        ))
    };
}

#[macro_export]
macro_rules! null_return {
    () => {
        $crate::runtime::RuntimeValue::Null($crate::runtime::NullValue::new())
    };
}

#[macro_export]
macro_rules! int_value {
    ($val:expr) => {
        $crate::runtime::RuntimeValue::Int($crate::runtime::IntValue::new($val))
    };
}

#[macro_export]
macro_rules! str_value {
    ($val:expr) => {
        $crate::runtime::RuntimeValue::Str($crate::runtime::StrValue::new($val))
    };
}

#[macro_export]
macro_rules! list_value {
    ($elements:expr, $element_type:expr) => {
        $crate::runtime::RuntimeValue::List($crate::runtime::ListValue::new(
            $elements,
            $element_type,
        ))
    };
}

#[macro_export]
macro_rules! declare_builtin {
    ($env:expr, $name:expr, $impl:expr, $type:expr) => {
        let _ = $env.declare(
            $name.to_string(),
            $crate::runtime::RuntimeValue::NativeFunction(
                $crate::runtime::NativeFunctionValue::new($impl, $type),
            ),
        );
    };
}

#[macro_export]
macro_rules! check_args {
    ($args:expr, $expected:expr) => {
        if $args.len() != $expected {
            return null_return!();
        }
    };

    ($args:expr, $min:expr, $max:expr) => {
        if $args.len() < $min || $args.len() > $max {
            return null_return!();
        }
    };
}

#[macro_export]
macro_rules! extract_value {
    ($args:expr, $index:expr, $pattern:pat => $body:expr) => {
        match &$args[$index] {
            $pattern => $body,
            _ => return null_return!(),
        }
    };
}

/// Macro para declarar múltiples funciones nativas de forma concisa
///
/// # Ejemplos
/// ```
/// native_functions! {
///     env,
///
///     "native_print" (variadic) -> void => |args| {
///         for (i, arg) in args.iter().enumerate() {
///             if i > 0 { print!(" "); }
///             print!("{}", arg);
///         }
///         println!();
///         null_return!()
///     },
///
///     "native_sqrt" (float) -> float => |args| {
///         if args.is_empty() { return null_return!(); }
///         match &args[0] {
///             RuntimeValue::Float(f) => {
///                 RuntimeValue::Float(FloatValue::new(f.value.sqrt()))
///             }
///             RuntimeValue::Int(i) => {
///                 RuntimeValue::Float(FloatValue::new((i.value as f64).sqrt()))
///             }
///             _ => null_return!()
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! native_functions {
    // Entry point: procesa el environment y las funciones
    ($env:expr, $( $name:expr => $body:expr ),* $(,)?) => {
        $(
            let _ = $env.declare(
                $name.to_string(),
                $body,
            );
        )*
    };
}

/// Macro para crear una función nativa con tipo variadic
#[macro_export]
macro_rules! native_fn_variadic {
    ($impl:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction($crate::runtime::NativeFunctionValue::new(
            $impl,
            fn_type!(variadic, $crate::ast::types::PrimitiveType::void()),
        ))
    };
    ($impl:expr, $return:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction($crate::runtime::NativeFunctionValue::new(
            $impl,
            fn_type!(variadic, $return),
        ))
    };
}

/// Macro para crear una función nativa con parámetros específicos
#[macro_export]
macro_rules! native_fn {
    // Sin parámetros, retorna void
    ($impl:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction(
            $crate::runtime::NativeFunctionValue::new(
                $impl,
                fn_type!(void)
            )
        )
    };

    // Un parámetro
    ($impl:expr, $param:expr => $return:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction(
            $crate::runtime::NativeFunctionValue::new(
                $impl,
                fn_type!($param, $return)
            )
        )
    };

    // Múltiples parámetros
    ($impl:expr, [$($param:expr),+] => $return:expr) => {
        $crate::runtime::RuntimeValue::NativeFunction(
            $crate::runtime::NativeFunctionValue::new(
                $impl,
                fn_type!([$($param),+], $return)
            )
        )
    };
}
