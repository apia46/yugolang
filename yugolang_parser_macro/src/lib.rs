use proc_macro::TokenStream;
use syn::{LitStr, parse::{Parse, ParseStream}};
use quote::quote;
use quoter::ToTokens;

mod quoter;

struct Input(String);

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Input(input.parse::<LitStr>()?.value()))
    }
}

#[proc_macro]
pub fn parse(input:TokenStream) -> TokenStream {
    match syn::parse::<Input>(input) {
        Ok(Input(input)) => {
            match yugolang_parser::parse(&input) {
                Ok(scope) => {
                    scope.to_tokens().into()
                },
                Err(e) => {
                    let error = format!("{e:?}");
                    quote!{ ::core::compile_error!(#error) }.into()
                }
            }
        },
        Err(e) => e.to_compile_error().into(),
    }
}

