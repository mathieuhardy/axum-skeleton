//! This file contains derive macros used in database crate.

mod export;
mod try_from_vec;

use proc_macro::TokenStream;

/// See [impl_try_from_vec]
#[proc_macro_derive(TryFromVec)]
pub fn derive_try_from_vec(input: TokenStream) -> TokenStream {
    try_from_vec::impl_try_from_vec(input)
}

/// See [impl_export]
#[proc_macro_derive(Export, attributes(is_in, optional_in))]
pub fn derive_export(input: TokenStream) -> TokenStream {
    export::impl_export(input)
}

/// Macro used to define the list of Structure names to generate.
#[proc_macro_attribute]
pub fn export(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Macro used to define the list of derives to apply to a generated structure.
#[proc_macro_attribute]
pub fn export_derives(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}
