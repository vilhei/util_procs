pub use proc_macros::*;
pub use util_procs_internal::*;

#[cfg(test)]
mod tests {
    use proc_macros::FieldNames;
    use util_procs_internal::FieldNames;

    #[derive(Default, FieldNames)]
    struct Foo {
        bar: String,
        id: i64,
        name: String,
        code: u32,
    }

    #[derive(Default)]
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
        let value = Foo::FIELD_NAMES;

        assert_eq!(expected, value);
    }

    #[test]
    fn manual_field_names_works() {
        let expected = vec![
            "bar".to_string(),
            "id".to_string(),
            "name".to_string(),
            "code".to_string(),
        ];
        let value = FooManual::FIELD_NAMES;

        assert_eq!(expected, value);
    }
}
