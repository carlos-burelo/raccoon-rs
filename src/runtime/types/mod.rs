// Type system - modular organization
pub mod primitives;
pub mod collections;
pub mod objects;
pub mod async_control;
pub mod special;
pub mod registry;

use crate::error::RaccoonError;
use crate::runtime::RuntimeValue;
use crate::tokens::Position;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

pub type CallbackExecutor = Box<
    dyn Fn(
            RuntimeValue,
            Vec<RuntimeValue>,
            Position,
        ) -> Pin<Box<dyn Future<Output = Result<RuntimeValue, RaccoonError>> + Send>>
        + Send
        + Sync,
>;

#[async_trait]
pub trait TypeHandler: Send + Sync {
    fn type_name(&self) -> &str;

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError>;

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError>;

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            format!(
                "Static property '{}' not found on type '{}'",
                property,
                self.type_name()
            ),
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool;

    fn has_static_method(&self, method: &str) -> bool;

    async fn call_async_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        _callback_executor: &CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        self.call_instance_method(value, method, args, position, file)
    }

    fn has_async_instance_method(&self, _method: &str) -> bool {
        false
    }
}
