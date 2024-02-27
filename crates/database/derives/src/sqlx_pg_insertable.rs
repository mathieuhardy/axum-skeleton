//! Implements SqlxPgInsertable trait for a struct.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Implements the methods of the trait SqlxPgInsertable for a struct.
///
/// # Arguments
/// * `input` - Input token of the struct.
///
/// # Returns
/// New tokens to be inserted in the AST.
pub fn impl_sqlx_pg_insertable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        Data::Struct(_) => (),
        _ => panic!("Derive macro can be applied to struct only"),
    }

    let struct_name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(data) => &data.fields,
        _ => return TokenStream::new(),
    };

    let idents: Vec<_> = fields.iter().map(|f| f.clone().ident).collect();

    TokenStream::from(quote! {
        impl SqlxPgInsertable for #struct_name {
            fn columns(&self) -> Vec<&'static str> {
                let mut cols = Vec::new();

                #(
                    if let Some(value) = &self.#idents {
                        cols.push(stringify!(#idents));
                    }
                )*

                cols
            }

            fn bind_insert_values<'a>(
                &'a self,
                mut query_builder: &mut sqlx::QueryBuilder<'a, sqlx::postgres::Postgres>,
            ) {
                let mut separated = query_builder.separated(", ");

                separated.push_unseparated("(");

                #(
                    if let Some(value) = &self.#idents {
                        separated.push_bind(value);
                    }
                )*

                separated.push_unseparated(")");
            }

            fn bind_update_values<'a>(
                &'a self,
                query_builder: &mut sqlx::QueryBuilder<'a, sqlx::postgres::Postgres>,
                prefix: Option<&str>,
            ) {
                let prefix = prefix
                    .map(|e| format!("{e}."))
                    .unwrap_or("".to_string());

                let mut separated = query_builder.separated(", ");

                #(
                    if let Some(value) = &self.#idents {
                        separated.push(format!("{} = {}", stringify!(#idents), prefix));
                        separated.push_bind_unseparated(value);
                    }
                )*
            }

            fn bind_unnest_values<'a>(
                query_builder: &mut sqlx::QueryBuilder<'a, sqlx::postgres::Postgres>,
                list: &[Self],
                as_suffix: bool,
            ) {
                let mut separated = query_builder.separated(", ");

                #(
                    if list[0].#idents.is_some() {
                        let values: Vec<_> = list.iter().filter_map(|data| {
                            if let Some(value) = &data.#idents {
                                Some(value.clone())
                            }
                            else {
                                None
                            }
                        }).collect();

                        separated.push("UNNEST(");
                        separated.push_bind_unseparated(values);
                        separated.push_unseparated(")");

                        if as_suffix {
                            separated.push_unseparated(format!(" AS {}", stringify!(#idents)));
                        }
                    }
                )*
            }
        }
    })
}
