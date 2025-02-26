#![allow(dead_code)]

#[test]
fn doc_for_struct() {
    use doc_for::{DocFor, doc_for};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_renamed() {
    use doc_for::{DocFor as RenamedDocFor, doc_for};

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_override() {
    use doc_for::{DocFor as RenamedDocFor, doc_for};

    trait DocFor {
        const WHAT: &'static str;
    }

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_tuple_struct() {
    use doc_for::{DocFor, doc_for};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct(i32);

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_enum() {
    use doc_for::{DocFor, doc_for};

    /// Some documentation
    #[derive(DocFor)]
    enum MyEnum {
        /// Variant documentation
        Variant,
    }

    assert_eq!(doc_for!(MyEnum), " Some documentation");
}

// Not supported yet
// #[test]
// fn doc_for_enum_variant() {
//     use doc_for::{DocFor, doc_for};

//     #[derive(DocFor)]
//     enum MyEnum {
//         /// Variant documentation
//         Variant,
//     }

//     assert_eq!(doc_for!(MyEnum::Variant), " Variant documentation");
// }

#[test]
fn doc_for_union() {
    use doc_for::{DocFor, doc_for};

    /// Some documentation
    #[derive(DocFor)]
    union MyUnion {
        field: i32,
    }

    assert_eq!(doc_for!(MyUnion), " Some documentation");
}
