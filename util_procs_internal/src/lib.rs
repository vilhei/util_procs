/// Implements const [FieldNames::FIELD_NAMES] which should contain struct's field names as string slices.
/// You should in reality implement this by deriving this trait.
///
/// Also supports enums by listing enum variants names. **NOTE** only returns variants name does not care if variant has data inside it
///
/// Also see [FieldNames::field_names] if you need to access the field names through an object instead of the type.
/// Has blanker implementation which just returns same reference as [FieldNames::FIELD_NAMES]
///
/// # Examples
///
/// ## 1 Derive
/// ```
/// #[derive(FieldNames)]
/// struct Foo {
///     bar: String,
///     id: i64,
///     name: String,
///     code: u32,
/// }
/// let expected = vec![
///     "bar".to_string(),
///     "id".to_string(),
///     "name".to_string(),
///     "code".to_string(),
/// ];
/// let value = Foo::FIELD_NAMES;
/// assert_eq!(expected, value);
/// ```
///
/// ## 2 Manual
///
/// ```
/// struct FooManual {
///     bar: String,
///     id: i64,
///     name: String,
///     code: u32,
/// }
///
/// impl FieldNames for FooManual {
///     const FIELD_NAMES: &'static [&'static str] = &["bar", "id", "name", "code"];
/// }
///
/// let expected = vec![
///     "bar".to_string(),
///     "id".to_string(),
///     "name".to_string(),
///     "code".to_string(),
/// ];
/// let value = FooManual::FIELD_NAMES;
///
/// assert_eq!(expected, value);
/// ```
///
pub trait FieldNames {
    /// Contains the names of struct's fields as string slices
    const FIELD_NAMES: &'static [&'static str];

    /// Convenience method so that field names can be accessed through an object
    /// ```
    /// #[derive(Default, FieldNames)]
    /// struct Foo {
    ///     bar: String,
    ///     id: i64,
    ///     name: String,
    ///     code: u32,
    /// }
    /// let expected = vec![
    ///     "bar".to_string(),
    ///     "id".to_string(),
    ///     "name".to_string(),
    ///     "code".to_string(),
    /// ];
    ///
    /// let f = Foo::default()
    /// let value = f.field_names();
    /// assert_eq!(expected, value);
    /// ```
    fn field_names(&self) -> &[&str] {
        Self::FIELD_NAMES
    }
}
