use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use quote::ToTokens;
use syn::Generics;
use syn::{Data, Ident};

pub struct IntoResponse {
    pub ident: Ident,
    pub data: Data,
    pub generics: Generics,
}

impl ToTokens for IntoResponse {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        let responses = match &self.data {
            Data::Enum(data_enum) => data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;

                match &variant.fields {
                    syn::Fields::Named(_) => abort!(
                        variant.ident,
                        "`IntoResponse` does not support named enum fields"
                    ),
                    syn::Fields::Unnamed(fields) => {
                        match fields.unnamed.iter().collect::<Vec<_>>().as_slice() {
                            [] => {
                                quote! {
                                    #ident::#variant_ident () => IntoResponse::into_response(())
                                }
                            }
                            [field] => {
                                let is_json = field.attrs.iter().any(|attribute| {
                                    attribute.path().get_ident().unwrap() == "json"
                                });

                                let get_body = if is_json {
                                    quote! {axum::Json(body)}
                                } else {
                                    quote! {body}
                                };

                                quote! {
                                    #ident::#variant_ident (body) => IntoResponse::into_response(#get_body)
                                }
                            }
                            _ => abort!(
                                variant.ident,
                                "`IntoResponse` requires at most one unnamed field"
                            ),
                        }
                    }
                    syn::Fields::Unit => quote! {
                        #ident::#variant_ident => IntoResponse::into_response(())
                    },
                }
            }),
            Data::Struct(_) => abort!(ident, "`IntoResponse` does not support `Struct` types"),
            Data::Union(_) => abort!(ident, "`IntoResponse` does not support `Union` types"),
        };

        tokens.extend(quote! {
            impl IntoResponse for #ident {
                fn into_response(self) -> axum::response::Response {
                    match self {
                        #(#responses),*
                    }
                }
            }
        });
    }
}
