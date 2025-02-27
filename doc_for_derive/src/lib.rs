#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Ident, Lit, LitByteStr, LitStr, Meta};

/// Get the documentation comment from the attributes.
fn get_doc(attrs: Vec<syn::Attribute>) -> Option<String> {
    let doc_lines: Vec<String> = attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            let Meta::NameValue(nv) = attr.meta else {
                return None;
            };
            let Expr::Lit(expr_lit) = nv.value else {
                return None;
            };
            let Lit::Str(lit_str) = expr_lit.lit else {
                return None;
            };
            Some(lit_str.value())
        })
        .collect();
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}

/// Takes an iterator of (ident, attributes) pairs and generates a match expression that matches documentation. Used to generate the match arms for the `doc_for_field` method.
fn generate_arms<I>(iter: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = (Ident, Vec<syn::Attribute>)>,
{
    let arms = iter.map(|(ident, attrs)| {
        let name = ident.to_string();
        // Convert the name to a byte string literal (Rust doesn't allow matching on string literals in const functions).
        let name = LitByteStr::new(name.as_bytes(), Span::call_site());
        let doc = get_doc(attrs);
        match doc {
            Some(doc) => {
                let lit_doc = LitStr::new(&doc, Span::call_site());
                quote! { #name => ::core::option::Option::Some(#lit_doc), }
            }
            None => quote! { #name => ::core::option::Option::None, },
        }
    });
    quote! {
        let name_bytes = name.as_bytes();
        match name_bytes {
            #(#arms)*
            _ => ::core::panic!("Field does not exist"),
        }
    }
}

/// Derive `DocFor`.
///
/// Derives the `DocFor` trait for a type, with additional `const fn doc_for_field` method. Primarily intended for use with the `doc_for!` macro, but you can also use the derived constant and method directly via `MyType::DOC` and `MyType::doc_for_field("field")`.
#[proc_macro_derive(DocFor)]
pub fn doc_for_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Get the documentation comment for the type.
    let doc_for_type = get_doc(input.attrs);
    let doc_for_type_ret = match doc_for_type {
        Some(doc) => {
            let lit_doc = LitStr::new(&doc, Span::call_site());
            quote! { ::core::option::Option::Some(#lit_doc) }
        }
        None => quote! { ::core::option::Option::None },
    };
    let doc_for_type_impl = quote! {
        impl ::doc_for::DocFor for #name {
            const DOC: ::core::option::Option<&'static str> = #doc_for_type_ret;
        }
    };

    // Get the documentation comments for the fields.
    let doc_for_field_body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => generate_arms(
                fields
                    .named
                    .into_iter()
                    .map(|f| (f.ident.unwrap(), f.attrs)),
            ),
            _ => quote! { ::core::option::Option::None },
        },
        Data::Union(data) => generate_arms(
            data.fields
                .named
                .into_iter()
                .map(|f| (f.ident.unwrap(), f.attrs)),
        ),
        Data::Enum(data) => generate_arms(data.variants.into_iter().map(|v| (v.ident, v.attrs))),
    };
    let doc_for_field_impl = quote! {
        impl #name {
            const fn doc_for_field(name: &'static str) -> ::core::option::Option<&'static str> {
                #doc_for_field_body
            }
        }
    };

    let expanded = quote! {
        #doc_for_type_impl
        #doc_for_field_impl
    };
    expanded.into()
}
