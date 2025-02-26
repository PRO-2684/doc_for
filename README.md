# doc_for

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doc_for?logo=opensourceinitiative)](https://github.com/PRO-2684/doc_for/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![docs.rs](https://img.shields.io/docsrs/doc_for?logo=rust)](https://docs.rs/doc_for)

📖 Get the documentation comment for structs, enums and unions.

## 🪄 Features

- **Zero-cost**: All work is done at compile-time
- **Simple**: Just derive the `DocFor` trait and use the `doc_for!` macro

## 🤔 Usage

### Get the documentation comment for a type

First, bring `DocFor` and `doc_for!` into scope:

```rust
use doc_for::{DocFor, doc_for};
```

Then, derive the `DocFor` trait for your struct:

```rust
# use doc_for::{DocFor, doc_for};
#
/// Some documentation
#[derive(DocFor)]
struct MyStruct {
    field: i32,
}
```

Finally, use the `doc_for!` macro to get the documentation comment:

```rust
# use doc_for::{DocFor, doc_for};
#
# /// Some documentation
# #[derive(DocFor)]
# struct MyStruct {
#     field: i32,
# }
assert_eq!(doc_for!(MyStruct), " Some documentation");
```

Note that the leading spaces are preserved. Multi-line comments are also supported:

```rust
# use doc_for::{DocFor, doc_for};
#
/// Some documentation
/// that spans multiple lines
///
/// Additional information
#[derive(DocFor)]
struct MyStruct {
    field: i32,
}
assert_eq!(doc_for!(MyStruct), r#" Some documentation
 that spans multiple lines

 Additional information"#);
```

Also works with tuple structs, enums and unions:

```rust
# use doc_for::{DocFor, doc_for};
#
/// Tuple struct documentation
#[derive(DocFor)]
struct MyTupleStruct(i32);
assert_eq!(doc_for!(MyTupleStruct), " Tuple struct documentation");

/// Enum documentation
#[derive(DocFor)]
enum MyEnum {
    Variant,
}
assert_eq!(doc_for!(MyEnum), " Enum documentation");

/// Union documentation
#[derive(DocFor)]
union MyUnion {
    field: i32,
}
assert_eq!(doc_for!(MyUnion), " Union documentation");
```

### Get the documentation comment for sub-items

This time, bring `DocSub` and `doc_sub!` into scope:

```rust
use doc_for::{DocSub, doc_sub};
```

Then, derive the `DocSub` trait for your struct:

```rust
# use doc_for::{DocSub, doc_sub};
#
#[derive(DocSub)]
struct MyStruct {
    /// Field documentation
    field: i32,
    not_documented: i32,
}
```

Finally, use the `doc_sub!` macro to get the documentation comment. If the field does not have a documentation comment, `doc_sub!` will return `Some("")`; If the field does not exist, `doc_sub!` will return `None`.

```rust
# use doc_for::{DocSub, doc_sub};
#
# #[derive(DocSub)]
# struct MyStruct {
#     /// Field documentation
#     field: i32,
#     not_documented: i32,
# }
assert_eq!(doc_sub!(MyStruct, field).unwrap(), " Field documentation");
assert_eq!(doc_sub!(MyStruct, not_documented).unwrap(), "");
assert_eq!(doc_sub!(MyStruct, non_existent), None);
```

## ⚙️ Implementation

### `doc_for` & `DocFor`

The `doc_for` crate provides a `DocFor` trait and a `doc_for!` macro:

- The `DocFor` trait requires an associated constant `DOC` to be implemented for the type
- The `doc_for!` macro retrieves the value of this constant

The `doc_for_derive` crate provides a derive macro for the `DocFor` trait, which simply sets the `DOC` constant as the documentation comment of the type.

Using these APIs is zero-cost, as all the work is done at compile-time:

- When compiled, types that derive `DocFor` will have their documentation comments inlined as associated constants
- Calls to `doc_for!` will be replaced with the value of the associated constant

## ✅ TODO

- [ ] Access module documentation (e.g. `doc_for!(my_module)`)
- [ ] Access trait documentation (e.g. `doc_for!(MyTrait)`)
- [ ] Access sub-item documentation
    - [x] Access field documentation (e.g. `doc_sub!(MyStruct::field)`)
    - [ ] Access method documentation (e.g. `doc_for!(MyStruct::method)`)
    - [ ] Access associated constant documentation (e.g. `doc_for!(MyStruct::CONSTANT)`)
    - [ ] Access associated type documentation (e.g. `doc_for!(MyStruct::Type)`)
    - [ ] Access enum variant documentation (e.g. `doc_for!(MyEnum::Variant)`)
