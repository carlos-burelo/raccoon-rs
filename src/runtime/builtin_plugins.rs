use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::plugin_system::{NativePlugin, PluginRegistry};
use crate::runtime::values::{NativeFunctionValue, NullValue, RuntimeValue};

fn native_print(args: Vec<RuntimeValue>) -> RuntimeValue {
    let output = args
        .iter()
        .map(|arg| {
            let plain = arg.to_string();

            plain
        })
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
    RuntimeValue::Null(NullValue::new())
}

fn native_eprint(args: Vec<RuntimeValue>) -> RuntimeValue {
    let output = args
        .iter()
        .map(|arg| {
            let plain = arg.to_string();
            if plain.contains('{') || plain.contains('[') || plain.starts_with('"') {
                plain
            } else {
                plain
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    eprintln!("{}", output);
    RuntimeValue::Null(NullValue::new())
}

pub struct OutputPlugin;

impl NativePlugin for OutputPlugin {
    fn namespace(&self) -> &str {
        "output"
    }

    fn register(&self, registry: &mut PluginRegistry) {
        let print_fn = NativeFunctionValue::new(
            native_print,
            Type::Function(Box::new(FunctionType {
                params: vec![],
                return_type: PrimitiveType::void(),
                is_variadic: true,
            })),
        );

        registry.register_sync("print", None::<String>, print_fn.fn_type.clone(), print_fn);

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

pub fn load_builtin_plugins(registry: &mut PluginRegistry) {
    let output = OutputPlugin;
    output.register(registry);

    let native_registry = crate::runtime::native::NativeRegistry::new();
    crate::runtime::rust_natives::register_all_native_functions(&native_registry);

    for (name, func) in native_registry.export_all() {
        registry.sync_functions.insert(name, func);
    }

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
