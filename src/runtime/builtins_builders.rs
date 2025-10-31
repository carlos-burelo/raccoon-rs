/// Builders and helper functions for creating builtin functionality
///
/// This module contains reusable functions and builders that eliminate
/// code duplication in builtin function implementations.

use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::*;
use crate::runtime::{FutureState, FutureValue};
use std::collections::HashMap;

/// Strategy for collecting futures
#[derive(Clone, Copy, Debug)]
pub enum FutureCollectionStrategy {
    /// All futures must be resolved
    All,
    /// All futures must settle (resolved or rejected)
    AllSettled,
    /// First to settle wins
    Race,
    /// First to resolve wins
    Any,
}

/// Collect and process a list of futures with the given strategy
///
/// Returns: (results, has_pending, first_error)
pub fn collect_futures(
    futures_list: &ListValue,
    strategy: FutureCollectionStrategy,
) -> (Vec<RuntimeValue>, bool, Option<String>) {
    let mut results = Vec::new();
    let mut has_pending = false;
    let mut first_error = None;

    for future_value in &futures_list.elements {
        match future_value {
            RuntimeValue::Future(future) => {
                let state = future.state.read().unwrap().clone();
                match (state, strategy) {
                    (FutureState::Resolved(value), _) => {
                        results.push(*value);
                    }
                    (FutureState::Rejected(error), FutureCollectionStrategy::AllSettled) => {
                        // For allSettled, collect the error as an object
                        let mut error_obj = HashMap::new();
                        error_obj.insert(
                            "status".to_string(),
                            RuntimeValue::Str(StrValue::new("rejected".to_string())),
                        );
                        error_obj.insert(
                            "reason".to_string(),
                            RuntimeValue::Str(StrValue::new(error)),
                        );
                        results.push(RuntimeValue::Object(ObjectValue::new(
                            error_obj,
                            PrimitiveType::any(),
                        )));
                    }
                    (FutureState::Rejected(error), _) => {
                        if first_error.is_none() {
                            first_error = Some(error);
                        }
                    }
                    (FutureState::Pending, _) => {
                        has_pending = true;
                    }
                }
            }
            _ => {}
        }
    }

    (results, has_pending, first_error)
}

/// Validate that all elements in a list are futures
pub fn validate_futures_list(value: &RuntimeValue) -> Result<ListValue, String> {
    match value {
        RuntimeValue::List(list) => {
            for element in &list.elements {
                if !matches!(element, RuntimeValue::Future(_)) {
                    return Err("List contains non-future elements".to_string());
                }
            }
            Ok(list.clone())
        }
        _ => Err("Expected a list of futures".to_string()),
    }
}

/// Helper to create a rejected future with an error message
pub fn error_future(message: &str, inner_type: Type) -> RuntimeValue {
    RuntimeValue::Future(FutureValue::new_rejected(message.to_string(), inner_type))
}

/// Helper to create a resolved future with a value
pub fn resolved_future(value: RuntimeValue, inner_type: Type) -> RuntimeValue {
    RuntimeValue::Future(FutureValue::new_resolved(value, inner_type))
}

/// Type method builder for creating static methods on types
///
/// Example:
/// ```ignore
/// let mut builder = TypeMethodBuilder::new("Object");
/// builder
///     .add_method("keys", fn_type, |args| { /* impl */ })
///     .add_method("values", fn_type, |args| { /* impl */ })
///     .build(env);
/// ```
pub struct TypeMethodBuilder {
    type_name: String,
    methods: HashMap<String, Box<NativeFunctionValue>>,
}

impl TypeMethodBuilder {
    /// Create a new type method builder
    pub fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_string(),
            methods: HashMap::new(),
        }
    }

    /// Add a static method to the type
    pub fn add_method(
        &mut self,
        name: &str,
        fn_type: Type,
        implementation: fn(Vec<RuntimeValue>) -> RuntimeValue,
    ) -> &mut Self {
        let native_fn = NativeFunctionValue::new(implementation, fn_type);
        self.methods.insert(name.to_string(), Box::new(native_fn));
        self
    }

    /// Build and register the type object in the environment
    pub fn build(self, env: &mut crate::runtime::Environment) {
        let mut static_methods = HashMap::new();

        for (name, method) in self.methods {
            static_methods.insert(name, method);
        }

        let type_obj = RuntimeValue::PrimitiveTypeObject(PrimitiveTypeObject::new(
            self.type_name.clone(),
            static_methods,
            HashMap::new(),
            PrimitiveType::any(),
        ));

        let _ = env.declare(self.type_name, type_obj);
    }
}

/// Helper function to check argument count
pub fn check_arg_count(args: &[RuntimeValue], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!("Expected {} arguments, got {}", expected, args.len()));
    }
    Ok(())
}

/// Helper function to check argument count range
pub fn check_arg_count_range(args: &[RuntimeValue], min: usize, max: usize) -> Result<(), String> {
    if args.len() < min || args.len() > max {
        return Err(format!(
            "Expected {} to {} arguments, got {}",
            min,
            max,
            args.len()
        ));
    }
    Ok(())
}

/// Extract a string from arguments
pub fn extract_string(args: &[RuntimeValue], index: usize) -> Result<String, String> {
    match &args.get(index) {
        Some(RuntimeValue::Str(s)) => Ok(s.value.clone()),
        _ => Err(format!("Argument {} is not a string", index)),
    }
}

/// Extract an integer from arguments
pub fn extract_int(args: &[RuntimeValue], index: usize) -> Result<i64, String> {
    match &args.get(index) {
        Some(RuntimeValue::Int(i)) => Ok(i.value),
        _ => Err(format!("Argument {} is not an integer", index)),
    }
}

/// Extract a list from arguments
pub fn extract_list(args: &[RuntimeValue], index: usize) -> Result<ListValue, String> {
    match &args.get(index) {
        Some(RuntimeValue::List(l)) => Ok(l.clone()),
        _ => Err(format!("Argument {} is not a list", index)),
    }
}

/// Extract a map from arguments
pub fn extract_map(args: &[RuntimeValue], index: usize) -> Result<MapValue, String> {
    match &args.get(index) {
        Some(RuntimeValue::Map(m)) => Ok(m.clone()),
        _ => Err(format!("Argument {} is not a map", index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::FunctionType;

    #[test]
    fn test_collect_futures_empty() {
        let list = ListValue::new(vec![], PrimitiveType::any());
        let (results, has_pending, error) =
            collect_futures(&list, FutureCollectionStrategy::All);
        assert!(results.is_empty());
        assert!(!has_pending);
        assert!(error.is_none());
    }

    #[test]
    fn test_type_method_builder() {
        let mut builder = TypeMethodBuilder::new("TestType");
        let fn_type = Type::Function(Box::new(FunctionType {
            params: vec![],
            return_type: PrimitiveType::void(),
            is_variadic: false,
        }));

        fn test_impl(_: Vec<RuntimeValue>) -> RuntimeValue {
            RuntimeValue::Null(NullValue::new())
        }

        builder.add_method("test", fn_type, test_impl);
        assert_eq!(builder.methods.len(), 1);
    }
}
