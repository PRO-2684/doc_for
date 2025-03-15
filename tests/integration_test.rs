#![allow(dead_code)]

#[test]
fn derive_doc_for_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn derive_doc_for_empty() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert!(doc_for!(MyStruct).is_none());
}

#[test]
fn derive_doc_for_renamed() {
    use doc_for::{doc_for, DocFor as RenamedDocFor};

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn derive_doc_for_override_trait() {
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
fn derive_doc_for_override_const() {
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
fn derive_doc_for_submod() {
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
fn derive_doc_for_tuple_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct(i32);

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
}

#[test]
fn derive_doc_for_enum() {
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
fn derive_doc_for_union() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    union MyUnion {
        field: i32,
    }

    assert_eq!(doc_for!(MyUnion).unwrap(), " Some documentation");
}

#[test]
fn derive_doc_for_sub_struct() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    struct MyStruct {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyStruct, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyStruct, not_documented).is_none());
    // assert_eq!(doc_for!(MyStruct, unknown_field), None); // Won't compile
}

#[test]
fn derive_doc_for_sub_tuple_struct() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    struct MyStruct(
        /// Field documentation
        i32,
    );

    assert_eq!(doc_for!(MyStruct, 0).unwrap(), " Field documentation");
    // assert_eq!(doc_for!(MyStruct, 1), None); // Won't compile
}

#[test]
fn derive_doc_for_sub_enum() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    enum MyEnum {
        /// Variant documentation
        Variant,
        NotDocumented,
    }

    assert_eq!(doc_for!(MyEnum, Variant).unwrap(), " Variant documentation");
    assert!(doc_for!(MyEnum, NotDocumented).is_none());
    // assert_eq!(doc_for!(MyEnum, UnknownVariant), None); // Won't compile
}

#[test]
fn derive_doc_for_sub_union() {
    use doc_for::{doc_for, DocFor};

    #[derive(DocFor)]
    union MyUnion {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyUnion, field).unwrap(), " Field documentation");
    assert!(doc_for!(MyUnion, not_documented).is_none());
    // assert_eq!(doc_for!(MyUnion, unknown_field), None); // Won't compile
}

#[test]
fn derive_mixed_struct() {
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
    // assert_eq!(doc_for!(MyStruct, unknown_field), None); // Won't compile
}

#[test]
fn derive_mixed_tuple_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct(
        /// Field documentation
        i32,
    );

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
    assert_eq!(doc_for!(MyStruct, 0).unwrap(), " Field documentation");
    // assert_eq!(doc_for!(MyStruct, 1), None); // Won't compile
}

#[test]
fn derive_mixed_enum() {
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
    // assert_eq!(doc_for!(MyEnum, UnknownVariant), None); // Won't compile
}

#[test]
fn derive_mixed_union() {
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
    // assert_eq!(doc_for!(MyUnion, unknown_field), None); // Won't compile
}

#[test]
fn derive_doc_for_unwrap() {
    use doc_for::doc;

    /// Some documentation
    #[derive(doc_for::DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc!(MyStruct), " Some documentation");
    // println!("{}", doc!(MyStruct, field)); // Won't compile
}

#[test]
fn derive_doc_dyn() {
    use doc_for::DocDyn;

    /// Some documentation
    #[derive(DocDyn)]
    enum MyEnum {
        /// Variant documentation
        Variant1,
        Variant2,
    }

    assert_eq!(
        MyEnum::Variant1.doc_dyn().unwrap(),
        " Variant documentation"
    );
    assert!(MyEnum::Variant2.doc_dyn().is_none());
}

#[test]
fn attr_doc_for_default() {
    use doc_for::{doc_for, doc_impl};

    /// Some documentation
    #[doc_impl]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct).unwrap(), " Some documentation");
    // assert_eq!(doc_for!(MyStruct, non_existent_field), None); // Won't compile
}

#[test]
fn attr_doc_for_gen_attrs() {
    use doc_for::doc_impl;
    use thiserror::Error;

    /// Some documentation
    #[doc_impl(strip = 1, gen_attr = "error({doc})")]
    #[derive(Debug, Error)]
    enum MyError {
        /// Error1 message
        Error1,
        /// Error2 message
        Error2,
    }

    // Check Display implementation
    assert_eq!(format!("{}", MyError::Error1), "Error1 message");
    assert_eq!(format!("{}", MyError::Error2), "Error2 message");
}
