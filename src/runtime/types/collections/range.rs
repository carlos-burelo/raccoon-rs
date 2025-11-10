/// Refactored RangeType using helpers and metadata system
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::TypeHandler;
use crate::runtime::{IntValue, ListValue, RuntimeValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// RangeType - Range of numbers (Range<T>)
// ============================================================================

pub struct RangeType;

impl RangeType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("range", "Range of integer values with start, end, and step")
            .with_static_methods(vec![
                MethodMetadata::new("new", "list<int>", "Create range from start to end")
                    .with_params(vec![
                        ParamMetadata::new("start", "int"),
                        ParamMetadata::new("end", "int"),
                        ParamMetadata::new("step", "int").optional(),
                    ]),
                MethodMetadata::new(
                    "inclusive",
                    "list<int>",
                    "Create inclusive range from start to end",
                )
                .with_params(vec![
                    ParamMetadata::new("start", "int"),
                    ParamMetadata::new("end", "int"),
                    ParamMetadata::new("step", "int").optional(),
                ]),
            ])
    }
}

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
        // Range is created as a list, so no instance methods
        Err(method_not_found_error("range", method, position, file))
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "new" | "inclusive" => {
                require_args_range(&args, 2, 3, method, position, file.clone())?;

                let start = extract_int(&args[0], "start", position, file.clone())?;
                let end = extract_int(&args[1], "end", position, file.clone())?;

                let step = if args.len() == 3 {
                    extract_int(&args[2], "step", position, file.clone())?
                } else if start < end {
                    1
                } else {
                    -1
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

                let inclusive = method == "inclusive";

                if step > 0 {
                    while if inclusive {
                        current <= end
                    } else {
                        current < end
                    } {
                        elements.push(RuntimeValue::Int(IntValue::new(current)));
                        current += step;
                    }
                } else {
                    while if inclusive {
                        current >= end
                    } else {
                        current > end
                    } {
                        elements.push(RuntimeValue::Int(IntValue::new(current)));
                        current += step;
                    }
                }

                Ok(RuntimeValue::List(ListValue::new(
                    elements,
                    PrimitiveType::int(),
                )))
            }
            _ => Err(static_method_not_found_error(
                "range", method, position, file,
            )),
        }
    }

    fn has_instance_method(&self, _method: &str) -> bool {
        false
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }
}
