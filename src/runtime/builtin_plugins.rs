/// Built-in plugins for standard library functions
///
/// This module provides plugin implementations for all standard native functions,
/// consolidating the previous distributed registration system.
use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::plugin_system::{NativePlugin, PluginRegistry};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};

// ============================================================================
// OUTPUT PLUGIN
// ============================================================================

pub struct OutputPlugin;

impl NativePlugin for OutputPlugin {
    fn namespace(&self) -> &str {
        "output"
    }

    fn register(&self, registry: &mut PluginRegistry) {
        // print function
        let print_fn = NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| {
                        let plain = arg.to_string();
                        crate::output_style::format_value(&plain)
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        registry.register_sync("print", None::<String>, print_fn.fn_type.clone(), print_fn);

        // eprint function
        let eprint_fn = NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let output = args
                    .iter()
                    .map(|arg| {
                        let plain = arg.to_string();
                        if plain.contains('{') || plain.contains('[') || plain.starts_with('"') {
                            crate::output_style::format_value(&plain)
                        } else {
                            plain
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                eprintln!("{}", output);
                RuntimeValue::Null(NullValue::new())
            },
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        registry.register_sync(
            "eprint",
            None::<String>,
            eprint_fn.fn_type.clone(),
            eprint_fn,
        );
    }
}

// ============================================================================
// BUILTIN PLUGIN MANAGER
// ============================================================================

/// Initialize and load all built-in plugins
pub fn load_builtin_plugins(registry: &mut PluginRegistry) {
    // Output functions
    let output = OutputPlugin;
    output.register(registry);

    // Load all other native functions from existing natives module
    // (This will be phased out as we migrate to plugins)
    crate::runtime::natives::output::register(&mut registry.sync_functions);
    crate::runtime::natives::time::register(&mut registry.sync_functions);
    crate::runtime::natives::random::register(&mut registry.sync_functions);
    crate::runtime::natives::json::register(&mut registry.sync_functions);
    crate::runtime::natives::string::register(&mut registry.sync_functions);
    crate::runtime::natives::array::register(&mut registry.sync_functions);
    crate::runtime::natives::math::register(&mut registry.sync_functions);
    crate::runtime::natives::ffi::register(&mut registry.sync_functions);
    crate::runtime::natives::http::register_async(&mut registry.async_functions);
}
