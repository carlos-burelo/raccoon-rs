/// Array functions: length, push, slice, reverse
///
/// Uses declarative macros to reduce registration boilerplate by ~50%
use crate::ast::types::{FunctionType, ListType, PrimitiveType, Type};
use crate::runtime::values::*;
use std::collections::HashMap;

// Macro to eliminate repetitive registration code
macro_rules! register_array_fn {
    (
        $functions:expr,
        $name:expr,
        $invoke:expr,
        $params:expr,
        $return_type:expr
    ) => {
        $functions.insert(
            $name.to_string(),
            NativeFunctionValue::new(
                $invoke,
                Type::Function(Box::new(FunctionType {
                    params: $params,
                    return_type: $return_type,
                    is_variadic: false,
                })),
            ),
        );
    };
}

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // Array length
    register_array_fn!(
        functions,
        "native_array_length",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::List(list)) = args.first() {
                RuntimeValue::Int(IntValue::new(list.elements.len() as i64))
            } else {
                RuntimeValue::Int(IntValue::new(0))
            }
        },
        vec![Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))],
        PrimitiveType::int()
    );

    // Array push
    register_array_fn!(
        functions,
        "native_array_push",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Null(NullValue::new());
            }
            if let RuntimeValue::List(mut list) = args[0].clone() {
                list.elements.push(args[1].clone());
                RuntimeValue::List(list)
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        },
        vec![
            Type::List(Box::new(ListType {
                element_type: PrimitiveType::any(),
            })),
            PrimitiveType::any(),
        ],
        Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))
    );

    // Array slice
    register_array_fn!(
        functions,
        "native_array_slice",
        |args: Vec<RuntimeValue>| {
            if args.len() < 3 {
                return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
            }

            let list = match &args[0] {
                RuntimeValue::List(l) => l.clone(),
                _ => return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
            };

            let start = match &args[1] {
                RuntimeValue::Int(i) => i.value.max(0) as usize,
                _ => 0,
            };

            let end = match &args[2] {
                RuntimeValue::Int(i) => i.value.max(0) as usize,
                _ => list.elements.len(),
            };

            let end = end.min(list.elements.len());
            let start = start.min(end);

            let sliced = list.elements[start..end].to_vec();
            RuntimeValue::List(ListValue::new(sliced, list.element_type))
        },
        vec![
            Type::List(Box::new(ListType {
                element_type: PrimitiveType::any(),
            })),
            PrimitiveType::int(),
            PrimitiveType::int(),
        ],
        Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))
    );

    // Array reverse
    register_array_fn!(
        functions,
        "native_array_reverse",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::List(list)) = args.first() {
                let mut reversed = list.clone();
                reversed.elements.reverse();
                RuntimeValue::List(reversed)
            } else {
                RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()))
            }
        },
        vec![Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))],
        Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))
    );

    // Array pop
    register_array_fn!(
        functions,
        "native_array_pop",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::List(mut list)) = args.first().cloned() {
                if !list.elements.is_empty() {
                    list.elements.pop().unwrap()
                } else {
                    RuntimeValue::Null(NullValue::new())
                }
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        },
        vec![Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))],
        PrimitiveType::any()
    );

    // Array shift
    register_array_fn!(
        functions,
        "native_array_shift",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::List(mut list)) = args.first().cloned() {
                if !list.elements.is_empty() {
                    list.elements.remove(0)
                } else {
                    RuntimeValue::Null(NullValue::new())
                }
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        },
        vec![Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))],
        PrimitiveType::any()
    );

    // Array unshift
    register_array_fn!(
        functions,
        "native_array_unshift",
        |args: Vec<RuntimeValue>| {
            if args.len() < 2 {
                return RuntimeValue::Null(NullValue::new());
            }
            if let RuntimeValue::List(mut list) = args[0].clone() {
                list.elements.insert(0, args[1].clone());
                RuntimeValue::List(list)
            } else {
                RuntimeValue::Null(NullValue::new())
            }
        },
        vec![
            Type::List(Box::new(ListType {
                element_type: PrimitiveType::any(),
            })),
            PrimitiveType::any(),
        ],
        Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))
    );

    // Array sort
    register_array_fn!(
        functions,
        "native_array_sort",
        |args: Vec<RuntimeValue>| {
            if let Some(RuntimeValue::List(mut list)) = args.first().cloned() {
                // Simple numeric/string sort for basic types
                list.elements.sort_by(|a, b| {
                    match (a, b) {
                        (RuntimeValue::Int(x), RuntimeValue::Int(y)) => x.value.cmp(&y.value),
                        (RuntimeValue::Float(x), RuntimeValue::Float(y)) => {
                            x.value.partial_cmp(&y.value).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (RuntimeValue::Str(x), RuntimeValue::Str(y)) => x.value.cmp(&y.value),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                RuntimeValue::List(list)
            } else {
                RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()))
            }
        },
        vec![Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))],
        Type::List(Box::new(ListType {
            element_type: PrimitiveType::any(),
        }))
    );
}
