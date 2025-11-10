//! Built-in global objects
//! - Future: Promise-like async computation
//! - Object: Utilities for object manipulation
//! - Type: Type system introspection

use crate::ast::types::PrimitiveType;
use crate::fn_type;
use crate::runtime::{
    ArrayValue, Environment, FutureState, FutureValue, NullValue, RuntimeValue, StrValue,
};
use super::builders::{collect_futures, FutureCollectionStrategy, TypeMethodBuilder};

pub fn register(env: &mut Environment) {
    register_future(env);
    register_object(env);
    register_type(env);
}

fn register_future(env: &mut Environment) {
    let mut builder = TypeMethodBuilder::new("Future");

    // Future.resolve(value)
    builder.add_method(
        "resolve",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            let value = if args.is_empty() {
                RuntimeValue::Null(NullValue::new())
            } else {
                args[0].clone()
            };
            let value_type = value.get_type();
            RuntimeValue::Future(FutureValue::new_resolved(value, value_type))
        },
    );

    // Future.reject(error)
    builder.add_method(
        "reject",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            let error = if args.is_empty() {
                "Unknown error".to_string()
            } else {
                args[0].to_string()
            };
            RuntimeValue::Future(FutureValue::new_rejected(error, PrimitiveType::any()))
        },
    );

    // Future.all(futures)
    builder.add_method(
        "all",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any())),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::Array(list) => list.clone(),
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
                    RuntimeValue::Array(ArrayValue::new(results, PrimitiveType::any())),
                    PrimitiveType::any(),
                ))
            }
        },
    );

    // Future.race(futures)
    builder.add_method(
        "race",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.race() requires a list of futures".to_string(),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::Array(list) => list.clone(),
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
        },
    );

    // Future.allSettled(futures)
    builder.add_method(
        "allSettled",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_resolved(
                    RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any())),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::Array(list) => list.clone(),
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
                    RuntimeValue::Array(ArrayValue::new(results, PrimitiveType::any())),
                    PrimitiveType::any(),
                ))
            }
        },
    );

    // Future.any(futures)
    builder.add_method(
        "any",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Future(FutureValue::new_rejected(
                    "Future.any() requires a list of futures".to_string(),
                    PrimitiveType::any(),
                ));
            }

            let futures_list = match &args[0] {
                RuntimeValue::Array(list) => list.clone(),
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
        },
    );

    builder.build(env);
}

fn register_object(env: &mut Environment) {
    let mut builder = TypeMethodBuilder::new("Object");

    // Object.keys(obj)
    builder.add_method(
        "keys",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::str()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let keys: Vec<RuntimeValue> = obj
                        .properties
                        .keys()
                        .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                        .collect();
                    RuntimeValue::Array(ArrayValue::new(keys, PrimitiveType::str()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let keys: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .keys()
                        .map(|k| RuntimeValue::Str(StrValue::new(k.clone())))
                        .collect();
                    RuntimeValue::Array(ArrayValue::new(keys, PrimitiveType::str()))
                }
                _ => RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::str())),
            }
        },
    );

    // Object.values(obj)
    builder.add_method(
        "values",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let values: Vec<RuntimeValue> = obj.properties.values().cloned().collect();
                    RuntimeValue::Array(ArrayValue::new(values, PrimitiveType::any()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let values: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .values()
                        .cloned()
                        .collect();
                    RuntimeValue::Array(ArrayValue::new(values, PrimitiveType::any()))
                }
                _ => RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any())),
            }
        },
    );

    // Object.entries(obj)
    builder.add_method(
        "entries",
        fn_type!(PrimitiveType::any(), PrimitiveType::any()),
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any()));
            }

            match &args[0] {
                RuntimeValue::Object(obj) => {
                    let entries: Vec<RuntimeValue> = obj
                        .properties
                        .iter()
                        .map(|(k, v)| {
                            let pair = vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()];
                            RuntimeValue::Array(ArrayValue::new(pair, PrimitiveType::any()))
                        })
                        .collect();
                    RuntimeValue::Array(ArrayValue::new(entries, PrimitiveType::any()))
                }
                RuntimeValue::ClassInstance(instance) => {
                    let entries: Vec<RuntimeValue> = instance
                        .properties
                        .read()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| {
                            let pair = vec![RuntimeValue::Str(StrValue::new(k.clone())), v.clone()];
                            RuntimeValue::Array(ArrayValue::new(pair, PrimitiveType::any()))
                        })
                        .collect();
                    RuntimeValue::Array(ArrayValue::new(entries, PrimitiveType::any()))
                }
                _ => RuntimeValue::Array(ArrayValue::new(vec![], PrimitiveType::any())),
            }
        },
    );

    builder.build(env);
}

fn register_type(_env: &mut Environment) {
    // TODO: Implement Type.typeOf, Type.name, Type.isInstance, etc.
}
