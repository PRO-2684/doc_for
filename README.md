# `doc_for`

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

### Get the documentation comment for fields and variants

Same as before, bring `DocFor` and `doc_for!` into scope and derive the `DocFor` trait for your struct:

```rust
use doc_for::{DocFor, doc_for};

#[derive(DocFor)]
struct MyStruct {
    /// Field documentation
    field: i32,
    not_documented: i32,
}
```

Finally, use the `doc_for!` macro to get the documentation comment. If the field does not have a documentation comment, `doc_for!` will return `Some("")`; If the field does not exist, `doc_for!` will return `None`.

```rust
# use doc_for::{DocFor, doc_for};
#
# #[derive(DocFor)]
# struct MyStruct {
#     /// Field documentation
#     field: i32,
#     not_documented: i32,
# }
assert_eq!(doc_for!(MyStruct, field).unwrap(), " Field documentation");
assert_eq!(doc_for!(MyStruct, not_documented).unwrap(), "");
assert_eq!(doc_for!(MyStruct, non_existent), None);
```

Similarly, it also works with union fields (not listed here) and enum variants:

```rust
# use doc_for::{DocFor, doc_for};
#
#[derive(DocFor)]
enum MyEnum {
    /// Variant documentation
    Variant,
    NotDocumented,
}
assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
assert_eq!(doc_for!(MyEnum, NotDocumented).unwrap(), "");
assert_eq!(doc_for!(MyEnum, NonExistent), None);
```

## ⚙️ Implementation

The `doc_for` crate provides a `DocFor` trait and a `doc_for!` macro:

- The `DocFor` trait requires an associated constant `DOC` to be implemented for the type
- Deriving the `DocFor` trait sets the `DOC` constant as the documentation comment of the type, and generates a `const fn doc_for_field(name: &'static str) -> Option<&'static str>` function
- If given a type, the `doc_for!` macro retrieves the value of this constant; If given a type and a field name, the `doc_for!` macro calls the `doc_for_field` function with the given field name

Using these APIs is zero-cost, as all the work is done at compile-time:

- When compiled, types that derive `DocFor` will have their documentation comments inlined as associated constants or in constant functions
- Calls to `doc_for!` will be replaced with the value of the associated constant or the result of the constant function

## ✅ TODO

- [ ] Strip each line of the documentation comment, via a `strip` attribute
- [ ] Access module documentation (e.g. `doc_for!(my_module)`)
- [ ] Access trait documentation (e.g. `doc_for!(MyTrait)`)
- [ ] Access sub-item documentation
    - [x] Access field documentation (e.g. `doc_for!(MyStruct, field)` or `doc_for!(MyUnion, field)`)
    - [x] Access enum variant documentation (e.g. `doc_for!(MyEnum, Variant)`)
    - [ ] Access enum variant instance documentation (e.g. `doc_for!(my_enum_variant)`)
    - [ ] Access method documentation (e.g. `doc_for!(MyStruct, method)`)
    - [ ] Access associated constant documentation (e.g. `doc_for!(MyStruct, CONSTANT)`)
    - [ ] Access associated type documentation (e.g. `doc_for!(MyStruct, Type)`)
