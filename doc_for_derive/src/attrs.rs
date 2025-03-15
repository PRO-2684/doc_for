//! Parsing attributes for `doc_impl` attribute macro.

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Expr, ExprLit, ExprPath, Lit, MetaNameValue, Token};

// Helper functions

/// Parses `Option<usize>` from `Expr`, mapping `all` to `None` and `n` to `Some(n)`.
fn parse_option_usize(expr: Expr) -> Result<Option<usize>> {
    match expr {
        Expr::Path(ExprPath { path, .. }) if path.is_ident("all") => Ok(None),
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => {
            let n: usize = lit_int.base10_parse()?;
            Ok(Some(n))
        }
        _ => Err(Error::new(expr.span(), "Expected `all` or integer literal")),
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

/// Parses a string literal from `Expr`.
fn parse_string(expr: &Expr) -> Result<String> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) => Ok(lit_str.value()),
        _ => Err(Error::new(expr.span(), "Expected string literal")),
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
    /// List of attributes to generate for each field. Default is empty.
    pub gen_attrs: Vec<String>,
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            strip: Some(0),
            doc_for: true,
            doc_dyn: false,
            gen_attrs: Vec::new(),
        }
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut attrs = Attrs::default();

        for mnv in parsed {
            let (name, value) = (
                mnv.path
                    .get_ident()
                    .ok_or(Error::new(mnv.span(), "Expected an identifier"))?,
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
                "gen_attr" => {
                    attrs.gen_attrs.push(parse_string(&value)?);
                }
                _ => return Err(Error::new(name.span(), "Unknown attribute")),
            }
        }

        Ok(attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parse_option_usize() {
        assert_eq!(
            parse_option_usize(parse_quote!(all)).unwrap(),
            None,
            "Expected `None` for `all`"
        );
        assert_eq!(
            parse_option_usize(parse_quote!(5)).unwrap(),
            Some(5),
            "Expected `Some(5)` for `5`"
        );
        assert!(parse_option_usize(parse_quote!(true)).is_err(), "Expected error for `true`");
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(
            parse_bool(parse_quote!(true)).unwrap(),
            true,
            "Expected `true`"
        );
        assert_eq!(
            parse_bool(parse_quote!(false)).unwrap(),
            false,
            "Expected `false`"
        );
        assert!(parse_bool(parse_quote!(5)).is_err(), "Expected error for `5`");
    }

    #[test]
    fn test_parse_pair_string() {
        assert_eq!(
            parse_pair_string(parse_quote!(("a", "b"))).unwrap(),
            ("a".to_string(), "b".to_string()),
            "Expected `(\"a\", \"b\")`"
        );
        assert!(parse_pair_string(parse_quote!(("a", "b", "c"))).is_err(), "Expected error for `(\"a\", \"b\", \"c\")`");
        assert!(parse_pair_string(parse_quote!(("a", 5))).is_err(), "Expected error for `(\"a\", 5)`");
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string(&parse_quote!("hello")).unwrap(),
            "hello".to_string(),
            "Expected `\"hello\"`"
        );
        assert!(parse_string(&parse_quote!(5)).is_err(), "Expected error for `5`");
    }

    #[test]
    fn test_parse_attrs() {
        assert_eq!(
            parse_quote!(strip = all, doc_for = false, doc_dyn = true, gen_attr = ("a", "b"), gen_attr = ("c", "d")).parse::<Attrs>().unwrap(),
            Attrs {
                strip: None,
                doc_for: false,
                doc_dyn: true,
                gen_attrs: vec![("a".to_string(), "b".to_string()), ("c".to_string(), "d".to_string())],
            },
            "Expected `strip = all, doc_for = false, doc_dyn = true, gen_attr = (\"a\", \"b\"), gen_attr = (\"c\", \"d\")`"
        );
    }
}
