/// Built-in plugins for standard library functions
///
/// This module provides plugin implementations for all standard native functions,
/// consolidating the previous distributed registration system with the new
/// clean @native decorator architecture.

use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::plugin_system::{NativePlugin, PluginRegistry};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};

// ============================================================================
// OUTPUT FUNCTIONS - Core I/O
// ============================================================================

/// Native print function
fn native_print(args: Vec<RuntimeValue>) -> RuntimeValue {
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
}

/// Native eprint function
fn native_eprint(args: Vec<RuntimeValue>) -> RuntimeValue {
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
}

// ============================================================================
// OUTPUT PLUGIN - Core I/O Functions
// ============================================================================

pub struct OutputPlugin;

impl NativePlugin for OutputPlugin {
    fn namespace(&self) -> &str {
        "output"
    }

    fn register(&self, registry: &mut PluginRegistry) {
        // print function - variadic output
        let print_fn = NativeFunctionValue::new(
            native_print,
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        registry.register_sync("print", None::<String>, print_fn.fn_type.clone(), print_fn);

        // eprint function - variadic error output
        let eprint_fn = NativeFunctionValue::new(
            native_eprint,
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
// BUILTIN PLUGIN MANAGER - NEW CLEAN ARCHITECTURE
// ============================================================================

/// Initialize and load all built-in plugins with the new native system
pub fn load_builtin_plugins(registry: &mut PluginRegistry) {
    // ========================================================================
    // PHASE 1: Core I/O Functions (keep for stability)
    // ========================================================================
    let output = OutputPlugin;
    output.register(registry);

    // ========================================================================
    // PHASE 2: New Native Rust Functions (@native decorator system)
    // ========================================================================
    let native_registry = crate::runtime::native::NativeRegistry::new();
    crate::runtime::rust_natives::register_all_native_functions(&native_registry);

    // Export all registered native functions to plugin registry
    for (name, func) in native_registry.export_all() {
        registry.sync_functions.insert(name, func);
    }

    // ========================================================================
    // PHASE 3: Legacy Functions (to be migrated gradually)
    // ========================================================================
    // These will be deprecated as we migrate functions to the new @native system
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
