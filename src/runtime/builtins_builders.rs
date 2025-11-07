use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::type_object::TypeKind;
use crate::runtime::type_object_builder::TypeObjectBuilder;
use crate::runtime::values::*;
use crate::runtime::{FutureState, FutureValue};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum FutureCollectionStrategy {
    All,
    AllSettled,
    Race,
    Any,
}

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
                    (FutureState::Resolved(value), FutureCollectionStrategy::AllSettled) => {
                        // For allSettled, wrap resolved values in object with status and value
                        let mut result_obj = HashMap::new();
                        result_obj.insert(
                            "status".to_string(),
                            RuntimeValue::Str(StrValue::new("fulfilled".to_string())),
                        );
                        result_obj.insert("value".to_string(), *value);
                        results.push(RuntimeValue::Object(ObjectValue::new(
                            result_obj,
                            PrimitiveType::any(),
                        )));
                    }
                    (FutureState::Resolved(value), _) => {
                        // For other strategies, just return the value directly
                        results.push(*value);
                    }
                    (FutureState::Rejected(error), FutureCollectionStrategy::AllSettled) => {
                        // For allSettled, wrap rejected values in object with status and reason
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

pub fn error_future(message: &str, inner_type: Type) -> RuntimeValue {
    RuntimeValue::Future(FutureValue::new_rejected(message.to_string(), inner_type))
}

pub fn resolved_future(value: RuntimeValue, inner_type: Type) -> RuntimeValue {
    RuntimeValue::Future(FutureValue::new_resolved(value, inner_type))
}

pub struct TypeMethodBuilder {
    type_name: String,
    methods: HashMap<String, Box<NativeFunctionValue>>,
}

impl TypeMethodBuilder {
    pub fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_string(),
            methods: HashMap::new(),
        }
    }

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

    pub fn build(self, env: &mut crate::runtime::Environment) {
        let mut static_methods_map = HashMap::new();

        for (name, method) in self.methods {
            static_methods_map.insert(name, RuntimeValue::NativeFunction(*method));
        }

        // Determine the TypeKind and Type based on the type name
        let (type_def, type_kind) = match self.type_name.as_str() {
            "Future" => (
                PrimitiveType::any(), // Future is generic, but we use any for now
                TypeKind::Generic {
                    name: "Future".to_string(),
                    constraints: vec![],
                },
            ),
            "Object" => (
                PrimitiveType::any(),
                TypeKind::Module {
                    name: "Object".to_string(),
                },
            ),
            _ => (PrimitiveType::any(), TypeKind::Unknown),
        };

        let type_obj = TypeObjectBuilder::new(type_def, type_kind)
            .static_methods(static_methods_map)
            .documentation(format!("Built-in {} type", self.type_name))
            .build();

        let _ = env.declare(self.type_name, RuntimeValue::Type(type_obj));
    }
}

pub fn check_arg_count(args: &[RuntimeValue], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!(
            "Expected {} arguments, got {}",
            expected,
            args.len()
        ));
    }
    Ok(())
}

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

pub fn extract_string(args: &[RuntimeValue], index: usize) -> Result<String, String> {
    match &args.get(index) {
        Some(RuntimeValue::Str(s)) => Ok(s.value.clone()),
        _ => Err(format!("Argument {} is not a string", index)),
    }
}

pub fn extract_int(args: &[RuntimeValue], index: usize) -> Result<i64, String> {
    match &args.get(index) {
        Some(RuntimeValue::Int(i)) => Ok(i.value),
        _ => Err(format!("Argument {} is not an integer", index)),
    }
}

pub fn extract_list(args: &[RuntimeValue], index: usize) -> Result<ListValue, String> {
    match &args.get(index) {
        Some(RuntimeValue::List(l)) => Ok(l.clone()),
        _ => Err(format!("Argument {} is not a list", index)),
    }
}

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
        let (results, has_pending, error) = collect_futures(&list, FutureCollectionStrategy::All);
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
