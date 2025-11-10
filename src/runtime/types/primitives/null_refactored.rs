/// Refactored NullType using helpers and metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct NullTypeRefactored;

impl NullTypeRefactored {
    /// Returns complete type metadata
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("null", "Null/None value type")
            .with_instance_methods(vec![
                MethodMetadata::new("toString", "str", "Convert to string representation")
                    .with_alias("toStr"),
            ])
    }

    /// Validate value is null
    fn validate_null(
        value: &RuntimeValue,
        position: Position,
        file: Option<String>,
    ) -> Result<(), RaccoonError> {
        match value {
            RuntimeValue::Null(_) => Ok(()),
            _ => Err(RaccoonError::new(
                format!("Expected null, got {}", value.get_name()),
                position,
                file,
            )),
        }
    }
}

#[async_trait]
impl TypeHandler for NullTypeRefactored {
    fn type_name(&self) -> &str {
        "null"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Self::validate_null(value, position, file.clone())?;

        match method {
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new("null".to_string())))
            }
            _ => Err(method_not_found_error("null", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error("null", method, position, file))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }

    fn has_async_instance_method(&self, _method: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::NullValue;

    #[test]
    fn test_null_to_string() {
        let handler = NullTypeRefactored;
        let mut value = RuntimeValue::Null(NullValue::new());
        let result = handler
            .call_instance_method(&mut value, "toStr", vec![], Position::default(), None)
            .unwrap();

        match result {
            RuntimeValue::Str(s) => assert_eq!(s.value, "null"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_metadata() {
        let metadata = NullTypeRefactored::metadata();
        assert_eq!(metadata.type_name, "null");
        assert!(metadata.has_instance_method("toStr"));
        assert!(metadata.has_instance_method("toString")); // alias
    }
}
