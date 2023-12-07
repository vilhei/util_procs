use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, Stmt};

/// Measures exection time of a function and prints it out <br> <br>
/// **IMPORTANT** Only supports function that have void return type <br>
/// # Examples
///
/// ## Default
/// ```
/// #[print_exec_time]
/// fn add(a:i32,b:i32) -> i32 {
///     a + b
/// }
///
/// fn main() {
///     let a = add(2,4);
///     println!("{a}");
///     // 6
///     // <execution time>
/// }
/// ```
/// ## With custom prefix message
///
/// ```
/// #[print_exec_time(Add exec time : )]
/// fn add(a:i32,b:i32) -> i32 {
///     a + b
/// }
///
/// fn main() {
///     let a = add(2,4);
///     println!("{a}");
///     // 6
///     // Add exec time : <execution time>
/// }
/// ```
///
#[proc_macro_attribute]
pub fn print_exec_time(msg: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item.clone()).unwrap();
    let time_now_statement: Stmt =
        syn::parse(quote!(let start = std::time::Instant::now();).into()).unwrap();
    let prefix = msg.to_string();
    let print_elapsed_statement: Stmt =
        syn::parse(quote!(println!("{} {:.3?}", #prefix, start.elapsed());).into()).unwrap();
    ast.block.stmts.insert(0, time_now_statement);
    ast.block.stmts.push(print_elapsed_statement);

    ast.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
