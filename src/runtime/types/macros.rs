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

#[macro_export]
macro_rules! prop_meta {
    ($name:expr => $type:expr, $desc:expr) => {
        $crate::runtime::types::metadata::PropertyMetadata::new($name, $type, $desc)
    };
    ($name:expr => $type:expr, $desc:expr, readonly) => {
        $crate::runtime::types::metadata::PropertyMetadata::new($name, $type, $desc).readonly()
    };
}

#[macro_export]
macro_rules! define_type {
    (
        struct $name:ident {
            type_name: $type_name:expr,
            description: $description:expr
            $(,)?
        }
    ) => {
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl $name {
            pub fn metadata() -> $crate::runtime::types::metadata::TypeMetadata {
                $crate::runtime::types::metadata::TypeMetadata::new($type_name, $description)
            }
        }

        #[async_trait::async_trait]
        impl $crate::runtime::types::TypeHandler for $name {
            fn type_name(&self) -> &str {
                $type_name
            }

            fn call_instance_method(
                &self,
                _value: &mut $crate::runtime::RuntimeValue,
                method: &str,
                _args: Vec<$crate::runtime::RuntimeValue>,
                position: $crate::tokens::Position,
                file: Option<String>,
            ) -> Result<$crate::runtime::RuntimeValue, $crate::error::RaccoonError> {
                Err($crate::runtime::types::helpers::method_not_found_error(
                    $type_name, method, position, file,
                ))
            }

            fn call_static_method(
                &self,
                method: &str,
                _args: Vec<$crate::runtime::RuntimeValue>,
                position: $crate::tokens::Position,
                file: Option<String>,
            ) -> Result<$crate::runtime::RuntimeValue, $crate::error::RaccoonError> {
                Err(
                    $crate::runtime::types::helpers::static_method_not_found_error(
                        $type_name, method, position, file,
                    ),
                )
            }

            fn has_instance_method(&self, _method: &str) -> bool {
                false
            }

            fn has_static_method(&self, _method: &str) -> bool {
                false
            }

            fn has_async_instance_method(&self, _method: &str) -> bool {
                false
            }
        }
    };
}
