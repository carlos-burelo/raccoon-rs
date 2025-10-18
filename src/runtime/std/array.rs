use crate::ast::types::{ListType, PrimitiveType, Type};
use crate::runtime::values::{BoolValue, IntValue, ListValue, NullValue, RuntimeValue};
use std::collections::HashMap;

pub struct ArrayModule;

impl ArrayModule {
    pub fn name() -> &'static str {
        "std:array"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("push".to_string(), Self::create_push_fn());
        exports.insert("pop".to_string(), Self::create_pop_fn());
        exports.insert("shift".to_string(), Self::create_shift_fn());
        exports.insert("unshift".to_string(), Self::create_unshift_fn());
        exports.insert("reverse".to_string(), Self::create_reverse_fn());
        exports.insert("slice".to_string(), Self::create_slice_fn());
        exports.insert("concat".to_string(), Self::create_concat_fn());
        exports.insert("indexOf".to_string(), Self::create_index_of_fn());
        exports.insert("includes".to_string(), Self::create_includes_fn());
        exports.insert("join".to_string(), Self::create_join_fn());
        exports.insert("sort".to_string(), Self::create_sort_fn());
        exports.insert("fill".to_string(), Self::create_fill_fn());
        exports.insert("flat".to_string(), Self::create_flat_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "push" => Some(Self::create_push_fn()),
            "pop" => Some(Self::create_pop_fn()),
            "shift" => Some(Self::create_shift_fn()),
            "unshift" => Some(Self::create_unshift_fn()),
            "reverse" => Some(Self::create_reverse_fn()),
            "slice" => Some(Self::create_slice_fn()),
            "concat" => Some(Self::create_concat_fn()),
            "indexOf" => Some(Self::create_index_of_fn()),
            "includes" => Some(Self::create_includes_fn()),
            "join" => Some(Self::create_join_fn()),
            "sort" => Some(Self::create_sort_fn()),
            "fill" => Some(Self::create_fill_fn()),
            "flat" => Some(Self::create_flat_fn()),
            _ => None,
        }
    }

    fn create_push_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut new_elements = list.elements.clone();
                        new_elements.push(args[1].clone());
                        RuntimeValue::List(ListValue::new(new_elements, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: PrimitiveType::void(),
                is_variadic: false,
            })),
        ))
    }

    fn create_pop_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        if list.elements.is_empty() {
                            RuntimeValue::Null(NullValue::new())
                        } else {
                            list.elements
                                .last()
                                .cloned()
                                .unwrap_or(RuntimeValue::Null(NullValue::new()))
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ))
    }

    fn create_shift_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        if list.elements.is_empty() {
                            RuntimeValue::Null(NullValue::new())
                        } else {
                            list.elements
                                .first()
                                .cloned()
                                .unwrap_or(RuntimeValue::Null(NullValue::new()))
                        }
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::any(),
                is_variadic: false,
            })),
        ))
    }

    fn create_unshift_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut new_elements = vec![args[1].clone()];
                        new_elements.extend(list.elements.clone());
                        RuntimeValue::List(ListValue::new(new_elements, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: PrimitiveType::void(),
                is_variadic: false,
            })),
        ))
    }

    fn create_reverse_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut new_elements = list.elements.clone();
                        new_elements.reverse();
                        RuntimeValue::List(ListValue::new(new_elements, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::void(),
                is_variadic: false,
            })),
        ))
    }

    fn create_slice_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::List(list) => {
                        let start = if args.len() > 1 {
                            match &args[1] {
                                RuntimeValue::Int(i) => i.value.max(0) as usize,
                                _ => 0,
                            }
                        } else {
                            0
                        };

                        let end = if args.len() > 2 {
                            match &args[2] {
                                RuntimeValue::Int(i) => (i.value.max(0) as usize).min(list.elements.len()),
                                _ => list.elements.len(),
                            }
                        } else {
                            list.elements.len()
                        };

                        let sliced = if start < end && start < list.elements.len() {
                            list.elements[start..end.min(list.elements.len())].to_vec()
                        } else {
                            vec![]
                        };

                        RuntimeValue::List(ListValue::new(sliced, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: true,
            })),
        ))
    }

    fn create_concat_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }

                let mut result = Vec::new();
                let mut element_type = PrimitiveType::any();

                for arg in args {
                    match arg {
                        RuntimeValue::List(list) => {
                            result.extend(list.elements.clone());
                            if element_type == PrimitiveType::any() {
                                element_type = list.element_type.clone();
                            }
                        }
                        _ => result.push(arg),
                    }
                }

                RuntimeValue::List(ListValue::new(result, element_type))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: true,
            })),
        ))
    }

    fn create_index_of_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Int(IntValue::new(-1));
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let search_value = &args[1];
                        for (i, elem) in list.elements.iter().enumerate() {
                            if Self::values_equal(elem, search_value) {
                                return RuntimeValue::Int(IntValue::new(i as i64));
                            }
                        }
                        RuntimeValue::Int(IntValue::new(-1))
                    }
                    _ => RuntimeValue::Int(IntValue::new(-1)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_includes_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Bool(BoolValue::new(false));
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let search_value = &args[1];
                        for elem in &list.elements {
                            if Self::values_equal(elem, search_value) {
                                return RuntimeValue::Bool(BoolValue::new(true));
                            }
                        }
                        RuntimeValue::Bool(BoolValue::new(false))
                    }
                    _ => RuntimeValue::Bool(BoolValue::new(false)),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![
                    Type::List(Box::new(ListType {
                        element_type: PrimitiveType::any(),
                    })),
                    PrimitiveType::any(),
                ],
                return_type: PrimitiveType::bool(),
                is_variadic: false,
            })),
        ))
    }

    fn create_join_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let separator = if args.len() > 1 {
                            match &args[1] {
                                RuntimeValue::Str(s) => s.value.clone(),
                                _ => ",".to_string(),
                            }
                        } else {
                            ",".to_string()
                        };

                        let strings: Vec<String> = list
                            .elements
                            .iter()
                            .map(|v| match v {
                                RuntimeValue::Str(s) => s.value.clone(),
                                _ => v.to_string(),
                            })
                            .collect();

                        RuntimeValue::Str(crate::runtime::StrValue::new(strings.join(&separator)))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: PrimitiveType::str(),
                is_variadic: true,
            })),
        ))
    }

    fn create_sort_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let mut elements = list.elements.clone();
                        elements.sort_by(|a, b| {
                            Self::compare_values(a, b)
                        });
                        RuntimeValue::List(ListValue::new(elements, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: false,
            })),
        ))
    }

    fn create_fill_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() < 2 {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let fill_value = &args[1];
                        let elements = vec![fill_value.clone(); list.elements.len()];
                        RuntimeValue::List(ListValue::new(elements, list.element_type.clone()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
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
        ))
    }

    fn create_flat_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }

                match &args[0] {
                    RuntimeValue::List(list) => {
                        let depth = if args.len() > 1 {
                            match &args[1] {
                                RuntimeValue::Int(i) => i.value as usize,
                                _ => 1,
                            }
                        } else {
                            1
                        };

                        let flattened = Self::flatten_array(&list.elements, depth);
                        RuntimeValue::List(ListValue::new(flattened, PrimitiveType::any()))
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                }))],
                return_type: Type::List(Box::new(ListType {
                    element_type: PrimitiveType::any(),
                })),
                is_variadic: true,
            })),
        ))
    }

    // Helper functions

    fn values_equal(a: &RuntimeValue, b: &RuntimeValue) -> bool {
        match (a, b) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => i1.value == i2.value,
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => f1.value == f2.value,
            (RuntimeValue::Str(s1), RuntimeValue::Str(s2)) => s1.value == s2.value,
            (RuntimeValue::Bool(b1), RuntimeValue::Bool(b2)) => b1.value == b2.value,
            (RuntimeValue::Null(_), RuntimeValue::Null(_)) => true,
            _ => false,
        }
    }

    fn compare_values(a: &RuntimeValue, b: &RuntimeValue) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (a, b) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => i1.value.cmp(&i2.value),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => {
                f1.value.partial_cmp(&f2.value).unwrap_or(Ordering::Equal)
            }
            (RuntimeValue::Str(s1), RuntimeValue::Str(s2)) => s1.value.cmp(&s2.value),
            (RuntimeValue::Bool(b1), RuntimeValue::Bool(b2)) => b1.value.cmp(&b2.value),
            _ => Ordering::Equal,
        }
    }

    fn flatten_array(elements: &[RuntimeValue], depth: usize) -> Vec<RuntimeValue> {
        if depth == 0 {
            return elements.to_vec();
        }

        let mut result = Vec::new();
        for elem in elements {
            match elem {
                RuntimeValue::List(list) => {
                    result.extend(Self::flatten_array(&list.elements, depth - 1));
                }
                _ => result.push(elem.clone()),
            }
        }
        result
    }
}
