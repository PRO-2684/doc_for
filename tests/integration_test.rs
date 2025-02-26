#![allow(dead_code)]

#[test]
fn doc_for_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_renamed() {
    use doc_for::{doc_for, DocFor as RenamedDocFor};

    /// Some documentation
    #[derive(RenamedDocFor)]
    struct MyStruct {
        field: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
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

    assert_eq!(doc_for!(MyStruct), " Some documentation");
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

    assert_eq!(doc_for!(MyStruct), " Some documentation");
}

#[test]
fn doc_for_sub() {
    use doc_for::doc_for;

    mod sub {
        use doc_for::DocFor;

        /// Some documentation
        #[derive(DocFor)]
        pub struct MyStruct {
            field: i32,
        }
    }

    assert_eq!(doc_for!(sub::MyStruct), " Some documentation");
}

#[test]
fn doc_for_tuple_struct() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    struct MyStruct(i32);

    assert_eq!(doc_for!(MyStruct), " Some documentation");
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

    assert_eq!(doc_for!(MyEnum), " Some documentation");
}

#[test]
fn doc_for_union() {
    use doc_for::{doc_for, DocFor};

    /// Some documentation
    #[derive(DocFor)]
    union MyUnion {
        field: i32,
    }

    assert_eq!(doc_for!(MyUnion), " Some documentation");
}

#[test]
fn doc_sub_struct() {
    use doc_for::{doc_sub, DocSub};

    #[derive(DocSub)]
    struct MyStruct {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_sub!(MyStruct, field).unwrap(), " Field documentation");
    assert_eq!(doc_sub!(MyStruct, not_documented).unwrap(), "");
    assert_eq!(doc_sub!(MyStruct, unknown_field), None);
}

#[test]
fn doc_sub_enum() {
    use doc_for::{doc_sub, DocSub};

    #[derive(DocSub)]
    enum MyEnum {
        /// Variant documentation
        Variant,
        NotDocumented,
    }

    assert_eq!(doc_sub!(MyEnum, Variant).unwrap(), " Variant documentation");
    assert_eq!(doc_sub!(MyEnum, NotDocumented).unwrap(), "");
    assert_eq!(doc_sub!(MyEnum, UnknownVariant), None);
}

#[test]
fn doc_sub_union() {
    use doc_for::{doc_sub, DocSub};

    #[derive(DocSub)]
    union MyUnion {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_sub!(MyUnion, field).unwrap(), " Field documentation");
    assert_eq!(doc_sub!(MyUnion, not_documented).unwrap(), "");
    assert_eq!(doc_sub!(MyUnion, unknown_field), None);
}

#[test]
fn mixed_struct() {
    use doc_for::{doc_for, doc_sub, DocFor, DocSub};

    /// Some documentation
    #[derive(DocFor, DocSub)]
    struct MyStruct {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyStruct), " Some documentation");
    assert_eq!(doc_sub!(MyStruct, field).unwrap(), " Field documentation");
    assert_eq!(doc_sub!(MyStruct, not_documented).unwrap(), "");
    assert_eq!(doc_sub!(MyStruct, unknown_field), None);
}

#[test]
fn mixed_enum() {
    use doc_for::{doc_for, doc_sub, DocFor, DocSub};

    /// Some documentation
    #[derive(DocFor, DocSub)]
    enum MyEnum {
        /// Variant documentation
        Variant,
        NotDocumented,
    }

    assert_eq!(doc_for!(MyEnum), " Some documentation");
    assert_eq!(doc_sub!(MyEnum, Variant).unwrap(), " Variant documentation");
    assert_eq!(doc_sub!(MyEnum, NotDocumented).unwrap(), "");
    assert_eq!(doc_sub!(MyEnum, UnknownVariant), None);
}

#[test]
fn mixed_union() {
    use doc_for::{doc_for, doc_sub, DocFor, DocSub};

    /// Some documentation
    #[derive(DocFor, DocSub)]
    union MyUnion {
        /// Field documentation
        field: i32,
        not_documented: i32,
    }

    assert_eq!(doc_for!(MyUnion), " Some documentation");
    assert_eq!(doc_sub!(MyUnion, field).unwrap(), " Field documentation");
    assert_eq!(doc_sub!(MyUnion, not_documented).unwrap(), "");
    assert_eq!(doc_sub!(MyUnion, unknown_field), None);
}


