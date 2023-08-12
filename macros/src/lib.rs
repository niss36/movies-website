use into_response::IntoResponse;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

mod into_response;

#[proc_macro_error]
#[proc_macro_derive(IntoResponse, attributes(response, json))]
pub fn into_response(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

    IntoResponse {
        ident,
        data,
        generics,
    }
    .to_token_stream()
    .into()
}
