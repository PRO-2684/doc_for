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

#[test]
fn doc_sub_field() {
    use doc_for::{DocSub, doc_sub};

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
