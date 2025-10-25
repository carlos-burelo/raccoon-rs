/// Array functions: length, push, slice, reverse
use crate::ast::types::{FunctionType, ListType, PrimitiveType, Type};
use crate::runtime::values::*;
use std::collections::HashMap;

pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    // Array length
    functions.insert(
        "native_array_length".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::List(list)) = args.first() {
                    RuntimeValue::Int(IntValue::new(list.elements.len() as i64))
                } else {
                    RuntimeValue::Int(IntValue::new(0))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ),
    );

    // Array push
    functions.insert(
        "native_array_push".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // Array slice
    functions.insert(
        "native_array_slice".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::int(),
                    PrimitiveType::int(),
                ],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // Array reverse
    functions.insert(
        "native_array_reverse".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if let Some(RuntimeValue::List(list)) = args.first() {
                    let mut reversed = list.clone();
                    reversed.elements.reverse();
                    RuntimeValue::List(reversed)
                } else {
                    RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()))
                }
            },
            Type::Function(Box::new(FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // Array pop
    functions.insert(
        "native_array_pop".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ),
    );

    // Array shift
    functions.insert(
        "native_array_shift".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ),
    );

    // Array unshift
    functions.insert(
        "native_array_unshift".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ),
    );

    // Array sort
    functions.insert(
        "native_array_sort".to_string(),
        NativeFunctionValue::new(
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
            Type::Function(Box::new(FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ),
    );
}
