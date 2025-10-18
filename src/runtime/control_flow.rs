use super::values::RuntimeValue;

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: RuntimeValue,
}

impl ReturnValue {
    pub fn new(value: RuntimeValue) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct BreakValue;

#[derive(Debug, Clone)]
pub struct ContinueValue;

#[derive(Debug, Clone)]
pub struct ThrownValue {
    pub value: RuntimeValue,
}

impl ThrownValue {
    pub fn new(value: RuntimeValue) -> Self {
        Self { value }
    }
}
