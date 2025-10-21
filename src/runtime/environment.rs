use super::values::RuntimeValue;
use crate::error::RaccoonError;
use crate::tokens::Position;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub file: Option<String>,
    scopes: Vec<HashMap<String, RuntimeValue>>,
}

impl Environment {
    pub fn new(file: Option<String>) -> Self {
        Self {
            file,
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: String, value: RuntimeValue) -> Result<(), RaccoonError> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(RaccoonError::new(
                    format!("Variable '{}' is already declared", name),
                    (0, 0),
                    self.file.clone(),
                ));
            }
            scope.insert(name, value);
            Ok(())
        } else {
            Err(RaccoonError::new(
                "No scope available".to_string(),
                (0, 0),
                self.file.clone(),
            ))
        }
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue, position: Position) -> Result<(), RaccoonError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(RaccoonError::new(
            format!("Variable '{}' is not declared", name),
            position,
            self.file.clone(),
        ))
    }

    pub fn get(&self, name: &str, position: Position) -> Result<RuntimeValue, RaccoonError> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(RaccoonError::new(
            format!("Variable '{}' is not declared", name),
            position,
            self.file.clone(),
        ))
    }

    pub fn exists(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }
}
