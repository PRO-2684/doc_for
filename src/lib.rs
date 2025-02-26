//! # doc_for
//!
//! 📖 Get the documentation comment for a [type](https://doc.rust-lang.org/reference/types.html).
//!
//! ## Usage
//!
//! First, bring `DocFor` and `doc_for!` into scope:
//!
//! ```rust
//! use doc_for::{DocFor, doc_for};
//! ```
//!
//! Then, derive the `DocFor` trait for your type:
//!
//! ```rust
//! # use doc_for::{DocFor, doc_for};
//! #
//! /// Some documentation
//! #[derive(DocFor)]
//! struct MyStruct {
//!     field: i32,
//! }
//! ```
//!
//! Finally, use the `doc_for!` macro to get the documentation comment:
//!
//! ```rust
//! # use doc_for::{DocFor, doc_for};
//! #
//! # /// Some documentation
//! # #[derive(DocFor)]
//! # struct MyStruct {
//! #     field: i32,
//! # }
//! assert_eq!(doc_for!(MyStruct), " Some documentation");
//! ```
//!
//! Note that the leading spaces are preserved. Multi-line comments are also supported:
//!
//! ```rust
//! # use doc_for::{DocFor, doc_for};
//! #
//! /// Some documentation
//! /// that spans multiple lines
//! ///
//! /// Additional information
//! #[derive(DocFor)]
//! struct MyStruct {
//!     field: i32,
//! }
//! assert_eq!(doc_for!(MyStruct), r#" Some documentation
//!  that spans multiple lines
//!
//!  Additional information"#);

pub use doc_for_derive::DocFor;

/// Get the documentation comment for a type.
#[macro_export]
macro_rules! doc_for {
    ($t:ty) => {
        <$t as DocFor>::DOC
    };
}

/// Trait for types that have a documentation comment.
pub trait DocFor {
    const DOC: &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy struct
    ///
    /// This is a dummy struct used for testing.
    #[derive(DocFor)]
    #[allow(dead_code)]
    struct MyStruct {
        a: i32,
        b: i32,
    }

    #[test]
    fn doc_for_my_struct() {
        let doc = doc_for!(MyStruct);
        assert_eq!(doc, " Dummy struct\n\n This is a dummy struct used for testing.");
    }
}
