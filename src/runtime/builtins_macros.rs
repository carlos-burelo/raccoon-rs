/// Macros for simplifying builtin function definitions
///
/// These macros reduce boilerplate when creating builtin functions,
/// making the code more readable and maintainable.

/// Create a function type with simpler syntax
///
/// # Examples
/// ```ignore
/// fn_type!(void)  // No params, returns void
/// fn_type!(str)   // No params, returns str
/// fn_type!(int, any)  // 1 param (any), returns int
/// fn_type!(variadic, void)  // Variadic params, returns void
/// ```
#[macro_export]
macro_rules! fn_type {
    // No parameters, void return
    (void) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $crate::ast::types::PrimitiveType::void(),
                is_variadic: false,
            }
        ))
    };

    // No parameters, specific return type
    ($return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $return,
                is_variadic: false,
            }
        ))
    };

    // Variadic parameters with return type
    (variadic, $return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![],
                return_type: $return,
                is_variadic: true,
            }
        ))
    };

    // Single parameter with return type
    ($param:expr, $return:expr) => {
        $crate::ast::types::Type::Function(Box::new(
            $crate::ast::types::FunctionType {
                params: vec![$param],
                return_type: $return,
                is_variadic: false,
            }
        ))
    };

    // Multiple parameters with return type
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

/// Quick builder for returning null value
#[macro_export]
macro_rules! null_return {
    () => {
        $crate::runtime::RuntimeValue::Null($crate::runtime::NullValue::new())
    };
}

/// Quick builder for returning int value
#[macro_export]
macro_rules! int_value {
    ($val:expr) => {
        $crate::runtime::RuntimeValue::Int($crate::runtime::IntValue::new($val))
    };
}

/// Quick builder for returning string value
#[macro_export]
macro_rules! str_value {
    ($val:expr) => {
        $crate::runtime::RuntimeValue::Str($crate::runtime::StrValue::new($val))
    };
}

/// Quick builder for returning list value
#[macro_export]
macro_rules! list_value {
    ($elements:expr, $element_type:expr) => {
        $crate::runtime::RuntimeValue::List($crate::runtime::ListValue::new($elements, $element_type))
    };
}

/// Declare a builtin function in the environment
///
/// # Examples
/// ```ignore
/// declare_builtin!(env, "print", print_impl, fn_type!(variadic, void));
/// ```
#[macro_export]
macro_rules! declare_builtin {
    ($env:expr, $name:expr, $impl:expr, $type:expr) => {
        let _ = $env.declare(
            $name.to_string(),
            $crate::runtime::RuntimeValue::NativeFunction(
                $crate::runtime::NativeFunctionValue::new($impl, $type)
            ),
        );
    };
}

/// Validate argument count for builtin functions
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

/// Extract a value from args with type checking
#[macro_export]
macro_rules! extract_value {
    ($args:expr, $index:expr, $pattern:pat => $body:expr) => {
        match &$args[$index] {
            $pattern => $body,
            _ => return null_return!(),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ast::types::{PrimitiveType, Type};

    #[test]
    fn test_fn_type_void() {
        let fn_type = fn_type!(void);
        match fn_type {
            Type::Function(ft) => {
                assert!(ft.params.is_empty());
                assert!(!ft.is_variadic);
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_fn_type_with_return() {
        let fn_type = fn_type!(PrimitiveType::int());
        match fn_type {
            Type::Function(ft) => {
                assert!(ft.params.is_empty());
                assert!(!ft.is_variadic);
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_fn_type_variadic() {
        let fn_type = fn_type!(variadic, PrimitiveType::void());
        match fn_type {
            Type::Function(ft) => {
                assert!(ft.is_variadic);
            }
            _ => panic!("Expected function type"),
        }
    }
}
