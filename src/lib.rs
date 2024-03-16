pub use proc_macros::*;
pub use util_procs_internal::*;

#[cfg(test)]
mod tests {
    use proc_macros::FieldNames;
    use util_procs_internal::FieldNames;

    #[derive(FieldNames)]
    struct Foo {
        bar: String,
        id: i64,
        name: String,
        code: u32,
    }

    struct FooManual {
        bar: String,
        id: i64,
        name: String,
        code: u32,
    }

    impl FieldNames for FooManual {
        const FIELD_NAMES: &'static [&'static str] = &["bar", "id", "name", "code"];
    }

    #[test]
    fn field_names_works() {
        let expected = vec![
            "bar".to_string(),
            "id".to_string(),
            "name".to_string(),
            "code".to_string(),
        ];

        assert_eq!(&expected, Foo::FIELD_NAMES);
    }

    #[test]
    fn manual_field_names_works() {
        let expected = vec![
            "bar".to_string(),
            "id".to_string(),
            "name".to_string(),
            "code".to_string(),
        ];

        let f = Foo {
            bar: Default::default(),
            id: Default::default(),
            name: Default::default(),
            code: Default::default(),
        };

        assert_eq!(expected, FooManual::FIELD_NAMES);
    }
}
