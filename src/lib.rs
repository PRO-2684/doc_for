#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

pub use doc_for_derive::{DocFor, DocSub};

/// Trait for types that allows getting the documentation comment for the type.
pub trait DocFor {
    /// The documentation comment for the type.
    const DOC: &'static str;
}

/// Get the documentation comment for a type.
#[macro_export]
macro_rules! doc_for {
    ($t:ty) => {
        <$t as $crate::DocFor>::DOC
    };
}

/// Trait for types that allows getting the documentation comments for its fields.
pub trait DocSub {
    /// Get documentation comment for the fields of the type.
    fn doc_sub(field: &str) -> Option<&'static str>;
}

/// Get the documentation comment for a field of a type.
#[macro_export]
macro_rules! doc_sub {
    ($t:ty, $field:ident) => {
        <$t as $crate::DocSub>::doc_sub(stringify!($field))
    };
}
