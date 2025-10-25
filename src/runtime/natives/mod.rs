/// Native function modules
///
/// This module organizes native functions by category to keep the codebase modular
/// and prevent native_bridge.rs from growing indefinitely.
///
/// Each submodule provides functions to register its category of natives.

pub mod io;
pub mod output;
pub mod time;
pub mod random;
pub mod json;
pub mod string;
pub mod array;
pub mod math;
pub mod http;
pub mod ffi;

use crate::runtime::values::NativeFunctionValue;
use std::collections::HashMap;

/// Helper struct to accumulate all native functions
pub struct NativeRegistry {
    pub functions: HashMap<String, NativeFunctionValue>,
    pub async_functions: HashMap<String, crate::runtime::values::NativeAsyncFunctionValue>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            async_functions: HashMap::new(),
        }
    }

    /// Register all native functions from all modules
    pub fn register_all(&mut self) {
        output::register(&mut self.functions);
        time::register(&mut self.functions);
        random::register(&mut self.functions);
        json::register(&mut self.functions);
        string::register(&mut self.functions);
        array::register(&mut self.functions);
        math::register(&mut self.functions);
        ffi::register(&mut self.functions);
        http::register_async(&mut self.async_functions);

        // Create backward compatibility aliases for old naming convention
        self.create_aliases();
    }

    /// Create aliases for old naming convention (_*_native) to new convention (native_*)
    fn create_aliases(&mut self) {
        // Array aliases
        if let Some(func) = self.functions.get("native_array_length").cloned() {
            self.functions.insert("_length_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_push").cloned() {
            self.functions.insert("_push_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_pop").cloned() {
            self.functions.insert("_pop_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_shift").cloned() {
            self.functions.insert("_shift_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_unshift").cloned() {
            self.functions.insert("_unshift_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_slice").cloned() {
            self.functions.insert("_slice_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_reverse").cloned() {
            self.functions.insert("_reverse_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_array_sort").cloned() {
            self.functions.insert("_sort_native".to_string(), func);
        }

        // String aliases
        if let Some(func) = self.functions.get("native_str_length").cloned() {
            self.functions.insert("_length_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_upper").cloned() {
            self.functions.insert("_upper_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_lower").cloned() {
            self.functions.insert("_lower_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_trim").cloned() {
            self.functions.insert("_trim_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_substring").cloned() {
            self.functions.insert("_substring_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_char_at").cloned() {
            self.functions.insert("_char_at_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_index_of").cloned() {
            self.functions.insert("_index_of_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_replace").cloned() {
            self.functions.insert("_replace_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_split").cloned() {
            self.functions.insert("_split_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_starts_with").cloned() {
            self.functions.insert("_starts_with_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_str_ends_with").cloned() {
            self.functions.insert("_ends_with_native".to_string(), func);
        }

        // JSON aliases
        if let Some(func) = self.functions.get("native_json_parse").cloned() {
            self.functions.insert("_parse_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_json_stringify").cloned() {
            self.functions.insert("_stringify_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_json_stringify_pretty").cloned() {
            self.functions.insert("_stringify_pretty_native".to_string(), func);
        }

        // Time aliases
        if let Some(func) = self.functions.get("native_time_now").cloned() {
            self.functions.insert("_now_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_time_now_secs").cloned() {
            self.functions.insert("_now_secs_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_time_sleep").cloned() {
            self.functions.insert("_sleep_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_time_format").cloned() {
            self.functions.insert("_format_native".to_string(), func);
        }

        // Random aliases
        if let Some(func) = self.functions.get("native_random").cloned() {
            self.functions.insert("_random_native".to_string(), func);
        }

        // IO aliases
        if let Some(func) = self.functions.get("native_io_read_file").cloned() {
            self.functions.insert("_read_file_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_write_file").cloned() {
            self.functions.insert("_write_file_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_append_file").cloned() {
            self.functions.insert("_append_file_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_file_exists").cloned() {
            self.functions.insert("_file_exists_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_delete_file").cloned() {
            self.functions.insert("_delete_file_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_read_dir").cloned() {
            self.functions.insert("_read_dir_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_create_dir").cloned() {
            self.functions.insert("_create_dir_native".to_string(), func);
        }
        if let Some(func) = self.functions.get("native_io_input").cloned() {
            self.functions.insert("_input_native".to_string(), func);
        }
    }
}
