# Plugin System Migration Guide

## Overview

The new **Plugin-Based Native Function System** replaces the legacy scattered native function registration with a unified, scalable architecture.

## Architecture Changes

### OLD (Legacy) System
```
src/runtime/natives/*.rs
  ├── Each file has a register() function
  ├── Functions spread across 10+ files
  └── Manual alias creation (40+ lines in mod.rs)

src/runtime/ffi.rs
  ├── Match statement explosion (100+ lines)
  └── Hardcoded signature support

src/runtime/native_bridge.rs
  ├── Simple HashMap wrapper
  └── No plugin concept
```

### NEW (Plugin) System
```
src/runtime/plugin_system.rs
  ├── NativePlugin trait
  ├── PluginRegistry (unified registration)
  └── PluginManager (centralized management)

src/runtime/builtin_plugins.rs
  ├── Plugin implementations
  └── load_builtin_plugins() helper

src/runtime/native_bridge_v2.rs
  ├── New unified interface
  └── Backward compatible API
```

## Key Benefits

| Feature | Legacy | Plugin |
|---------|--------|--------|
| Boilerplate | 40+ alias lines | Zero |
| New function | 10+ lines | 2-3 lines |
| Type erasure | Manual | Automatic |
| Scale | O(n²) complexity | O(n) linear |
| Code locations | 10+ scattered files | Centralized |

## Migration Path

### Phase 1: Parallel Systems (Current)
- ✅ Plugin system is fully functional
- ✅ NativeBridgeV2 handles new plugins
- ❌ Legacy system still in use
- ⏳ Old code can coexist

### Phase 2: Gradual Migration
1. Migrate one native module at a time to plugin interface
2. Update builtin_plugins.rs to use new plugins instead of old register()
3. Remove aliases from natives/mod.rs

### Phase 3: Complete Cleanup
- Delete legacy ffi.rs (match explosion)
- Delete legacy ffi_registry.rs (if FFI decoupled)
- Delete legacy native_bridge.rs
- Clean up natives/mod.rs aliases

## How to Add New Functions

### Legacy Way (DEPRECATED)
```rust
// src/runtime/natives/mymodule.rs
pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    let my_fn = NativeFunctionValue::new(|args| { ... });
    functions.insert("native_my_fn".to_string(), my_fn);
}

// Then manually add alias in natives/mod.rs
if let Some(func) = self.functions.get("native_my_fn").cloned() {
    self.functions.insert("native_my_alias".to_string(), func);
}
```

### New Way (RECOMMENDED)
```rust
// Already in builtin_plugins.rs or new file
impl NativePlugin for MyPlugin {
    fn namespace(&self) -> &str {
        "mymodule"
    }

    fn register(&self, registry: &mut PluginRegistry) {
        let my_fn = NativeFunctionValue::new(|args| { ... });
        registry.register_sync("my_fn", Some("mymodule"), fn_type, my_fn);
        // NO aliases needed!
    }
}

// In load_builtin_plugins():
let my_plugin = MyPlugin;
my_plugin.register(registry);
```

## Files Involved

### New Files
- `src/runtime/plugin_system.rs` - Core traits & registry
- `src/runtime/builtin_plugins.rs` - Plugin implementations
- `src/runtime/native_bridge_v2.rs` - New unified interface
- `PLUGIN_SYSTEM_MIGRATION.md` - This file

### Legacy Files (To Be Cleaned Up)
- `src/runtime/ffi.rs` - FFI with match explosion
- `src/runtime/ffi_registry.rs` - FFI registry
- `src/runtime/native_bridge.rs` - Old interface
- `src/runtime/natives/mod.rs` - 40+ lines of aliases

### Updated Files
- `src/runtime/mod.rs` - Exports plugin types
- `src/interpreter/mod.rs` - Still uses FFIRegistry (needs gradual update)

## Testing the New System

```bash
# The plugin system compiles and loads:
cargo build

# Test that native functions work:
cargo test native
```

## Future Improvements

1. **Dynamic Plugin Loading**: Load .so/.dll plugins at runtime
2. **Plugin Discovery**: Automatic plugin scanning (reflection)
3. **FFI Refactor**: Eliminate match explosion with generic function pointers
4. **Type System Integration**: Plugin signature validation
