#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, Fields, Ident, Lit, LitByteStr, LitInt,
    LitStr, Meta,
};

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

/// Generate the return value for a match arm, given the attributes of a field or variant. Used in the `generate_arms` and `generate_arms_index` functions.
fn generate_arm_value(attrs: Vec<Attribute>) -> proc_macro2::TokenStream {
    let doc = get_doc(attrs);
    match doc {
        Some(doc) => {
            let lit_doc = LitStr::new(&doc, Span::call_site());
            quote! { ::core::option::Option::Some(#lit_doc) }
        }
        None => quote! { ::core::option::Option::None },
    }
}

/// Takes an iterator of (ident, attributes) pairs and generates a match expression that matches on field names. Used to generate the match arms for the `doc_for_field` method.
fn generate_arms<I>(iter: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = (Ident, Vec<syn::Attribute>)>,
{
    let arms = iter.map(|(ident, attrs)| {
        let field_or_variant = ident.to_string();
        // Convert the name to a byte string literal (Rust doesn't allow matching on string literals in const functions).
        let field_or_variant = LitByteStr::new(field_or_variant.as_bytes(), Span::call_site());
        let arm_value = generate_arm_value(attrs);
        quote! { #field_or_variant => #arm_value, }
    });
    quote! {
        let name_bytes = field_or_variant.as_bytes();
        match name_bytes {
            #(#arms)*
            _ => ::core::panic!("The field or variant does not exist"),
        }
    }
}

/// Takes an iterator of attributes and generates a match expression that matches on field indices. Used to generate the match arms for the `doc_for_field` method.
fn generate_arms_index<I>(iter: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = Vec<syn::Attribute>>,
{
    let arms = iter.enumerate().map(|(field_index, attrs)| {
        let field_index = LitInt::new(&field_index.to_string(), Span::call_site());
        let arm_value = generate_arm_value(attrs);
        quote! { #field_index => #arm_value, }
    });
    quote! {
        match field_index {
            #(#arms)*
            _ => ::core::panic!("The field or variant does not exist"),
        }
    }
}

/// Takes an iterator of (ident, attributes) pairs and generates a match expression that matches on varients. Used to generate the match arms for the `doc_dyn` method.
fn generate_arms_enum<I>(iter: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = (Ident, Vec<syn::Attribute>)>,
{
    let arms = iter.map(|(ident, attrs)| {
        let arm_value = generate_arm_value(attrs);
        quote! { Self::#ident => #arm_value, }
    });
    quote! {
        match self {
            #(#arms)*
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
    let mut numeric = false;
    let doc_for_field_body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => generate_arms(
                fields
                    .named
                    .into_iter()
                    .map(|f| (f.ident.unwrap(), f.attrs)),
            ),
            Fields::Unnamed(fields) => {
                numeric = true;
                generate_arms_index(fields.unnamed.into_iter().map(|f| f.attrs))
            }
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
    let doc_for_field_input = if numeric {
        quote! { field_index: usize }
    } else {
        quote! { field_or_variant: &'static str }
    };
    let doc_for_field_impl = quote! {
        impl #name {
            const fn doc_for_field(#doc_for_field_input) -> ::core::option::Option<&'static str> {
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

/// Derive `DocDyn`.
///
/// Derives the `DocDyn` trait for an enum type, which provides a `doc_dyn` method that returns the documentation comment for the variant.
#[proc_macro_derive(DocDyn)]
pub fn doc_dyn_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let doc_for_variant_body = match input.data {
        Data::Enum(data) => {
            generate_arms_enum(data.variants.into_iter().map(|v| (v.ident, v.attrs)))
        }
        _ => ::core::panic!("DocDyn can only be derived for enums"),
    };
    let doc_for_variant_impl = quote! {
        impl ::doc_for::DocDyn for #name {
            fn doc_dyn(&self) -> ::core::option::Option<&'static str> {
                #doc_for_variant_body
            }
        }
    };

    let expanded = quote! {
        #doc_for_variant_impl
    };
    expanded.into()
}
