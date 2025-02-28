//! Parsing attributes for `doc_impl` attribute macro.

use syn::spanned::Spanned;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Error, Expr, ExprLit, ExprPath, Lit, MetaNameValue, Token};

// Helper functions

/// Parses `Option<usize>` from `Expr`, mapping `all` to `None` and `n` to `Some(n)`.
fn parse_option_usize(expr: Expr) -> Result<Option<usize>> {
    match expr {
        Expr::Path(ExprPath { path, .. }) if path.is_ident("all") => {
            Ok(None)
        }
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => {
            let n: usize = lit_int.base10_parse()?;
            Ok(Some(n))
        }
        _ => panic!("Expected `all` or integer literal"),
    }
}

/// Parses `bool` from `Expr`.
fn parse_bool(expr: Expr) -> Result<bool> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(lit_bool),
            ..
        }) => Ok(lit_bool.value),
        _ => Err(Error::new(expr.span(), "Expected boolean literal")),
    }
}

/// Attributes for the `doc_impl` attribute macro.
pub struct Attrs {
    /// The number of leading whitespace characters to strip from the documentation comments.
    ///
    /// If `None`, all will be stripped; if `Some(n)`, `n` whitespace characters will be stripped, if present. Default is `Some(0)`.
    ///
    /// When parsing, `all` is mapped to `None` and `n` to `Some(n)`.
    pub strip: Option<usize>,
    /// Whether to generate implementation for `DocFor` and `doc_for_field`. Default is `true`.
    pub doc_for: bool,
    /// Whether to generate implementation for `DocDyn` for an enum. Default is `false`.
    pub doc_dyn: bool,
}

impl Default for Attrs {
    fn default() -> Self {
        Self { strip: Some(0), doc_for: true, doc_dyn: false }
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut attrs = Attrs::default();

        for mnv in parsed {
            let (name, value) = (
                mnv.path.get_ident().ok_or(Error::new(mnv.span(), "Expected an identifier"))?,
                mnv.value,
            );
            match name.to_string().as_str() {
                "strip" => {
                    attrs.strip = parse_option_usize(value)?;
                }
                "doc_for" => {
                    attrs.doc_for = parse_bool(value)?;
                }
                "doc_dyn" => {
                    attrs.doc_dyn = parse_bool(value)?;
                }
                _ => return Err(Error::new(name.span(), "Unknown attribute")),
            }
        }

        Ok(attrs)
    }
}

