use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, Stmt};

/// Measures exection time of a function and prints it out
///
///Mutates the function as follows with a wrapper
///```
/// let start = std::time::Instant::now();
/// // Here is your original function
/// let prefix = macro_input.to_string()
/// println!("{} {:.3?}", prefix, start.elapsed());
///```
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
