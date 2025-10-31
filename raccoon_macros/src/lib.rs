extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, FnArg, ItemFn, Pat, ReturnType, Type,
    spanned::Spanned,
};

/// Procedural macro to automatically register native functions
///
/// Transforms a simple Rust function into a native function wrapper that:
/// 1. Extracts typed parameters from RuntimeValue arguments
/// 2. Calls the original function
/// 3. Converts the result back to RuntimeValue
///
/// Usage:
/// ```ignore
/// #[native_fn]
/// pub fn str_length(s: &str) -> i64 {
///     s.len() as i64
/// }
/// ```
///
/// This generates:
/// - The original function (unchanged)
/// - A __native_wrapper module with:
///   - NAME: the native function name
///   - get_function_type(): returns the FunctionType
///   - invoke(args): wraps the call and conversions
#[proc_macro_attribute]
pub fn native_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let native_fn_name = format!("native_{}", fn_name_str);
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_sig = &input_fn.sig;

    let inputs = &fn_sig.inputs;
    let output = &fn_sig.output;

    // Build parameter extraction and types
    let mut param_extractors = Vec::new();
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();

    for (i, input) in inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = input {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let param_name = &pat_ident.ident;
                param_names.push(param_name);

                let ty = &pat_type.ty;
                let (ptype, extractor) = build_param_extractor(i, ty, &param_name.to_string());
                param_types.push(ptype);
                param_extractors.push(extractor);
            }
        }
    }

    // Parse return type
    let return_type = match output {
        ReturnType::Default => quote! { PrimitiveType::void() },
        ReturnType::Type(_, ty) => build_return_type(ty),
    };

    // Generate conversion code for result
    let result_conversion = build_result_conversion(output);

    let param_types_vec = quote! { vec![#(#param_types),*] };

    // Generate wrapper code
    let expanded = quote! {
        // Keep the original function
        #fn_vis fn #fn_name #inputs #output #fn_block

        // Generate wrapper with metadata and invocation
        pub mod __native_wrapper {
            use super::*;
            use crate::runtime::values::*;
            use crate::ast::types::{FunctionType, PrimitiveType, Type};

            pub const NAME: &str = #native_fn_name;

            pub fn get_function_type() -> Type {
                Type::Function(Box::new(FunctionType {
                    params: #param_types_vec,
                    return_type: #return_type,
                    is_variadic: false,
                }))
            }

            pub fn invoke(args: Vec<RuntimeValue>) -> RuntimeValue {
                #(#param_extractors)*
                let result = super::#fn_name(#(#param_names),*);
                #result_conversion
            }
        }
    };

    TokenStream::from(expanded)
}

/// Build parameter extraction code and its type
fn build_param_extractor(
    index: usize,
    ty: &Type,
    name: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ty_str = quote!(#ty).to_string();
    let param_ident = syn::Ident::new(name, ty.span());

    if ty_str.contains("&str") {
        (
            quote! { PrimitiveType::str() },
            quote! {
                let #param_ident = match args.get(#index) {
                    Some(RuntimeValue::Str(s)) => s.value.as_str(),
                    _ => "",
                };
            },
        )
    } else if ty_str.contains("String") {
        (
            quote! { PrimitiveType::str() },
            quote! {
                let #param_ident = match args.get(#index) {
                    Some(RuntimeValue::Str(s)) => s.value.clone(),
                    _ => String::new(),
                };
            },
        )
    } else if ty_str.contains("i64") {
        (
            quote! { PrimitiveType::int() },
            quote! {
                let #param_ident = match args.get(#index) {
                    Some(RuntimeValue::Int(i)) => i.value,
                    Some(RuntimeValue::Float(f)) => f.value as i64,
                    _ => 0i64,
                };
            },
        )
    } else if ty_str.contains("f64") {
        (
            quote! { PrimitiveType::float() },
            quote! {
                let #param_ident = match args.get(#index) {
                    Some(RuntimeValue::Float(f)) => f.value,
                    Some(RuntimeValue::Int(i)) => i.value as f64,
                    _ => 0.0f64,
                };
            },
        )
    } else if ty_str.contains("bool") {
        (
            quote! { PrimitiveType::bool() },
            quote! {
                let #param_ident = match args.get(#index) {
                    Some(RuntimeValue::Bool(b)) => b.value,
                    _ => false,
                };
            },
        )
    } else {
        (
            quote! { PrimitiveType::any() },
            quote! {
                let #param_ident = args.get(#index).cloned().unwrap_or(RuntimeValue::Null(NullValue::new()));
            },
        )
    }
}

/// Build return type conversion
fn build_return_type(ty: &Type) -> proc_macro2::TokenStream {
    let ty_str = quote!(#ty).to_string();

    if ty_str.contains("str") || ty_str.contains("String") {
        quote! { PrimitiveType::str() }
    } else if ty_str.contains("i64") {
        quote! { PrimitiveType::int() }
    } else if ty_str.contains("f64") {
        quote! { PrimitiveType::float() }
    } else if ty_str.contains("bool") {
        quote! { PrimitiveType::bool() }
    } else {
        quote! { PrimitiveType::any() }
    }
}

/// Build result conversion code
fn build_result_conversion(output: &ReturnType) -> proc_macro2::TokenStream {
    match output {
        ReturnType::Default => quote! {
            RuntimeValue::Null(NullValue::new())
        },
        ReturnType::Type(_, ty) => {
            let ty_str = quote!(#ty).to_string();

            if ty_str.contains("str") || ty_str.contains("String") {
                quote! {
                    RuntimeValue::Str(StrValue::new(result))
                }
            } else if ty_str.contains("i64") {
                quote! {
                    RuntimeValue::Int(IntValue::new(result))
                }
            } else if ty_str.contains("f64") {
                quote! {
                    RuntimeValue::Float(FloatValue::new(result))
                }
            } else if ty_str.contains("bool") {
                quote! {
                    RuntimeValue::Bool(BoolValue::new(result))
                }
            } else {
                quote! {
                    RuntimeValue::Null(NullValue::new())
                }
            }
        }
    }
}
