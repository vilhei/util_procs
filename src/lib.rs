pub use proc_macros::*;
pub use util_procs_internal::*;

#[cfg(test)]
mod tests {
    use proc_macros::FieldNames;
    use util_procs_internal::FieldNames;

    #[allow(dead_code)]
    #[derive(FieldNames)]
    struct Foo {
        bar: String,
        id: i64,
        name: String,
        code: u32,
    }

    #[allow(dead_code)]
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

        assert_eq!(expected, FooManual::FIELD_NAMES);
    }

    #[allow(dead_code)]
    #[derive(FieldNames)]
    enum Bar {
        Ok,
        Error,
        Fail,
        Crash,
    }

    #[allow(dead_code)]
    enum BarManual {
        Ok,
        Error,
        Fail,
        Crash,
    }

    impl FieldNames for BarManual {
        const FIELD_NAMES: &'static [&'static str] = &["Ok", "Error", "Fail", "Crash"];
    }

    #[test]
    fn enuam_variants_names() {
        let expected = vec![
            "Ok".to_string(),
            "Error".to_string(),
            "Fail".to_string(),
            "Crash".to_string(),
        ];

        assert_eq!(&expected, Bar::FIELD_NAMES);
    }

    #[test]
    fn manual_enuam_variants_names() {
        let expected = vec![
            "Ok".to_string(),
            "Error".to_string(),
            "Fail".to_string(),
            "Crash".to_string(),
        ];

        assert_eq!(expected, BarManual::FIELD_NAMES);
    }
}
