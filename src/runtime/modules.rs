use super::std::{ArrayModule, ConsoleModule, MathModule, StringModule};
use super::values::RuntimeValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub exports: HashMap<String, RuntimeValue>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exports: HashMap::new(),
        }
    }

    pub fn add_export(&mut self, name: String, value: RuntimeValue) {
        self.exports.insert(name, value);
    }

    pub fn get_export(&self, name: &str) -> Option<RuntimeValue> {
        self.exports.get(name).cloned()
    }

    pub fn get_all_exports(&self) -> HashMap<String, RuntimeValue> {
        self.exports.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    modules: HashMap<String, Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
        };
        registry.register_std_modules();
        registry
    }

    fn register_std_modules(&mut self) {
        self.register_std_math();
        self.register_std_string();
        self.register_std_array();
        self.register_std_console();
    }

    fn register_std_math(&mut self) {
        let mut module = Module::new(MathModule::name().to_string());
        let exports = MathModule::get_exports();
        for (name, value) in exports {
            module.add_export(name, value);
        }
        self.modules.insert(MathModule::name().to_string(), module);
    }

    fn register_std_string(&mut self) {
        let mut module = Module::new(StringModule::name().to_string());
        let exports = StringModule::get_exports();
        for (name, value) in exports {
            module.add_export(name, value);
        }
        self.modules
            .insert(StringModule::name().to_string(), module);
    }

    fn register_std_array(&mut self) {
        let mut module = Module::new(ArrayModule::name().to_string());
        let exports = ArrayModule::get_exports();
        for (name, value) in exports {
            module.add_export(name, value);
        }
        self.modules.insert(ArrayModule::name().to_string(), module);
    }

    fn register_std_console(&mut self) {
        let mut module = Module::new(ConsoleModule::name().to_string());
        let exports = ConsoleModule::get_exports();
        for (name, value) in exports {
            module.add_export(name, value);
        }
        self.modules
            .insert(ConsoleModule::name().to_string(), module);
    }

    pub fn get_module(&self, specifier: &str) -> Option<&Module> {
        self.modules.get(specifier)
    }

    pub fn register_module(&mut self, module: Module) {
        self.modules.insert(module.name.clone(), module);
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
