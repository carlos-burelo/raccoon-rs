/// FutureType - Asynchronous computation result with metadata system
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::metadata::{MethodMetadata, ParamMetadata, TypeMetadata};
use crate::runtime::types::{CallbackExecutor, TypeHandler};
use crate::runtime::{BoolValue, FutureState, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

// ============================================================================
// FutureType - Asynchronous computation result (Future<T>)
// ============================================================================

pub struct FutureType;

impl FutureType {
    /// Returns complete type metadata with all methods
    pub fn metadata() -> TypeMetadata {
        TypeMetadata::new("future", "Future type for asynchronous computations")
            .with_instance_methods(vec![
                MethodMetadata::new("isPending", "bool", "Check if future is pending"),
                MethodMetadata::new("isResolved", "bool", "Check if future is resolved"),
                MethodMetadata::new("isRejected", "bool", "Check if future is rejected"),
                MethodMetadata::new("toString", "str", "Convert to string representation")
                    .with_alias("toStr"),
                MethodMetadata::new("then", "future", "Chain callback on resolution")
                    .with_params(vec![ParamMetadata::new("callback", "function")])
                    .async_method(),
                MethodMetadata::new("catch", "any", "Handle rejection with callback")
                    .with_params(vec![ParamMetadata::new("callback", "function")])
                    .async_method(),
            ])
    }
}

#[async_trait]
impl TypeHandler for FutureType {
    fn type_name(&self) -> &str {
        "future"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let future = match value {
            RuntimeValue::Future(f) => f,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected future, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "isPending" => {
                require_args(&args, 0, method, position, file)?;
                let state = future.state.read().unwrap();
                let is_pending = matches!(*state, FutureState::Pending);
                Ok(RuntimeValue::Bool(BoolValue::new(is_pending)))
            }
            "isResolved" => {
                require_args(&args, 0, method, position, file)?;
                let state = future.state.read().unwrap();
                let is_resolved = matches!(*state, FutureState::Resolved(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_resolved)))
            }
            "isRejected" => {
                require_args(&args, 0, method, position, file)?;
                let state = future.state.read().unwrap();
                let is_rejected = matches!(*state, FutureState::Rejected(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_rejected)))
            }
            "toString" | "toStr" => {
                require_args(&args, 0, method, position, file)?;
                let state = future.state.read().unwrap();
                let status = match *state {
                    FutureState::Pending => "Pending",
                    FutureState::Resolved(_) => "Resolved",
                    FutureState::Rejected(_) => "Rejected",
                };
                Ok(RuntimeValue::Str(StrValue::new(format!(
                    "Future({})",
                    status
                ))))
            }
            _ => Err(method_not_found_error("future", method, position, file)),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(static_method_not_found_error(
            "future", method, position, file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_instance_method(method)
    }

    fn has_static_method(&self, method: &str) -> bool {
        Self::metadata().has_static_method(method)
    }

    fn has_async_instance_method(&self, method: &str) -> bool {
        Self::metadata().has_async_instance_method(method)
    }

    async fn call_async_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        callback_executor: &CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        let future = match value {
            RuntimeValue::Future(f) => f,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected future, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "then" => {
                require_args(&args, 1, method, position, file.clone())?;
                let callback = args[0].clone();

                // Wait for the future to resolve
                loop {
                    let state_result = {
                        let state = future.state.read().unwrap();
                        match &*state {
                            FutureState::Resolved(val) => Some(Ok((**val).clone())),
                            FutureState::Rejected(err) => Some(Err(err.clone())),
                            FutureState::Pending => None,
                        }
                    }; // Lock is dropped here

                    match state_result {
                        Some(Ok(val)) => {
                            return callback_executor(callback, vec![val], position).await;
                        }
                        Some(Err(err)) => {
                            return Err(RaccoonError::new(
                                format!("Future rejected: {}", err),
                                position,
                                file,
                            ));
                        }
                        None => {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        }
                    }
                }
            }
            "catch" => {
                require_args(&args, 1, method, position, file.clone())?;
                let callback = args[0].clone();

                // Wait for the future to complete
                loop {
                    let state_result = {
                        let state = future.state.read().unwrap();
                        match &*state {
                            FutureState::Resolved(val) => Some(Ok((**val).clone())),
                            FutureState::Rejected(err) => Some(Err(err.clone())),
                            FutureState::Pending => None,
                        }
                    }; // Lock is dropped here

                    match state_result {
                        Some(Ok(val)) => {
                            return Ok(val);
                        }
                        Some(Err(err)) => {
                            let err_value = RuntimeValue::Str(StrValue::new(err));
                            return callback_executor(callback, vec![err_value], position).await;
                        }
                        None => {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        }
                    }
                }
            }
            _ => self.call_instance_method(value, method, args, position, file),
        }
    }
}
