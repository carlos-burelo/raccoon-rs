use crate::runtime::{FromRaccoon, ListValue, Registrar, RuntimeValue, ToRaccoon};

pub fn register_array_module(registrar: &mut Registrar) {
    // length(arr: list) -> i32
    registrar.register_fn(
        "length",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => (list.elements.len() as i32).to_raccoon(),
            _ => (0i32).to_raccoon(),
        },
        1,
        Some(1),
    );

    // push(arr: list, element: any) -> list
    registrar.register_fn(
        "push",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                let mut new_elements = list.elements.clone();
                if args.len() > 1 {
                    new_elements.push(args[1].clone());
                }
                RuntimeValue::List(ListValue::new(new_elements, list.element_type.clone()))
            }
            _ => args[0].clone(),
        },
        2,
        Some(2),
    );

    // pop(arr: list) -> any
    registrar.register_fn(
        "pop",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                if list.elements.is_empty() {
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                } else {
                    list.elements
                        .last()
                        .cloned()
                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()))
                }
            }
            _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
        },
        1,
        Some(1),
    );

    // shift(arr: list) -> any
    registrar.register_fn(
        "shift",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                if list.elements.is_empty() {
                    RuntimeValue::Null(crate::runtime::NullValue::new())
                } else {
                    list.elements
                        .first()
                        .cloned()
                        .unwrap_or(RuntimeValue::Null(crate::runtime::NullValue::new()))
                }
            }
            _ => RuntimeValue::Null(crate::runtime::NullValue::new()),
        },
        1,
        Some(1),
    );

    // slice(arr: list, start: i32, end: i32) -> list
    registrar.register_fn(
        "slice",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                let start = i32::from_raccoon(&args[1]).unwrap_or(0) as usize;
                let end =
                    i32::from_raccoon(&args[2]).unwrap_or(list.elements.len() as i32) as usize;

                let start = start.min(list.elements.len());
                let end = end.min(list.elements.len());

                if start <= end {
                    let sliced = list.elements[start..end].to_vec();
                    RuntimeValue::List(ListValue::new(sliced, list.element_type.clone()))
                } else {
                    RuntimeValue::List(ListValue::new(vec![], list.element_type.clone()))
                }
            }
            _ => RuntimeValue::List(ListValue::new(
                vec![],
                crate::ast::types::PrimitiveType::any(),
            )),
        },
        3,
        Some(3),
    );

    // reverse(arr: list) -> list
    registrar.register_fn(
        "reverse",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                let mut reversed = list.elements.clone();
                reversed.reverse();
                RuntimeValue::List(ListValue::new(reversed, list.element_type.clone()))
            }
            _ => args[0].clone(),
        },
        1,
        Some(1),
    );

    // contains(arr: list, element: any) -> bool
    registrar.register_fn(
        "contains",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                let found = list
                    .elements
                    .iter()
                    .any(|elem| elem.to_string() == args[1].to_string());
                found.to_raccoon()
            }
            _ => false.to_raccoon(),
        },
        2,
        Some(2),
    );

    // index_of(arr: list, element: any) -> i32
    registrar.register_fn(
        "index_of",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                for (i, elem) in list.elements.iter().enumerate() {
                    if elem.to_string() == args[1].to_string() {
                        return (i as i32).to_raccoon();
                    }
                }
                (-1i32).to_raccoon()
            }
            _ => (-1i32).to_raccoon(),
        },
        2,
        Some(2),
    );

    // join(arr: list, separator: string) -> string
    registrar.register_fn(
        "join",
        Some("array"),
        |args| match &args[0] {
            RuntimeValue::List(list) => {
                let sep = String::from_raccoon(&args[1]).unwrap_or_default();
                let joined = list
                    .elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(&sep);
                joined.to_raccoon()
            }
            _ => String::new().to_raccoon(),
        },
        2,
        Some(2),
    );
}
