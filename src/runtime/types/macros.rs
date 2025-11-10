/// Macros for reducing boilerplate in type system
/// Provides declarative syntax for defining types and operations

/// Macro for defining binary operations with multiple type combinations
/// Reduces code duplication in arithmetic, comparison, and logical operations
///
/// # Example
/// ```ignore
/// binary_op! {
///     fn add(left, right) -> RuntimeValue {
///         (Int, Int) => Int(left.value + right.value),
///         (Float, Float) => Float(left.value + right.value),
///         (Int, Float) => Float(left.value as f64 + right.value),
///     }
/// }
/// ```
#[macro_export]
macro_rules! binary_op {
    (
        fn $name:ident($left:ident, $right:ident) -> RuntimeValue {
            $(($left_type:ident, $right_type:ident) => $result_type:ident($expr:expr)),+ $(,)?
        }
    ) => {
        pub fn $name(
            $left: $crate::runtime::RuntimeValue,
            $right: $crate::runtime::RuntimeValue,
            position: $crate::tokens::Position,
            file: &Option<String>,
        ) -> Result<$crate::runtime::RuntimeValue, $crate::error::RaccoonError> {
            use $crate::runtime::RuntimeValue::*;

            match (&$left, &$right) {
                $(
                    ($left_type(left), $right_type(right)) => {
                        Ok($result_type($expr))
                    }
                )+
                _ => Err($crate::error::RaccoonError::new(
                    format!(
                        "Invalid operands for {}: {} and {}",
                        stringify!($name),
                        $left.get_name(),
                        $right.get_name()
                    ),
                    position,
                    file.clone(),
                )),
            }
        }
    };
}

/// Macro for defining a method map for TypeHandler implementations
/// Automatically generates call_instance_method and has_instance_method
///
/// # Example
/// ```ignore
/// instance_methods! {
///     type_name: "str",
///     value_type: Str,
///     methods: {
///         "toUpper" => |s: &StrValue, args, pos, file| {
///             require_args(&args, 0, "toUpper", pos, file.clone())?;
///             Ok(RuntimeValue::Str(StrValue::new(s.value.to_uppercase())))
///         },
///         "toLower" => |s: &StrValue, args, pos, file| {
///             // ...
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! instance_methods {
    (
        type_name: $type_name:expr,
        value_type: $variant:ident,
        methods: {
            $($method:expr => $handler:expr),* $(,)?
        }
    ) => {
        fn call_instance_method(
            &self,
            value: &mut $crate::runtime::RuntimeValue,
            method: &str,
            args: Vec<$crate::runtime::RuntimeValue>,
            position: $crate::tokens::Position,
            file: Option<String>,
        ) -> Result<$crate::runtime::RuntimeValue, $crate::error::RaccoonError> {
            use $crate::runtime::RuntimeValue::*;

            let inner_value = match value {
                $variant(v) => v,
                _ => {
                    return Err($crate::error::RaccoonError::new(
                        format!("Expected {}, got {}", $type_name, value.get_name()),
                        position,
                        file,
                    ))
                }
            };

            match method {
                $(
                    $method => {
                        let handler: fn(_, Vec<_>, _, Option<String>) -> Result<_, _> = $handler;
                        handler(inner_value, args, position, file)
                    }
                )*
                _ => Err($crate::error::RaccoonError::new(
                    format!("Method '{}' not found on {}", method, $type_name),
                    position,
                    file,
                )),
            }
        }

        fn has_instance_method(&self, method: &str) -> bool {
            matches!(method, $($method)|*)
        }
    };
}

/// Macro for defining static methods
#[macro_export]
macro_rules! static_methods {
    (
        type_name: $type_name:expr,
        methods: {
            $($method:expr => $handler:expr),* $(,)?
        }
    ) => {
        fn call_static_method(
            &self,
            method: &str,
            args: Vec<$crate::runtime::RuntimeValue>,
            position: $crate::tokens::Position,
            file: Option<String>,
        ) -> Result<$crate::runtime::RuntimeValue, $crate::error::RaccoonError> {
            match method {
                $(
                    $method => {
                        let handler: fn(Vec<_>, _, Option<String>) -> Result<_, _> = $handler;
                        handler(args, position, file)
                    }
                )*
                _ => Err($crate::error::RaccoonError::new(
                    format!("Static method '{}' not found on {}", method, $type_name),
                    position,
                    file,
                )),
            }
        }

        fn has_static_method(&self, method: &str) -> bool {
            matches!(method, $($method)|*)
        }
    };
}

/// Macro for creating method metadata
#[macro_export]
macro_rules! method_meta {
    ($name:expr => $ret:expr, $desc:expr) => {
        $crate::runtime::types::metadata::MethodMetadata::new($name, $ret, $desc)
    };
    ($name:expr => $ret:expr, $desc:expr, params: [$(($param_name:expr, $param_type:expr)),*]) => {
        $crate::runtime::types::metadata::MethodMetadata::new($name, $ret, $desc)
            .with_params(vec![
                $($crate::runtime::types::metadata::ParamMetadata::new($param_name, $param_type)),*
            ])
    };
    ($name:expr => $ret:expr, $desc:expr, alias: $alias:expr) => {
        $crate::runtime::types::metadata::MethodMetadata::new($name, $ret, $desc)
            .with_alias($alias)
    };
}

/// Macro for creating property metadata
#[macro_export]
macro_rules! prop_meta {
    ($name:expr => $type:expr, $desc:expr) => {
        $crate::runtime::types::metadata::PropertyMetadata::new($name, $type, $desc)
    };
    ($name:expr => $type:expr, $desc:expr, readonly) => {
        $crate::runtime::types::metadata::PropertyMetadata::new($name, $type, $desc).readonly()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros_compile() {
        // This test just ensures the macros compile correctly
        // Actual functionality is tested in the modules that use them
    }
}
