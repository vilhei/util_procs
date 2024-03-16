/// Implements const [`FIELD_NAMES`] which should contain struct's field names as string slices.
/// You should in reality implement this by deriving this trait.
///
/// # Examples
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
pub trait FieldNames<const T: usize> {
    /// Contains the names of struct's fields as string slices
    const FIELD_NAMES: [&'static str; T];
}
