pub mod builtins; // Modular built-in functions and types
pub mod call_stack;
pub mod control_flow;
pub mod conversion;
pub mod decorator_registry;
pub mod dynamic;
pub mod environment;
pub mod module_registry;
pub mod module_system;
pub mod native;
pub mod natives;
pub mod plugin_system;
pub mod registrar;
pub mod rust_natives;
pub mod stdlib; // Standard library system (loader, wrappers, natives)
pub mod type_object;
pub mod type_object_builder;
pub mod types;
pub mod values;

pub use builtins::setup_builtins;
pub use call_stack::{CallStack, StackFrame};
pub use control_flow::{BreakValue, ContinueValue, ReturnValue, ThrownValue};
pub use conversion::{FromRaccoon, ToRaccoon};
pub use decorator_registry::{DecoratorRegistry, DecoratorTarget, DecoratorVisibility};
pub use dynamic::{DynamicRuntimeValue, DynamicValue};
pub use environment::Environment;
pub use module_registry::ModuleRegistry;
pub use module_system::{analyze_exports, resolve_module_path, Module, ModuleCache, ModuleSystem};
pub use native::{NativeDecoratorProcessor, NativeRegistry};
pub use plugin_system::{NativePlugin, PluginManager, PluginRegistry};
pub use registrar::Registrar;
pub use stdlib::{register_all_stdlib_natives, register_stdlib_wrappers, StdLibLoader};
pub use type_object::{PrimitiveKind, SourceLocation, TypeKind, TypeMetadata, TypeObject};
pub use type_object_builder::TypeObjectBuilder;
pub use types::registry::TypeRegistry;
pub use values::*;
