#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, LitStr, Meta};

/// Get the documentation comment from the attributes.
fn get_doc(attrs: Vec<syn::Attribute>) -> String {
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
    doc_lines.join("\n")
}

/// Generate the match arms for the `doc_sub` method.
///
/// Takes an iterator of (name, attributes) pairs and generates a match expression that returns the documentation for the given name.
fn generate_arms<I>(iter: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = (String, Vec<syn::Attribute>)>,
{
    let arms = iter.map(|(name, attrs)| {
        let doc = get_doc(attrs);
        let lit_doc = LitStr::new(&doc, Span::call_site());
        quote! { #name => Some(#lit_doc), }
    });
    quote! {
        match name {
            #(#arms)*
            _ => None,
        }
    }
}

#[proc_macro_derive(DocFor)]
pub fn doc_for_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let doc = get_doc(input.attrs);
    let lit_doc = syn::LitStr::new(&doc, Span::call_site());

    let expanded = quote! {
        impl ::doc_for::DocFor for #name {
            const DOC: &'static str = #lit_doc;
        }
    };
    expanded.into()
}

#[proc_macro_derive(DocSub)]
pub fn doc_sub_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let doc_sub_body = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                generate_arms(fields.named.into_iter().map(|f| {
                    (f.ident.unwrap().to_string(), f.attrs)
                }))
            }
            _ => quote! { None },
        },
        Data::Union(data_union) => {
            generate_arms(data_union.fields.named.into_iter().map(|f| {
                (f.ident.unwrap().to_string(), f.attrs)
            }))
        }
        Data::Enum(data_enum) => {
            generate_arms(data_enum.variants.into_iter().map(|v| {
                (v.ident.to_string(), v.attrs)
            }))
        }
    };

    let expanded = quote! {
        impl ::doc_for::DocSub for #name {
            fn doc_sub(name: &str) -> Option<&'static str> {
                #doc_sub_body
            }
        }
    };
    expanded.into()
}
