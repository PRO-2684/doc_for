# doc_for

📖 Get the documentation comment for a [type](https://doc.rust-lang.org/reference/types.html).

## Usage

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
