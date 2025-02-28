#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

pub use doc_for_derive::{DocDyn, DocFor, doc_impl};

/// Trait for types that allows getting the documentation comment for the type.
pub trait DocFor {
    /// The documentation comment for the type.
    const DOC: Option<&'static str>;
}

/// Trait for enums that allows getting the documentation comment for the variant.
pub trait DocDyn {
    /// The documentation comment for the variant.
    fn doc_dyn(&self) -> Option<&'static str>;
}

/// Force compile-time evaluation. Used internally.
#[doc(hidden)]
#[macro_export]
macro_rules! force_const {
    ($t:ty, $e:expr) => {{
        const VALUE: $t = $e;
        VALUE
    }};
}

/// Get the documentation comment for a type or its fields, returning `None` if not documented.
///
/// # Examples
///
/// ```rust
/// use doc_for::{DocFor, doc_for};
///
/// /// Some documentation
/// #[derive(DocFor)]
/// struct MyStruct {
///     /// Documentation for the field
///     field: i32,
///     not_documented: i32,
/// }
///
/// assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
/// assert_eq!(doc_for!(MyStruct, field).unwrap(), " Documentation for the field");
/// assert!(doc_for!(MyStruct, not_documented).is_none());
/// ```
///
/// Also works with enums and unions.
///
/// # Panics
///
/// Panics and fails the compilation if the type does not derive `DocFor`, or if the field or variant does not exist.
#[macro_export]
macro_rules! doc_for {
    ($t:ty) => {
        // Type
        <$t as $crate::DocFor>::DOC
    };
    ($t:ty, $field:ident) => {
        // Field
        $crate::force_const!(
            Option<&'static str>,
            <$t>::doc_for_field(stringify!($field))
        )
    };
    ($t:ty, $index:expr) => {
        // Tuple field (`field_<index>`)
        $crate::force_const!(Option<&'static str>, <$t>::doc_for_field($index))
    };
}

/// Get the documentation comment for a type or its fields. Basically [`doc_for!`] with `unwrap`.
///
/// # Panics
///
/// Panics and fails the compilation if the type does not derive `DocFor`, the field or variant does not exist, or not documented.
#[macro_export]
macro_rules! doc {
    ($t:ty) => {
        $crate::force_const!(
            &'static str,
            $crate::doc_for!($t).expect("The type is not documented")
        )
    };
    ($t:ty, $field:ident) => {
        $crate::force_const!(
            &'static str,
            $crate::doc_for!($t, $field).expect("The field or variant is not documented")
        )
    };
    ($t:ty, $index:expr) => {
        $crate::force_const!(
            &'static str,
            $crate::doc_for!($t, $index).expect("The field or variant is not documented")
        )
    };
}
