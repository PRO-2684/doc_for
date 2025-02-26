#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta};

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

    // Generate DocSub implementation for structs with named fields.
    let sub_impl = if let syn::Data::Struct(data_struct) = input.data {
        if let syn::Fields::Named(fields) = data_struct.fields {
            let field_matches = fields.named.into_iter().map(|field| {
                let field_name = field.ident.unwrap().to_string();
                let field_doc = get_doc(field.attrs);
                let lit_field_doc = syn::LitStr::new(&field_doc, Span::call_site());
                quote! {
                    #field_name => Some(#lit_field_doc),
                }
            });
            quote! {
                fn doc_sub(field: &str) -> Option<&'static str> {
                    match field {
                        #(#field_matches)*
                        _ => None,
                    }
                }
            }
        } else {
            quote! {
                fn doc_sub(_field: &str) -> Option<&'static str> { None }
            }
        }
    } else {
        quote! {
            fn doc_sub(_field: &str) -> Option<&'static str> { None }
        }
    };

    let expanded = quote! {
        impl ::doc_for::DocSub for #name {
            #sub_impl
        }
    };
    TokenStream::from(expanded)
}
