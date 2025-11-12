use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::{DynamicValue, RuntimeValue};
use std::collections::HashMap;

use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct IRClassValue {
    pub name: String,
    pub constructor: Option<(Vec<String>, Vec<Instruction>)>,
    pub methods: HashMap<String, IRMethod>,
    pub properties: HashMap<String, RuntimeValue>,
}

#[derive(Debug, Clone)]
pub struct IRMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Instruction>,
    pub is_async: bool,
}

impl IRClassValue {
    pub fn new(
        name: String,
        constructor: Option<(Vec<String>, Vec<Instruction>)>,
        methods: Vec<(String, Vec<String>, Vec<Instruction>, bool)>,
        properties: Vec<(String, RuntimeValue)>,
    ) -> Self {
        let mut method_map = HashMap::new();
        for (method_name, params, body, is_async) in methods {
            method_map.insert(
                method_name.clone(),
                IRMethod {
                    name: method_name,
                    params,
                    body,
                    is_async,
                },
            );
        }

        let mut prop_map = HashMap::new();
        for (prop_name, value) in properties {
            prop_map.insert(prop_name, value);
        }

        Self {
            name,
            constructor,
            methods: method_map,
            properties: prop_map,
        }
    }
}

impl DynamicValue for IRClassValue {
    fn get_type(&self) -> Type {
        PrimitiveType::any()
    }

    fn to_string(&self) -> String {
        format!("[IR Class: {}]", self.name)
    }

    fn clone_boxed(&self) -> Box<dyn DynamicValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> &str {
        "IRClass"
    }
}
