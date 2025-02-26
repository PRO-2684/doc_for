# doc_for

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doc_for?logo=opensourceinitiative)](https://github.com/PRO-2684/doc_for/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doc_for?logo=rust)](https://crates.io/crates/doc_for)
[![docs.rs](https://img.shields.io/docsrs/doc_for?logo=rust)](https://docs.rs/doc_for)

📖 Get the documentation comment for a [type](https://doc.rust-lang.org/reference/types.html).

## 🤔 Usage

First, bring `DocFor` and `doc_for!` into scope:

```rust
use doc_for::{DocFor, doc_for};
```

Then, derive the `DocFor` trait for your type:

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

## ✅ TODO

- [ ] Access module documentation (e.g. `doc_for!(my_module)`)
- [ ] Access trait documentation (e.g. `doc_for!(MyTrait)`)
- [ ] Access sub-item documentation
    - [ ] Access field documentation (e.g. `doc_for!(MyStruct::field)`)
    - [ ] Access method documentation (e.g. `doc_for!(MyStruct::method)`)
    - [ ] Access associated constant documentation (e.g. `doc_for!(MyStruct::CONSTANT)`)
    - [ ] Access associated type documentation (e.g. `doc_for!(MyStruct::Type)`)
    - [ ] Access enum variant documentation (e.g. `doc_for!(MyEnum::Variant)`)
