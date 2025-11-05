use crate::ast::types::PrimitiveType;
use crate::fn_type;

use crate::runtime::{
    builtins_builders::{collect_futures, FutureCollectionStrategy, TypeMethodBuilder},
    Environment, FutureState, FutureValue, IntValue, ListValue, NativeFunctionValue, NullValue,
    ObjectValue, RuntimeValue, StrValue,
};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};

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
    plain
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
        fn_type!(variadic, PrimitiveType::void()),
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

                println!("{}", msg);
            }
            RuntimeValue::Null(NullValue::new())
        },
        fn_type!(PrimitiveType::str(), PrimitiveType::void()),
    ));
    let _ = env.declare("println".to_string(), println_fn);
}

fn register_input_function(env: &mut Environment) {
    use crate::ast::types::{FunctionType, Type};

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
        fn_type!(PrimitiveType::any(), PrimitiveType::int()),
    ));
    let _ = env.declare("len".to_string(), len_fn);
}

fn register_future_object(env: &mut Environment) {
    let mut builder = TypeMethodBuilder::new("Future");

    let resolve_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
        let value = if args.is_empty() {
            RuntimeValue::Null(NullValue::new())
        } else {
            args[0].clone()
        };
        let value_type = value.get_type();
        RuntimeValue::Future(FutureValue::new_resolved(value, value_type))
    };
    builder.add_method(
        "resolve",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        resolve_impl,
    );

    let reject_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
        let error = if args.is_empty() {
            "Unknown error".to_string()
        } else {
            args[0].to_string()
        };
        RuntimeValue::Future(FutureValue::new_rejected(error, PrimitiveType::any()))
    };
    builder.add_method(
        "reject",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        reject_impl,
    );

    let all_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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

        let (results, has_pending, first_error) =
            collect_futures(&futures_list, FutureCollectionStrategy::All);

        if has_pending {
            RuntimeValue::Future(FutureValue::new_rejected(
                "Cannot call Future.all() on pending futures. Use 'await' on all futures first."
                    .to_string(),
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
    };
    builder.add_method(
        "all",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        all_impl,
    );

    let race_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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
                        FutureState::Pending => {}
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

        RuntimeValue::Future(FutureValue::new_rejected(
            "Cannot call Future.race() when all futures are pending. Use 'await' on at least one future first.".to_string(),
            PrimitiveType::any(),
        ))
    };
    builder.add_method(
        "race",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        race_impl,
    );

    let all_settled_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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

        let (results, has_pending, _) =
            collect_futures(&futures_list, FutureCollectionStrategy::AllSettled);

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
    };
    builder.add_method(
        "allSettled",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        all_settled_impl,
    );

    let any_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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

        let (results, has_pending, _) =
            collect_futures(&futures_list, FutureCollectionStrategy::Any);

        if !results.is_empty() {
            RuntimeValue::Future(FutureValue::new_resolved(
                results[0].clone(),
                PrimitiveType::any(),
            ))
        } else if has_pending {
            RuntimeValue::Future(FutureValue::new_rejected(
                "Cannot call Future.any() when all resolved futures are rejected and some are pending. Use 'await' on futures first.".to_string(),
                PrimitiveType::any(),
            ))
        } else {
            RuntimeValue::Future(FutureValue::new_rejected(
                "All futures were rejected".to_string(),
                PrimitiveType::any(),
            ))
        }
    };
    builder.add_method(
        "any",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        any_impl,
    );

    builder.build(env);
}

fn register_object_object(env: &mut Environment) {
    let mut builder = TypeMethodBuilder::new("Object");

    let keys_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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
    };
    builder.add_method(
        "keys",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        keys_impl,
    );

    let values_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
        if args.is_empty() {
            return RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any()));
        }

        match &args[0] {
            RuntimeValue::Object(obj) => {
                let values: Vec<RuntimeValue> = obj.properties.values().cloned().collect();
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
    };
    builder.add_method(
        "values",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        values_impl,
    );

    let entries_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
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
                            vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()],
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
                            vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()],
                            PrimitiveType::any(),
                        ))
                    })
                    .collect();
                RuntimeValue::List(ListValue::new(entries, PrimitiveType::any()))
            }
            _ => RuntimeValue::List(ListValue::new(vec![], PrimitiveType::any())),
        }
    };
    builder.add_method(
        "entries",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        entries_impl,
    );

    let assign_impl: fn(Vec<RuntimeValue>) -> RuntimeValue = |args: Vec<RuntimeValue>| {
        if args.is_empty() {
            return RuntimeValue::Object(ObjectValue::new(HashMap::new(), PrimitiveType::any()));
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
    };
    builder.add_method(
        "assign",
        fn_type!(variadic, PrimitiveType::any()),
        assign_impl,
    );

    builder.build(env);
}
