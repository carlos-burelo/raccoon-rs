use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

pub fn core_array_join(args: Vec<RuntimeValue>) -> RuntimeValue {
    let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());
    let separator = String::from_raccoon(&args[1]).unwrap_or_default();

    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
        let strings: Vec<String> = arr
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                _ => v.to_string(),
            })
            .collect();
        strings.join(&separator).to_raccoon()
    } else {
        "".to_string().to_raccoon()
    }
}

pub fn core_array_sort(args: Vec<RuntimeValue>) -> RuntimeValue {
    let array_json = String::from_raccoon(&args[0]).unwrap_or_else(|_| "[]".to_string());

    if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
        arr.sort_by(|a, b| match (a, b) {
            (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => n1
                .as_f64()
                .partial_cmp(&n2.as_f64())
                .unwrap_or(std::cmp::Ordering::Equal),
            (serde_json::Value::String(s1), serde_json::Value::String(s2)) => s1.cmp(s2),
            _ => std::cmp::Ordering::Equal,
        });
        serde_json::to_string(&arr)
            .unwrap_or_else(|_| "[]".to_string())
            .to_raccoon()
    } else {
        "[]".to_string().to_raccoon()
    }
}

primitive! {
    array::core_array_reverse(array_json: String) -> String {
        if let Ok(mut arr) = serde_json::from_str::<Vec<serde_json::Value>>(&array_json) {
            arr.reverse();
            serde_json::to_string(&arr)
                .unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }
}

pub fn register_array_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, array, {
        core_array_join: 2..=2,
        core_array_sort: 1..=1,
        core_array_reverse: 1..=1,
    });
}
