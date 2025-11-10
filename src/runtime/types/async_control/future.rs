use crate::error::RaccoonError;
use crate::runtime::types::{CallbackExecutor, TypeHandler};
use crate::runtime::{BoolValue, FutureState, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct FutureType;

#[async_trait]
impl TypeHandler for FutureType {
    fn type_name(&self) -> &str {
        "future"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        _args: Vec<RuntimeValue>,
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
                let state = future.state.read().unwrap();
                let is_pending = matches!(*state, FutureState::Pending);
                Ok(RuntimeValue::Bool(BoolValue::new(is_pending)))
            }
            "isResolved" => {
                let state = future.state.read().unwrap();
                let is_resolved = matches!(*state, FutureState::Resolved(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_resolved)))
            }
            "isRejected" => {
                let state = future.state.read().unwrap();
                let is_rejected = matches!(*state, FutureState::Rejected(_));
                Ok(RuntimeValue::Bool(BoolValue::new(is_rejected)))
            }
            "toString" | "toStr" => {
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
            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on future", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!("Static method '{}' not found on future type", method),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isPending"
                | "isResolved"
                | "isRejected"
                | "toString"
                | "toStr"
                | "then"
                | "catch"
                | "finally"
        )
    }

    fn has_static_method(&self, _method: &str) -> bool {
        false
    }

    fn has_async_instance_method(&self, method: &str) -> bool {
        matches!(method, "then" | "catch" | "finally")
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
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "then requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = args[0].clone();

                loop {
                    let state_result = {
                        let state = future.state.read().unwrap();
                        match &*state {
                            FutureState::Resolved(val) => Some(Ok((**val).clone())),
                            FutureState::Rejected(err) => Some(Err(err.clone())),
                            FutureState::Pending => None,
                        }
                    };

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
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "catch requires a callback function".to_string(),
                        position,
                        file,
                    ));
                }

                let callback = args[0].clone();

                loop {
                    let state_result = {
                        let state = future.state.read().unwrap();
                        match &*state {
                            FutureState::Resolved(val) => Some(Ok((**val).clone())),
                            FutureState::Rejected(err) => Some(Err(err.clone())),
                            FutureState::Pending => None,
                        }
                    };

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
