//! This file contains derive macros used in database crate.

#![forbid(unsafe_code)]

mod export;
mod sqlx_pg_insertable;
mod try_from_vec;

use proc_macro::TokenStream;

/// Adds a `TryFrom<Vec<T>>` for the type T. The idea is to get only the first
/// element of a vector.
///
/// # Examples
///
/// ```rust
/// use database_derives::TryFromVec;
///
/// enum Error {
///   NotFound,
/// }
///
/// #[derive(Clone, TryFromVec)]
/// struct Foo {
///   pub field: Option<bool>,
/// }
/// ```
#[proc_macro_derive(TryFromVec)]
pub fn derive_try_from_vec(input: TokenStream) -> TokenStream {
    try_from_vec::impl_try_from_vec(input)
}

/// Creates another structure taking this one as model.
///
/// # Examples
///
/// ```rust
/// use database_derives::*;
///
/// // A struct FooBar will be created. By default no fields from Foo will be added to FooBar.
/// // Fields with attribute `is_in` will be added as is.
/// // Fields with attribute `optional_in` will be added as an optional value.
/// #[derive(Export)]
/// #[export(Bar)]
/// #[export(derives(Bar(Debug)))]
/// struct Foo {
///   pub field: bool,
///   #[is_in(Bar)]
///   pub field_1: bool,
///   #[optional_in(Bar)]
///   pub field_2: bool,
/// }
/// ```
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

/// Implements the methods of the trait SqlxPgInsertable for a struct.
///
/// # Examples
///
/// ```rust
/// use database_derives::SqlxPgInsertable;
/// use database::traits::sqlx::postgres::crud::SqlxPgInsertable;
///
/// #[derive(SqlxPgInsertable)]
/// struct Foo {
///   pub field: Option<bool>,
/// }
/// ```
#[proc_macro_derive(SqlxPgInsertable)]
pub fn derive_insertable(input: TokenStream) -> TokenStream {
    sqlx_pg_insertable::impl_sqlx_pg_insertable(input)
}
