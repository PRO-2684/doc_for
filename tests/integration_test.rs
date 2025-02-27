#![allow(dead_code)]

#[test]
fn doc_for_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_empty() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert!(doc_for!(MyStruct).is_none());
}

#[test]
fn doc_for_renamed() {
    use doc_for::{doc_for, DocFor as RenamedDocFor};

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_override_trait() {
    use doc_for::{doc_for, DocFor as RenamedDocFor};

    trait DocFor {
        const WHAT: &'static str;
    }

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_override_const() {
    use doc_for::{doc_for, DocFor as RenamedDocFor};

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    impl MyStruct {
        const DOC: &'static str = " Some other documentation";
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_submod() {
    use doc_for::doc_for;

    mod sub {
        use doc_for::DocFor;

        /// Some documentation
        #[derive(DocFor)]
        pub struct MyStruct {
            field: i32,
        }
    }

    assert_eq!(doc_for!(sub::MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_tuple_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct(i32);

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn doc_for_enum() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    enum MyEnum {
        /// Variant documentation
        Variant,
    }

    assert_eq!(doc_for!(MyEnum).unwrap(), " Some documentation");
}

#[test]
fn doc_for_union() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    union MyUnion {
        field: i32,
    }

    assert_eq!(doc_for!(MyUnion).unwrap(), " Some documentation");
}

#[test]
fn doc_for_sub_struct() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    struct MyStruct {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyStruct, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyStruct, not_documented).is_none());
    // assert_eq!(doc_for!(MyStruct, unknown_field), None);
}

// #[test]
// fn doc_for_sub_tuple_struct() {
//     use doc_for::{doc_for, DocFor};

//     #[derive(DocFor)]
//     struct MyStruct(
//         /// Field documentation
//         i32
//     );

//     assert_eq!(doc_for!(MyStruct, 0).unwrap(), " Field documentation");
//     assert_eq!(doc_for!(MyStruct, 1), None);
// }

#[test]
fn doc_for_sub_enum() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    enum MyEnum {
        /// Variant documentation
        Variant,
        NotDocumented,
    }

    assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
    assert!(doc_for!(MyEnum, NotDocumented).is_none());
    // assert_eq!(doc_for!(MyEnum, UnknownVariant), None);
}

#[test]
fn doc_for_sub_union() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    union MyUnion {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyUnion, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyUnion, not_documented).is_none());
    // assert_eq!(doc_for!(MyUnion, unknown_field), None);
}

#[test]
fn mixed_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
    assert_eq!(doc_for!(MyStruct, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyStruct, not_documented).is_none());
    // assert_eq!(doc_for!(MyStruct, unknown_field), None);
}

// #[test]
// fn mixed_tuple_struct() {
//     use doc_for::{doc_for, DocFor};

//     /// Some documentation
//     #[derive(DocFor)]
//     struct MyStruct(
//         /// Field documentation
//         i32
//     );

//     assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
//     assert_eq!(doc_for!(MyStruct, 0).unwrap(), "");
//     assert_eq!(doc_for!(MyStruct, 1), None);
// }

#[test]
fn mixed_enum() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    enum MyEnum {
        /// Variant documentation
        Variant,
        NotDocumented,
    }

    assert_eq!(doc_for!(MyEnum).unwrap(), " Some documentation");
    assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
    assert!(doc_for!(MyEnum, NotDocumented).is_none());
    // assert_eq!(doc_for!(MyEnum, UnknownVariant), None);
}

#[test]
fn mixed_union() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    union MyUnion {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyUnion).unwrap(), " Some documentation");
    assert_eq!(doc_for!(MyUnion, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyUnion, not_documented).is_none());
    // assert_eq!(doc_for!(MyUnion, unknown_field), None);
}
