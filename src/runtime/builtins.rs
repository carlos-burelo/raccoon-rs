use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::{
    Environment, IntValue, NativeFunctionValue, NullValue, RuntimeValue, StrValue,
    PrimitiveTypeObject, FutureValue, FutureState, ListValue,
};
use crate::output_style;
use colored::Colorize;
use std::io::{self, Write};
use std::collections::HashMap;

pub fn setup_builtins(env: &mut Environment) {
    register_builtin_functions(env);
    register_builtin_constants(env);
    register_future_object(env);
    register_object_object(env);
}

fn register_builtin_functions(env: &mut Environment) {
    register_print_function(env);
    register_println_function(env);
    register_input_function(env);
    register_len_function(env);
}

fn register_builtin_constants(_env: &mut Environment) {}

fn format_colored_value(value: &RuntimeValue) -> String {
    let plain = value.to_string();
    // Use syntax-aware colorization through the output_style module
    output_style::format_value(&plain)
}

fn register_print_function(env: &mut Environment) {
    let print_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", format_colored_value(arg));
            }
            println!();
            RuntimeValue::Null(NullValue::new())
        },
        Type::Function(Box::new(FunctionType {
            params: vec![],
            return_type: PrimitiveType::void(),
            is_variadic: true,
        })),
    ));
    let _ = env.declare("print".to_string(), print_fn);
}

fn register_println_function(env: &mut Environment) {
    let println_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                println!();
            } else {
                let msg = args[0].to_string();
                let colored = output_style::format_value(&msg);
                println!("{}", colored);
            }
            RuntimeValue::Null(NullValue::new())
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::str()],
            return_type: PrimitiveType::void(),
            is_variadic: false,
        })),
    ));
    let _ = env.declare("println".to_string(), println_fn);
}

fn register_input_function(env: &mut Environment) {
    let input_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let prompt = if !args.is_empty() {
                args[0].to_string()
            } else {
                String::new()
            };

            let breakline = if args.len() > 1 {
                match &args[1] {
                    RuntimeValue::Bool(b) => b.value,
                    _ => false,
                }
            } else {
                false
            };

            if !prompt.is_empty() {
                if breakline {
                    println!("{}", prompt.cyan());
                } else {
                    print!("{}", prompt.cyan());
                    io::stdout().flush().unwrap();
                }
            }

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim_end_matches(&['\r', '\n'][..]).to_string();
                    RuntimeValue::Str(StrValue::new(trimmed))
                }
                Err(_) => RuntimeValue::Null(NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::str(), PrimitiveType::bool()],
            return_type: PrimitiveType::str(),
            is_variadic: false,
        })),
    ));
    let _ = env.declare("input".to_string(), input_fn);
}

fn register_len_function(env: &mut Environment) {
    let len_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.len() != 1 {
                return RuntimeValue::Null(NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
                RuntimeValue::List(l) => RuntimeValue::Int(IntValue::new(l.elements.len() as i64)),
                RuntimeValue::Map(m) => RuntimeValue::Int(IntValue::new(m.entries.len() as i64)),
                _ => RuntimeValue::Null(NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::int(),
            is_variadic: false,
        })),
    ));
    let _ = env.declare("len".to_string(), len_fn);
}

fn register_future_object(env: &mut Environment) {
    let mut static_methods = HashMap::new();

    // Future.resolve(value) - Creates a resolved future
    let resolve_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let value = if args.is_empty() {
                RuntimeValue::Null(NullValue::new())
            } else {
                args[0].clone()
            };
            let value_type = value.get_type();
            RuntimeValue::Future(FutureValue::new_resolved(value, value_type))
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("resolve".to_string(), resolve_fn);

    // Future.reject(error) - Creates a rejected future
    let reject_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let error = if args.is_empty() {
                "Unknown error".to_string()
            } else {
                args[0].to_string()
            };
            RuntimeValue::Future(FutureValue::new_rejected(error, PrimitiveType::any()))
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("reject".to_string(), reject_fn);

    // Future.all(futures) - Waits for all futures to resolve
    let all_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::List(list) => list.clone(),
                _ => {
                    return RuntimeValue::Future(FutureValue::new_rejected(
                        "Future.all() requires a list of futures".to_string(),
                        PrimitiveType::any(),
                    ))
                }
            };

            let mut results = Vec::new();
            let mut has_pending = false;
            let mut first_error = None;

            for future_value in &futures_list.elements {
                match future_value {
                    RuntimeValue::Future(future) => {
                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                results.push(*value);
                            }
                            FutureState::Rejected(error) => {
                                if first_error.is_none() {
                                    first_error = Some(error);
                                }
                            }
                            FutureState::Pending => {
                                has_pending = true;
                            }
                        }
                    }
                    _ => {
                        return RuntimeValue::Future(FutureValue::new_rejected(
                            "Future.all() requires a list of futures".to_string(),
                            PrimitiveType::any(),
                        ))
                    }
                }
            }

            if has_pending {
                RuntimeValue::Future(FutureValue::new_rejected(
                    "Cannot call Future.all() on pending futures. Use 'await' on all futures first.".to_string(),
                    PrimitiveType::any(),
                ))
            } else if let Some(error) = first_error {
                RuntimeValue::Future(FutureValue::new_rejected(error, PrimitiveType::any()))
            } else {
                RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::List(ListValue::new(results, PrimitiveType::any())),
                    PrimitiveType::any(),
                ))
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("all".to_string(), all_fn);

    // Future.race(futures) - Returns the first resolved future
    let race_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.race() requires a list of futures".to_string(),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::List(list) => list.clone(),
                _ => {
                    return RuntimeValue::Future(FutureValue::new_rejected(
                        "Future.race() requires a list of futures".to_string(),
                        PrimitiveType::any(),
                    ))
                }
            };

            if futures_list.elements.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.race() requires at least one future".to_string(),
                    PrimitiveType::any(),
                ));
            }

            // Find the first resolved or rejected future
            for future_value in &futures_list.elements {
                match future_value {
                    RuntimeValue::Future(future) => {
                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                return RuntimeValue::Future(FutureValue::new_resolved(
                                    *value,
                                    PrimitiveType::any(),
                                ));
                            }
                            FutureState::Rejected(error) => {
                                return RuntimeValue::Future(FutureValue::new_rejected(
                                    error,
                                    PrimitiveType::any(),
                                ));
                            }
                            FutureState::Pending => {
                                // Continue to next future
                            }
                        }
                    }
                    _ => {
                        return RuntimeValue::Future(FutureValue::new_rejected(
                            "Future.race() requires a list of futures".to_string(),
                            PrimitiveType::any(),
                        ))
                    }
                }
            }

            // All futures are pending
            RuntimeValue::Future(FutureValue::new_rejected(
                "Cannot call Future.race() when all futures are pending. Use 'await' on at least one future first.".to_string(),
                PrimitiveType::any(),
            ))
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("race".to_string(), race_fn);

    // Future.allSettled(futures) - Waits for all futures to settle (resolve or reject)
    let all_settled_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::List(list) => list.clone(),
                _ => {
                    return RuntimeValue::Future(FutureValue::new_rejected(
                        "Future.allSettled() requires a list of futures".to_string(),
                        PrimitiveType::any(),
                    ))
                }
            };

            let mut results = Vec::new();
            let mut has_pending = false;

            for future_value in &futures_list.elements {
                match future_value {
                    RuntimeValue::Future(future) => {
                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                // Create a result object { status: "fulfilled", value: ... }
                                let mut result_obj = std::collections::HashMap::new();
                                result_obj.insert(
                                    "status".to_string(),
                                    RuntimeValue::Str(StrValue::new("fulfilled".to_string())),
                                );
                                result_obj.insert("value".to_string(), *value);
                                results.push(RuntimeValue::Object(
                                    crate::runtime::ObjectValue::new(result_obj, PrimitiveType::any()),
                                ));
                            }
                            FutureState::Rejected(error) => {
                                // Create a result object { status: "rejected", reason: ... }
                                let mut result_obj = std::collections::HashMap::new();
                                result_obj.insert(
                                    "status".to_string(),
                                    RuntimeValue::Str(StrValue::new("rejected".to_string())),
                                );
                                result_obj.insert(
                                    "reason".to_string(),
                                    RuntimeValue::Str(StrValue::new(error)),
                                );
                                results.push(RuntimeValue::Object(
                                    crate::runtime::ObjectValue::new(result_obj, PrimitiveType::any()),
                                ));
                            }
                            FutureState::Pending => {
                                has_pending = true;
                            }
                        }
                    }
                    _ => {
                        return RuntimeValue::Future(FutureValue::new_rejected(
                            "Future.allSettled() requires a list of futures".to_string(),
                            PrimitiveType::any(),
                        ))
                    }
                }
            }

            if has_pending {
                RuntimeValue::Future(FutureValue::new_rejected(
                    "Cannot call Future.allSettled() on pending futures. Use 'await' on all futures first.".to_string(),
                    PrimitiveType::any(),
                ))
            } else {
                RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::List(ListValue::new(results, PrimitiveType::any())),
                    PrimitiveType::any(),
                ))
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("allSettled".to_string(), all_settled_fn);

    // Future.any(futures) - Returns the first resolved future (ignores rejections)
    let any_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.any() requires a list of futures".to_string(),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::List(list) => list.clone(),
                _ => {
                    return RuntimeValue::Future(FutureValue::new_rejected(
                        "Future.any() requires a list of futures".to_string(),
                        PrimitiveType::any(),
                    ))
                }
            };

            if futures_list.elements.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.any() requires at least one future".to_string(),
                    PrimitiveType::any(),
                ));
            }

            // Find the first resolved future (ignore rejected ones)
            let mut all_rejected = true;
            let mut errors = Vec::new();

            for future_value in &futures_list.elements {
                match future_value {
                    RuntimeValue::Future(future) => {
                        let state = future.state.read().unwrap().clone();
                        match state {
                            FutureState::Resolved(value) => {
                                // Found a resolved future, return it
                                return RuntimeValue::Future(FutureValue::new_resolved(
                                    *value,
                                    PrimitiveType::any(),
                                ));
                            }
                            FutureState::Rejected(error) => {
                                errors.push(error);
                            }
                            FutureState::Pending => {
                                all_rejected = false;
                            }
                        }
                    }
                    _ => {
                        return RuntimeValue::Future(FutureValue::new_rejected(
                            "Future.any() requires a list of futures".to_string(),
                            PrimitiveType::any(),
                        ))
                    }
                }
            }

            if all_rejected && !errors.is_empty() {
                // All futures rejected
                RuntimeValue::Future(FutureValue::new_rejected(
                    format!("All futures were rejected: {}", errors.join(", ")),
                    PrimitiveType::any(),
                ))
            } else {
                // Some futures are still pending
                RuntimeValue::Future(FutureValue::new_rejected(
                    "Cannot call Future.any() when all resolved futures are rejected and some are pending. Use 'await' on futures first.".to_string(),
                    PrimitiveType::any(),
                ))
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("any".to_string(), any_fn);

    let future_object = RuntimeValue::PrimitiveTypeObject(PrimitiveTypeObject::new(
        "Future".to_string(),
        static_methods,
        HashMap::new(),
        PrimitiveType::any(),
    ));

    let _ = env.declare("Future".to_string(), future_object);
}

fn register_object_object(env: &mut Environment) {
    let mut static_methods = HashMap::new();

    // Object.keys(obj) - Returns an array of object's own property names
    let keys_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let keys: Vec<RuntimeValue> = obj
                        .properties
                        .keys()
                        .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                        .collect();
                    RuntimeValue::List(ListValue::new(keys, PrimitiveType::str()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let keys: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .keys()
                        .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                        .collect();
                    RuntimeValue::List(ListValue::new(keys, PrimitiveType::str()))
                }
                _ => RuntimeValue::List(ListValue::new(vec![], PrimitiveType::str())),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("keys".to_string(), keys_fn);

    // Object.values(obj) - Returns an array of object's own property values
    let values_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let values: Vec<RuntimeValue> = obj
                        .properties
                        .values()
                        .cloned()
                        .collect();
                    RuntimeValue::List(ListValue::new(values, PrimitiveType::any()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let values: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .values()
                        .cloned()
                        .collect();
                    RuntimeValue::List(ListValue::new(values, PrimitiveType::any()))
                }
                _ => RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("values".to_string(), values_fn);

    // Object.entries(obj) - Returns an array of [key, value] pairs
    let entries_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let entries: Vec<RuntimeValue> = obj
                        .properties
                        .iter()
                        .map(|(k, v)| {
                            RuntimeValue::List(ListValue::new(
                                vec![
                                    RuntimeValue::Str(StrValue::new(k.clone())),
                                    v.clone(),
                                ],
                                PrimitiveType::any(),
                            ))
                        })
                        .collect();
                    RuntimeValue::List(ListValue::new(entries, PrimitiveType::any()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let entries: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| {
                            RuntimeValue::List(ListValue::new(
                                vec![
                                    RuntimeValue::Str(StrValue::new(k.clone())),
                                    v.clone(),
                                ],
                                PrimitiveType::any(),
                            ))
                        })
                        .collect();
                    RuntimeValue::List(ListValue::new(entries, PrimitiveType::any()))
                }
                _ => RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        })),
    ));
    static_methods.insert("entries".to_string(), entries_fn);

    // Object.assign(target, ...sources) - Copies properties from sources to target
    let assign_fn = Box::new(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Object(crate::runtime::ObjectValue::new(HashMap::new(), PrimitiveType::any()));
            }

            let mut target = args[0].clone();

            match &mut target {
                RuntimeValue::Object(target_obj) => {
                    for source in args.iter().skip(1) {
                        if let RuntimeValue::Object(source_obj) = source {
                            for (key, value) in &source_obj.properties {
                                target_obj.properties.insert(key.clone(), value.clone());
                            }
                        }
                    }
                    target
                }
                _ => target,
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::any(),
            is_variadic: true,
        })),
    ));
    static_methods.insert("assign".to_string(), assign_fn);

    let object_object = RuntimeValue::PrimitiveTypeObject(PrimitiveTypeObject::new(
        "Object".to_string(),
        static_methods,
        HashMap::new(),
        PrimitiveType::any(),
    ));

    let _ = env.declare("Object".to_string(), object_object);
}
