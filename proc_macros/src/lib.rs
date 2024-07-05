use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::ParseStream, DeriveInput, ItemFn, Stmt};

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
    // println!("{:#?}", ast.block);
    let time_now_statement: Stmt = syn::parse(
        quote!(let print_exec_time_start
             = std::time::Instant::now();)
        .into(),
    )
    .unwrap();

    let mut return_indices = Vec::new();

    for (i, s) in ast.block.stmts.iter().enumerate() {
        match s {
            Stmt::Local(_) => (),
            Stmt::Item(_) => (),
            Stmt::Expr(a, _) => {
                if let syn::Expr::Return(_) = a {
                    dbg!(&s);
                    return_indices.push(i)
                }
            }
            Stmt::Macro(_) => (),
        }
    }

    let prefix = msg.to_string();
    let print_elapsed_statement: Stmt = syn::parse(
        quote!(println!("{} {:.3?}", #prefix, print_exec_time_start
        .elapsed());)
        .into(),
    )
    .unwrap();

    ast.block.stmts.insert(0, time_now_statement);
    println!("Indices : {:?}", &return_indices);
    for i in return_indices {
        ast.block.stmts.insert(i, print_elapsed_statement.clone());
    }
    // ast.block.stmts.push(print_elapsed_statement);

    ast.into_token_stream().into()
}

#[proc_macro_derive(FieldNames)]
pub fn field_names(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    match ast.data {
        syn::Data::Struct(s) => impl_field_names(&s, &ast.ident),
        syn::Data::Enum(e) => impl_enum_names(&e, &ast.ident),
        syn::Data::Union(_) => {
            let ty_span = ast.ident.span();

            syn::Error::new(ty_span, "Not implemented for unions")
                .to_compile_error()
                .into()
        }
    }
}

fn impl_field_names(input: &syn::DataStruct, struct_name: &syn::Ident) -> TokenStream {
    let fields: Vec<String> = match &input.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                f.ident
                    .as_ref()
                    .expect("Field did not have ident?")
                    .to_string()
            })
            .collect(),
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
    };

    quote! {
        impl FieldNames for #struct_name {
            const FIELD_NAMES: &'static [&'static str] = &[#(#fields),*];
        }
    }
    .into()
}

fn impl_enum_names(input: &syn::DataEnum, enum_name: &syn::Ident) -> TokenStream {
    let variants: Vec<String> = input.variants.iter().map(|v| v.ident.to_string()).collect();

    quote! {
        impl FieldNames for #enum_name {
            const FIELD_NAMES: &'static [&'static str] = &[#(#variants),*];
        }
    }
    .into()
}
