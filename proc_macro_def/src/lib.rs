mod parser;
mod processor;

use quote::quote;
use proc_macro::TokenStream;
use syn::{braced, Result, Token, Field};

// Implement a function-like macro which takes struct definitions
// as input using proc macro2
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        let ast = syn::parse2::<syn::DeriveInput>(input).unwrap();
        let name = &ast.ident;
        let gen = quote! {
            impl #name {
                fn hello_world(&self) {
                    println!("Hello, world! My name is {}", stringify!(#name));
                }
            }
        };
        gen
    };

    proc_macro::TokenStream::from(output)
}