//! This file contains derive macro implementation used to convert an array to
//! its first element.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Add a TryFrom<Vec<T>> for the type T. The idea is to get only the first
/// element of a vector.
///
/// # Arguments:
/// * `input` - Input token stream.
///
/// # Returns:
/// Generated token stream to be added for compilation.
pub fn impl_try_from_vec(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl std::convert::TryFrom<Vec<#name>> for #name {
            type Error = Error;

            fn try_from(values: Vec<#name>) -> Result<Self, Self::Error> {
                values.first().ok_or(Error::NotFound).cloned()
            }
        }
    };

    TokenStream::from(expanded)
}
