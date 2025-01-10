use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match builder::builder(ast) {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
