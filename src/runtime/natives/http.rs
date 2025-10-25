/// HTTP async function (stub for now)
use crate::runtime::values::NativeAsyncFunctionValue;
use std::collections::HashMap;

pub fn register_async(
    _async_functions: &mut HashMap<String, NativeAsyncFunctionValue>,
) {
    // HTTP functions handled separately by native_bridge.rs
    // TODO: Move HTTP implementation here when refactoring complete
}
