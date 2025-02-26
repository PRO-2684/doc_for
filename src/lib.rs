#![doc = include_str!("../README.md")]

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

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
    /// The documentation comment for the type.
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
