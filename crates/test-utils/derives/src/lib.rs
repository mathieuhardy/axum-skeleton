//! This file contains derive macros used in test-utils crate.

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{parse, parse_quote, Block, ItemFn, Stmt};

/// Error message to be displayed if something goes wrong during macro processing.
const ERROR_MSG: &str = "Attribute should be like: #[hook(setup, teardown)]";

/// Attribute macro used to generate automatically a unit test giving some hooks for setup and
/// teardown methods.
///
/// The attribute must be placed before other attribute macros like `#[tokio::test]`.
///
/// The format of the attribute is `#[hook(setup, teardown)]` where setup and teardown are the
/// names of functions to be called. If any of them is not needed, you can replace them by
/// underscores.
///
/// # Arguments:
/// * `attr` - Attribute arguments (setup and teardown callbacks).
/// * `item` - Tokens of the function this macro is attached to.
///
/// # Returns:
/// A new token stream that will fully replace the actual.
#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = attr.into_iter().collect::<Vec<TokenTree>>();

    // Expect a setup, a comma and a teardown
    if tokens.len() != 3 {
        panic!("{}", ERROR_MSG);
    }

    let (setup, teardown) = match (&tokens[0], &tokens[1], &tokens[2]) {
        (TokenTree::Ident(setup), TokenTree::Punct(_), TokenTree::Ident(teardown)) => {
            (setup.to_string(), teardown.to_string())
        }
        _ => panic!("{}", ERROR_MSG),
    };

    let setup = match setup.as_str() {
        "_" => syn::Ident::new("no_setup", Span::call_site()),
        _ => syn::Ident::new(&setup, Span::call_site()),
    };

    let teardown = match teardown.as_str() {
        "_" => syn::Ident::new("no_teardown", Span::call_site()),
        _ => syn::Ident::new(&teardown, Span::call_site()),
    };

    // Get original function AST
    let mut orig: ItemFn = parse(item).expect("Attribute must be attached to a function");
    let stmts = orig.block.clone();

    // Replace block of the function
    let runner: Stmt = parse_quote! {
        test_utils::run_test(#setup, #stmts, #teardown).await;
    };

    orig.block = Box::new(Block {
        brace_token: orig.block.brace_token,
        stmts: vec![runner],
    });

    TokenStream::from(orig.to_token_stream())
}
