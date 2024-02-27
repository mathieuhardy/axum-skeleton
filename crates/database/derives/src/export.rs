//! This file contains the functions used to generate automatically some structures according to
//! base structure. It's possible to choose which fields to take and specify the list of derive
//! macros to apply to each generated structure.

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Ident, Span, TokenTree};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::{Gt, Lt};
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, AttrStyle, Attribute, Data, DeriveInput,
    Field, GenericArgument, MacroDelimiter, Meta, Path, PathArguments, PathSegment, Type, TypePath,
};

/// Entry point of the derice macro.
///
/// # Arguments
/// * `input` - Token stream of the structure.
///
/// # Returns
/// A new token stream added to the AST.
pub fn impl_export(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        Data::Struct(_) => (),
        _ => panic!("Derive macro can be applied to struct only"),
    }

    let outputs = find_attribute_idents_for_input(&ast, "export")
        .into_iter()
        .filter(|e| *e != "derives")
        .collect::<Vec<_>>();

    generate_exports(&ast, &outputs)
}

/// Finds the list of identifiers for a DeriveInput.
///
/// # Arguments
/// * `input` - The input to search into.
/// * `name` - The name of the macro to find.
///
/// # Returns
/// The list of identifiers found in for the macro.
fn find_attribute_idents_for_input(input: &DeriveInput, name: &str) -> Vec<Ident> {
    find_attribute_idents(&input.attrs, name)
}

/// Finds the list of identifiers inside a list of attributes.
///
/// # Arguments
/// * `attrs` - The list of attributes.
/// * `name` - The name of the macro to find.
///
/// # Returns
/// The list of identifiers found in for the macro.
fn find_attribute_idents(attrs: &[Attribute], name: &str) -> Vec<Ident> {
    let mut idents = Vec::new();

    for attr in attrs {
        if !attribute_is(attr, name) {
            continue;
        }

        let list = match &attr.meta {
            Meta::List(list) => list,
            _ => continue,
        };

        for token in list.tokens.clone() {
            if let TokenTree::Ident(ident) = token {
                idents.push(ident);
            }
        }
    }

    idents
}

/// Checks if an attribute is of a given type.
///
/// # Arguments
/// * `attr` - Attribute to check.
/// * `name` - `is_in`, `optional_in`, etc.
///
/// # Returns
/// `true` if it matches, `false` otherwise.
fn attribute_is(attr: &Attribute, name: &str) -> bool {
    if attr.style != AttrStyle::Outer {
        return false;
    }

    let list = match &attr.meta {
        Meta::List(list) => list,
        _ => return false,
    };

    match list.delimiter {
        MacroDelimiter::Paren(_) => (),
        _ => return false,
    }

    for segment in &list.path.segments {
        if segment.ident == name {
            return true;
        }
    }

    false
}

/// Checks if a field has an attribute attached given the attribute name and an identifier it
/// applies to.
///
/// # Arguments
/// * `field` - A field of the structure.
/// * `scope` - `is_in`, `optional_in`, etc.
/// * `name` - Target structure name.
///
/// # Returns
/// `true` if found, `false` otherwise.
fn has_attribute(field: &Field, scope: &str, name: &Ident) -> bool {
    let idents = find_attribute_idents_for_field(field, scope);

    for ident in idents {
        if &ident == name {
            return true;
        }
    }

    false
}

/// Finds the list of identifiers for a Field.
///
/// # Arguments
/// * `field` - The field to search into.
/// * `name` - The name of the macro to find.
///
/// # Returns
/// The list of identifiers found in for the macro.
fn find_attribute_idents_for_field(field: &Field, name: &str) -> Vec<Ident> {
    find_attribute_idents(&field.attrs, name)
}

/// Generate all the structures from the model.
///
/// # Arguments
/// * `input` - The input of the model structure.
/// * `idents` - List of identifiers of the structures to generate.
///
/// # Returns
/// A new token stream to insert in the AST.
fn generate_exports(input: &DeriveInput, idents: &[Ident]) -> TokenStream {
    let mut stream = TokenStream::new();

    for ident in idents {
        stream.extend::<TokenStream>(generate_export(input, ident));
    }

    stream
}

/// Generate one structure from the model.
///
/// # Arguments
/// * `input` - The input of the model structure.
/// * `ident` - Identifier of the structure to generate.
///
/// # Returns
/// A new token stream to insert in the AST.
fn generate_export(input: &DeriveInput, ident: &Ident) -> TokenStream {
    let input_name = &input.ident;
    let struct_name = format_ident!("{}{}", input_name, ident);

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => return TokenStream::new(),
    };

    let filtered_fields = fields.iter().filter_map(|f| {
        if has_attribute(f, "is_in", ident) {
            let mut sanitized = f.clone();

            sanitized.attrs.retain(|a| !attribute_is(a, "is_in"));

            Some(sanitized)
        } else if has_attribute(f, "optional_in", ident) {
            let mut sanitized = f.clone();

            sanitized.ty = to_optional(&f.ty);
            sanitized.attrs.retain(|a| !attribute_is(a, "optional_in"));

            Some(sanitized)
        } else {
            None
        }
    });

    let derives = find_derives(&input.attrs, ident);

    //let fields = find_fields(&input.attrs, ident);

    TokenStream::from(quote! {
        /// Auto-generated structure.
        #[derive(#(#derives),*)]
        pub struct #struct_name {
            #(#filtered_fields),*
        }
    })
}

/// Converts a type to an optional of this type.
///
/// # Arguments
/// * `ty` - Input type to convert.
///
/// # Returns
/// Option<T>.
fn to_optional(ty: &Type) -> Type {
    let option = Ident::new("Option", Span::call_site());

    let mut option_segments = Punctuated::new();
    option_segments.push(GenericArgument::Type(ty.clone()));

    let mut segments = Punctuated::new();
    segments.push(PathSegment {
        ident: option,
        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Lt {
                spans: [Span::call_site()],
            },
            args: option_segments,
            gt_token: Gt {
                spans: [Span::call_site()],
            },
        }),
    });

    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    })
}

/// Finds the list of identifiers of the derives to apply to a generated structure.
///
/// # Arguments
/// * `attrs` - List to attributes to traverse.
/// * `target` - Target identifier to match (name of the generated structure).
///
/// # Returns
/// List of identifiers found for this structure.
fn find_derives(attrs: &[Attribute], target: &Ident) -> Vec<Ident> {
    for attr in attrs {
        if !attribute_is(attr, "export") {
            continue;
        }

        let idents = find_derive_idents(attr, target);

        if !idents.is_empty() {
            return idents;
        }
    }

    Vec::new()
}

/// Finds the list of identifiers of the derives to apply to a generated structure for a given
/// attribute.
///
/// # Arguments
/// * `attr` - Attribute to traverse.
/// * `target` - Target identifier to match (name of the generated structure).
///
/// # Returns
/// List of identifiers found for this structure.
fn find_derive_idents(attr: &Attribute, target: &Ident) -> Vec<Ident> {
    if attr.style != AttrStyle::Outer {
        return Vec::new();
    }

    let list = match &attr.meta {
        Meta::List(list) => list,
        _ => return Vec::new(),
    };

    match list.delimiter {
        MacroDelimiter::Paren(_) => (),
        _ => return Vec::new(),
    }

    // Check if it's a `derives` entry
    let token = list.tokens.clone().into_iter().find(|token| match token {
        TokenTree::Ident(ident) => *ident == "derives",
        _ => false,
    });

    if token.is_none() {
        return Vec::new();
    }

    // Find the group inside the `derives` entry
    let group = list
        .tokens
        .clone()
        .into_iter()
        .find_map(|token| match token {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Parenthesis {
                    Some(group)
                } else {
                    None
                }
            }

            _ => None,
        });

    let group = match group {
        Some(group) => group,
        None => return Vec::new(),
    };

    // Get name of the structure
    let ident = group.stream().into_iter().find_map(|e| {
        if let TokenTree::Ident(ident) = e {
            Some(ident)
        } else {
            None
        }
    });

    let ident = match ident {
        Some(ident) => ident,
        None => return Vec::new(),
    };

    if ident != *target {
        return Vec::new();
    }

    // Find the group after the name of the structure
    let group = group.stream().into_iter().find_map(|token| match token {
        TokenTree::Group(group) => {
            if group.delimiter() == Delimiter::Parenthesis {
                Some(group)
            } else {
                None
            }
        }

        _ => None,
    });

    if group.is_none() {
        return Vec::new();
    }

    // Get the list of derives
    let idents = group
        .unwrap()
        .stream()
        .into_iter()
        .filter_map(|token| {
            if let TokenTree::Ident(ident) = token {
                Some(ident)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if idents.is_empty() {
        return Vec::new();
    }

    idents
}

/*
fn find_fields(attrs: &[Attribute], target: &Ident) -> TokenStream2 {
    for attr in attrs {
        if !attribute_is(attr, "export") {
            continue;
        }

        if let Some(stream) = find_field_info(attr, target) {
            return stream;
        }
    }

    TokenStream2::new()
}

fn find_field_info(attr: &Attribute, target: &Ident) -> Option<TokenStream2> {
    if attr.style != AttrStyle::Outer {
        return None;
    }

    let list = match &attr.meta {
        Meta::List(list) => list,
        _ => return None,
    };

    match list.delimiter {
        MacroDelimiter::Paren(_) => (),
        _ => return None,
    }

    // Check if it's a `field` entry
    let token = list.tokens.clone().into_iter().find(|token| match token {
        TokenTree::Ident(ident) => ident.to_string() == "field",
        _ => false,
    });

    if token.is_none() {
        return None;
    }

    // Find the group inside the `field` entry
    let group = list
        .tokens
        .clone()
        .into_iter()
        .find_map(|token| match token {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Parenthesis {
                    Some(group)
                } else {
                    None
                }
            }

            _ => None,
        });

    let group = match group {
        Some(group) => group,
        None => return None,
    };

    // Get name of the structure
    let ident = group.stream().into_iter().find_map(|e| {
        if let TokenTree::Ident(ident) = e {
            Some(ident)
        } else {
            None
        }
    });

    let ident = match ident {
        Some(ident) => ident,
        None => return None,
    };

    if ident.to_string() != target.to_string() {
        return None;
    }

    // Find the group after the name of the structure
    let group = group.stream().into_iter().find_map(|token| match token {
        TokenTree::Group(group) => {
            if group.delimiter() == Delimiter::Parenthesis {
                Some(group)
            } else {
                None
            }
        }

        _ => None,
    });

    let group = match group {
        Some(group) => group,
        None => return None,
    };

    Some(group.stream())
}
*/
