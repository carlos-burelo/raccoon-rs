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
