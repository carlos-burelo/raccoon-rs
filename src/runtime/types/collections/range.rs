use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ListValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// RangeType - Range of numbers (Range<T>)
// ============================================================================

pub struct RangeType;

#[async_trait]
impl TypeHandler for RangeType {
    fn type_name(&self) -> &str {
        "range"
    }

    fn call_instance_method(
        &self,
        _value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        // Range would typically be represented as a custom struct
        // For now, using a placeholder implementation
        Err(RaccoonError::new(
            format!("Method '{}' not found on range", method),
            position,
            file,
        ))
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "new" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(RaccoonError::new(
                        "new requires 2 or 3 arguments (start, end, [step])".to_string(),
                        position,
                        file,
                    ));
                }

                let start = match &args[0] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "Range start must be an integer".to_string(),
                            position,
                            file,
                        ))
                    }
                };

                let end = match &args[1] {
                    RuntimeValue::Int(i) => i.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "Range end must be an integer".to_string(),
                            position,
                            file,
                        ))
                    }
                };

                let step = if args.len() == 3 {
                    match &args[2] {
                        RuntimeValue::Int(i) => i.value,
                        _ => {
                            return Err(RaccoonError::new(
                                "Range step must be an integer".to_string(),
                                position,
                                file,
                            ))
                        }
                    }
                } else {
                    if start < end {
                        1
                    } else {
                        -1
                    }
                };

                if step == 0 {
                    return Err(RaccoonError::new(
                        "Range step cannot be zero".to_string(),
                        position,
                        file,
                    ));
                }

                // Generate range as a list
                let mut elements = Vec::new();
                let mut current = start;

                if step > 0 {
                    while current < end {
                        elements.push(RuntimeValue::Int(IntValue::new(current)));
                        current += step;
                    }
                } else {
                    while current > end {
                        elements.push(RuntimeValue::Int(IntValue::new(current)));
                        current += step;
                    }
                }

                Ok(RuntimeValue::List(ListValue::new(
                    elements,
                    PrimitiveType::int(),
                )))
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on range type", method),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, _method: &str) -> bool {
        false
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(method, "new")
    }
}
