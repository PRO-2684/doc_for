#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

pub use doc_for_derive::DocFor;

/// Get the documentation comment for a type.
#[macro_export]
macro_rules! doc_for {
    ($t:ty) => {
        <$t as $crate::DocFor>::DOC
    };
}

/// Trait for types that have a documentation comment.
pub trait DocFor {
    /// The documentation comment for the type.
    const DOC: &'static str;
}
