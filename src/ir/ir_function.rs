use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::{DynamicValue, RuntimeValue};

use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct IRFunctionValue {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Instruction>,
    pub is_async: bool,
}

impl IRFunctionValue {
    pub fn new(name: String, params: Vec<String>, body: Vec<Instruction>, is_async: bool) -> Self {
        Self {
            name,
            params,
            body,
            is_async,
        }
    }
}

impl DynamicValue for IRFunctionValue {
    fn get_type(&self) -> Type {
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any(); self.params.len()],
            return_type: PrimitiveType::any(),
            is_variadic: false,
        }))
    }

    fn to_string(&self) -> String {
        format!("[IR Function: {}]", self.name)
    }

    fn clone_boxed(&self) -> Box<dyn DynamicValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> &str {
        "IRFunction"
    }

    fn call(&self, _args: Vec<RuntimeValue>) -> Result<RuntimeValue, String> {
        Err("IR functions must be called through VM.call_function".to_string())
    }
}
