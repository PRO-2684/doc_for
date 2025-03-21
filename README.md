# `doc_for`

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doc_for?logo=opensourceinitiative)](https://github.com/PRO-2684/doc_for/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![docs.rs](https://img.shields.io/docsrs/doc_for?logo=rust)](https://docs.rs/doc_for)

> [WARNING]
> This crate is still in development, and the API is subject to BREAKING CHANGES.

📖 Get the documentation comment for structs, enums and unions, in a zero-cost fashion.

## 🪄 Features

- **Zero-cost**: All work is done at compile-time
- **Simple**: Just annotate your struct with `#[doc_impl]` and use the `doc_for!` or `doc!` macro

## 🤔 Usage

### Get the documentation comment for a type

First, bring `doc_for` and `doc_impl` into scope:

```rust
use doc_for::{doc_for, doc_impl};
```

Then, annotate your struct with `#[doc_impl]` attribute macro:

```rust
# use doc_for::{doc_for, doc_impl};
#
/// Some documentation
#[doc_impl]
struct MyStruct {
    field: i32,
}
```

Finally, use the `doc_for!` macro to get the documentation comment, which returns an `Option<&'static str>`:

```rust
# use doc_for::{doc_for, doc_impl};
#
# /// Some documentation
# #[doc_impl]
# struct MyStruct {
#     field: i32,
# }
assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
```

Note that the leading spaces are preserved. Multi-line comments are also supported:

```rust
# use doc_for::{doc_for, doc_impl};
#
/// Some documentation
/// that spans multiple lines
///
/// Additional information
#[doc_impl]
struct MyStruct {
    field: i32,
}
assert_eq!(doc_for!(MyStruct).unwrap(), r#" Some documentation
 that spans multiple lines

 Additional information"#);
```

If the type does not have a documentation comment, `doc_for!` will return `None`:

```rust
# use doc_for::{doc_for, doc_impl};
#
// No documentation comment here
#[doc_impl]
struct MyStruct {
    field: i32,
}
assert!(doc_for!(MyStruct).is_none());
```

Also works with tuple structs, enums and unions:

```rust
# use doc_for::{doc_for, doc_impl};
#
/// Tuple struct documentation
#[doc_impl]
struct MyTupleStruct(i32);
assert_eq!(doc_for!(MyTupleStruct).unwrap(), " Tuple struct documentation");

/// Enum documentation
#[doc_impl]
enum MyEnum {
    Variant,
}
assert_eq!(doc_for!(MyEnum).unwrap(), " Enum documentation");

/// Union documentation
#[doc_impl]
union MyUnion {
    field: i32,
}
assert_eq!(doc_for!(MyUnion).unwrap(), " Union documentation");
```

### Get the documentation comment for fields and variants

Same as before, bring `doc_impl` and `doc_for!` into scope and annotate your struct with `#[doc_impl]` attribute macro:

```rust
use doc_for::{doc_for, doc_impl};

#[doc_impl]
struct MyStruct {
    /// Field documentation
    field: i32,
    not_documented: i32,
}
```

Then, use the `doc_for!` macro to get the documentation comment. If the field does not have a documentation comment, `doc_for!` will return `None`:

```rust
# use doc_for::{doc_for, doc_impl};
#
# #[doc_impl]
# struct MyStruct {
#     /// Field documentation
#     field: i32,
#     not_documented: i32,
# }
assert_eq!(doc_for!(MyStruct, field).unwrap(), " Field documentation");
assert!(doc_for!(MyStruct, not_documented).is_none());
```

If the field or variant does not exist, `doc_for!` will panic, thus failing the compilation:

```rust compile_fail
# use doc_for::{doc_for, doc_impl};
#
# #[doc_impl]
# struct MyStruct {
#     /// Field documentation
#     field: i32,
#     not_documented: i32,
# }
// Won't compile due to `The field or variant does not exist`
assert!(doc_for!(MyStruct, non_existent).is_none());
```

Similarly, it also works with union fields (not listed here), enum variants and tuple struct fields:

```rust
# use doc_for::{doc_for, doc_impl};
#
#[doc_impl]
enum MyEnum {
    /// Variant documentation
    Variant,
    NotDocumented,
}
assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
assert!(doc_for!(MyEnum, NotDocumented).is_none());
// Won't compile due to `The field or variant does not exist`
// assert_eq!(doc_for!(MyEnum, NonExistent), None);

#[doc_impl]
struct MyTupleStruct(
    /// Tuple struct field documentation
    i32,
    i32,
);
assert_eq!(doc_for!(MyTupleStruct, 0).unwrap(), " Tuple struct field documentation");
assert!(doc_for!(MyTupleStruct, 1).is_none());
// Won't compile due to `The field or variant does not exist`
// assert_eq!(doc_for!(MyTupleStruct, 2), None);
```

### Stripping the documentation comment

The `strip` attribute can be used to strip leading whitespace characters of the documentation comment. If `all`, all will be stripped; if `n`, at most `n` whitespace characters will be stripped. Default is `0`.

```rust
use doc_for::{doc_for, doc_impl};

/// Some documentation
#[doc_impl(strip = 1)]
struct MyStruct {
    field: i32,
}
assert_eq!(doc_for!(MyStruct).unwrap(), "Some documentation");

///  Two leading spaces
#[doc_impl(strip = 1)]
struct TwoLeadingSpaces {
    field: i32,
}
assert_eq!(doc_for!(TwoLeadingSpaces).unwrap(), " Two leading spaces");

///          Too many spaces
#[doc_impl(strip = all)]
struct TooManySpaces {
    field: i32,
}
assert_eq!(doc_for!(TooManySpaces).unwrap(), "Too many spaces");
```

### If you don't care about the `Option`

The `doc!` macro is basically `doc_for!` with `unwrap`:

```rust
use doc_for::{doc, doc_impl};

#[doc_impl]
struct MyStruct {
    /// Field documentation
    field: i32,
    not_documented: i32,
}

assert_eq!(doc!(MyStruct, field), " Field documentation");
```

...So it panics and fails the compilation if the requested type or field is not documented:

```rust compile_fail
# use doc_for::{doc, doc_impl};
#
# #[doc_impl]
# struct MyStruct {
#    /// Field documentation
#    field: i32,
#    not_documented: i32,
# }
#
// Won't compile due to `The type is not documented`
println!("{}", doc!(MyStruct));
```

```rust compile_fail
# use doc_for::{doc, doc_impl};
#
# #[doc_impl]
# struct MyStruct {
#    /// Field documentation
#    field: i32,
#    not_documented: i32,
# }
#
// Won't compile due to `The field or variant is not documented`
println!("{}", doc!(MyStruct, not_documented));
```

As you might have expected, trying to access a non-existent field or variant will still fail the compilation:

```rust compile_fail
# use doc_for::{doc, doc_impl};
#
# #[doc_impl]
# struct MyStruct {
#    /// Field documentation
#    field: i32,
#    not_documented: i32,
# }
#
// Won't compile due to `The field or variant does not exist`
println!("{}", doc!(MyStruct, non_existent));
```

### Get the documentation comment for an enum variant

This time, bring `DocDyn` and `doc_impl` into scope:

```rust
use doc_for::{DocDyn, doc_impl};
```

Then, annotate your enum with `#[doc_impl(doc_for = false, doc_dyn = true)]`:

```rust
# use doc_for::{DocDyn, doc_impl};
#
#[doc_impl(doc_for = false, doc_dyn = true)]
enum MyEnum {
    /// Variant documentation
    Variant,
    NotDocumented,
}
```

Finally, call the `doc_dyn` method on the enum variant:

```rust
# use doc_for::{DocDyn, doc_impl};
#
# #[doc_impl(doc_for = false, doc_dyn = true)]
# enum MyEnum {
#     /// Variant documentation
#     Variant,
#     NotDocumented,
# }
assert_eq!(MyEnum::Variant.doc_dyn().unwrap(), " Variant documentation");
assert!(MyEnum::NotDocumented.doc_dyn().is_none());
```

Note that this method is not zero-cost, as it matches the enum variant at runtime.

To use both `doc_for!` and `doc_dyn` on the same enum, annotate it with `#[doc_impl(doc_dyn = true)]`. You can include `doc_for = true` if you want, but since it's the default, it's not necessary.

```rust
# use doc_for::{DocDyn, doc_for, doc_impl};
#
#[doc_impl(doc_dyn = true, strip = 1)]
enum MyEnum {
    /// Variant documentation
    Variant,
    NotDocumented,
}

assert_eq!(doc_for!(MyEnum, Variant).unwrap(), "Variant documentation");
assert_eq!(MyEnum::Variant.doc_dyn().unwrap(), "Variant documentation");
```

### Automatically generate attribute macros with documentation as parameters

Consider the following scenario:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    /// Error1 message
    #[error("Error1 message")]
    Error1,
    /// Error2 message
    #[error("Error2 message")]
    Error2,
}
```

Which seems quite repetitive. Luckily, `doc_impl` provides a way to automatically generate those repetitive attribute macros. Simply append `gen_attr = "your_attr({doc})"`, and respective documentation will take the place of `{doc}`:

```rust
use doc_for::doc_impl;
use thiserror::Error;

/// Some documentation
#[doc_impl(strip = 1, doc_for = false, gen_attr = "error({doc})")]
#[derive(Debug, Error)]
enum MyError {
    /// Error1 message
    Error1,
    /// Error2 message
    Error2,
}

assert_eq!(format!("{}", MyError::Error1), "Error1 message");
assert_eq!(format!("{}", MyError::Error2), "Error2 message");
```

Also works on struct fields, where you might want to generate `#[serde(rename = "...")]` attributes:

```rust
use doc_for::doc_impl;
use serde::Deserialize;

/// Some documentation
#[doc_impl(strip = 1, doc_for = false, gen_attr = "serde(rename = {doc})")]
#[derive(Deserialize)]
struct MyStruct {
    /// field1_rename
    // No need for #[serde(rename = "field1_rename")]
    field1: i32,
    /// field2_rename
    // No need for #[serde(rename = "field2_rename")]
    field2: i32,
}

let json = r#"{"field1_rename": 1, "field2_rename": 2}"#;
let my_struct: MyStruct = serde_json::from_str(json).unwrap();
assert_eq!(my_struct.field1, 1);
assert_eq!(my_struct.field2, 2);
```

Do note that:

- `doc_impl` annotation must be placed BEFORE attribute macros that introduced the target attribute.
- `gen_attr` can be used multiple times.

### The `derive` alternative

If you prefer to use `derive`, you can use `DocFor` and `DocDyn` to replace `doc_for` and `doc_dyn` respectively:

```rust
use doc_for::{DocDyn, DocFor, doc_for};

#[derive(DocFor, DocDyn)]
/// Some documentation
enum MyEnum {
    /// Variant documentation
    Variant,
    NotDocumented,
}

assert_eq!(doc_for!(MyEnum).unwrap(), " Some documentation");
assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
assert_eq!(MyEnum::Variant.doc_dyn().unwrap(), " Variant documentation");
```

However, you won't be able to configure the `strip` and `gen_attr` attribute in this case.

## ⚙️ Implementation

### `DocFor` and `doc_for!`

The `doc_for` crate provides a `DocFor` trait and a `doc_for!` macro:

- The `DocFor` trait requires an associated constant `DOC` to be implemented for the type
- Deriving the `DocFor` trait sets the `DOC` constant as the documentation comment of the type, and generates a `const fn doc_for_field(name) -> Option<&'static str>` function
    - Currently Rust doesn't support constant functions in traits, so the `doc_for_field` function is implemented directly on the annotated type
    - If the annotated type is a struct, union or enum, the `name` parameter accepts a `&'static str`
    - If the annotated type is a tuple struct, the `name` parameter accepts an `usize`
- If given a type, the `doc_for!` macro retrieves the value of this constant; If given a type and a field name, the `doc_for!` macro calls the `doc_for_field` function with the given field name

Using these APIs is zero-cost, as all the work is done at compile-time:

- When compiled, types that derive `DocFor` will have their documentation comments inlined as associated constants or in constant functions
- Calls to `doc_for!` will be replaced with the value of the associated constant or the result of the constant function

### `DocDyn` and `doc_dyn`

The `doc_for` crate also provides a `DocDyn` trait and a `doc_dyn` method:

- The `DocDyn` trait requires a `doc_dyn` method to be implemented for the type, which returns an `Option<&'static str>`
- Deriving the `DocDyn` trait generates a `doc_dyn` method, which returns the documentation comment that matches the variant of the enum

This method is not zero-cost, as it matches the enum variant at runtime.

### `doc_impl`

The `doc_impl` attribute macro is used to derive the `DocFor` and `DocDyn` traits for a type, along with configuring the `strip` attribute. `gen_attr` attribute, when set, prepends the specified attribute macros to fields or variants.

## ✅ TODO

- [x] Strip each line of the documentation comment, via a `strip` attribute
- [ ] Better error reporting and handling
- [ ] Access module documentation (e.g. `doc_for!(my_module)`)
- [ ] Access trait documentation (e.g. `doc_for!(MyTrait)`)
- [ ] Access sub-item documentation
    - [x] Access field documentation (e.g. `doc_for!(MyStruct, field)` or `doc_for!(MyUnion, field)`)
    - [x] Access tuple struct field documentation (e.g. `doc_for!(MyTupleStruct, 0)`)
    - [x] Access enum variant documentation (statically) (e.g. `doc_for!(MyEnum, Variant)`)
    - [x] Access enum variant documentation (dynamically) (e.g. `doc_for!(my_enum_variant)`)
    - [ ] Access method documentation (e.g. `doc_for!(MyStruct, method)`)
    - [ ] Access associated constant documentation (e.g. `doc_for!(MyStruct, CONSTANT)`)
    - [ ] Access associated type documentation (e.g. `doc_for!(MyStruct, Type)`)
