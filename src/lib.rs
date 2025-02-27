#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

pub use doc_for_derive::DocFor;

/// Trait for types that allows getting the documentation comment for the type.
pub trait DocFor {
    /// The documentation comment for the type.
    const DOC: Option<&'static str>;
}

/// Get the documentation comment for a type.
#[macro_export]
macro_rules! doc_for {
    ($t:ty) => {
        <$t as $crate::DocFor>::DOC
    };
    ($t:ty, $field:ident) => {
        // <$t>::doc_for_field(stringify!($field))
        // Force compile-time evaluation
        {
            const DOC: Option<&'static str> = <$t>::doc_for_field(stringify!($field));
            DOC
        }
    };
}
