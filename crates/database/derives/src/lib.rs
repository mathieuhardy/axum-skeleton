//! This file contains derive macros used in database crate.

use quote::quote;

use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

/// Add a TryFrom<Vec<T>> for the type T. The idea is to get only the first
/// element of a vector.
///
/// # Arguments:
/// * `input` - Input token stream.
///
/// # Returns:
/// Generated token stream to be added for compilation.
#[proc_macro_derive(TryFromVec)]
pub fn macro_try_from_vec(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl std::convert::TryFrom<Vec<#name>> for #name {
            type Error = crate::error::Error;

            fn try_from(values: Vec<#name>) -> Result<Self, Self::Error> {
                values.first().ok_or(Error::NotFound).cloned()
            }
        }
    };

    TokenStream::from(expanded)
}
