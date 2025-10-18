use crate::runtime::values::RuntimeValue;
use std::collections::HashMap;

use super::{
    ArrayModule, ConsoleModule, IOModule, JSONModule, MathModule, RandomModule, StringModule,
    TimeModule,
};

pub trait StdModule {
    fn name() -> &'static str
    where
        Self: Sized;

    fn get_exports() -> HashMap<String, RuntimeValue>
    where
        Self: Sized;

    fn get_export(name: &str) -> Option<RuntimeValue>
    where
        Self: Sized;
}

pub struct ModuleRegistry;

impl ModuleRegistry {
    pub fn get_module_exports(module_name: &str) -> Option<HashMap<String, RuntimeValue>> {
        match module_name {
            "std:math" => Some(MathModule::get_exports()),
            "std:string" => Some(StringModule::get_exports()),
            "std:array" => Some(ArrayModule::get_exports()),
            "std:console" => Some(ConsoleModule::get_exports()),
            "std:time" => Some(TimeModule::get_exports()),
            "std:io" => Some(IOModule::get_exports()),
            "std:json" => Some(JSONModule::get_exports()),
            "std:random" => Some(RandomModule::get_exports()),
            _ => None,
        }
    }

    pub fn get_module_export(module_name: &str, export_name: &str) -> Option<RuntimeValue> {
        match module_name {
            "std:math" => MathModule::get_export(export_name),
            "std:string" => StringModule::get_export(export_name),
            "std:array" => ArrayModule::get_export(export_name),
            "std:console" => ConsoleModule::get_export(export_name),
            "std:time" => TimeModule::get_export(export_name),
            "std:io" => IOModule::get_export(export_name),
            "std:json" => JSONModule::get_export(export_name),
            "std:random" => RandomModule::get_export(export_name),
            _ => None,
        }
    }

    pub fn module_exists(module_name: &str) -> bool {
        matches!(
            module_name,
            "std:math"
                | "std:string"
                | "std:array"
                | "std:console"
                | "std:time"
                | "std:io"
                | "std:json"
                | "std:random"
        )
    }

    pub fn available_modules() -> Vec<&'static str> {
        vec![
            "std:math",
            "std:string",
            "std:array",
            "std:console",
            "std:time",
            "std:io",
            "std:json",
            "std:random",
        ]
    }
}
