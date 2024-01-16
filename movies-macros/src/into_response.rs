use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Data, Error, ExprPath, Generics, Ident, Token, Variant};

struct VariantAttributes {
    response: VariantResponseArgs,
}

impl TryFrom<&Variant> for VariantAttributes {
    type Error = Error;

    fn try_from(value: &Variant) -> Result<Self, Self::Error> {
        let variant_response_attribute = value
            .attrs
            .iter()
            .find(|attribute| attribute.path().get_ident().unwrap() == "response")
            .ok_or(Error::new(
                value.ident.span(),
                "`IntoResponse` requires a `#[response(...)]` attribute on each variant",
            ))?;

        let variant_response_args: VariantResponseArgs = variant_response_attribute.parse_args()?;

        Ok(Self {
            response: variant_response_args,
        })
    }
}

struct VariantResponseArgs {
    status: TokenStream,
}

impl Parse for VariantResponseArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const MISSING_STATUS_ERROR: &str = "missing expected `status` attribute";

        let first_span = input.span();

        let status_ident = input.parse::<Ident>()?;

        if status_ident != "status" {
            return Err(Error::new(status_ident.span(), MISSING_STATUS_ERROR));
        }

        input.parse::<Token![=]>()?;
        let http_status_path = input.parse::<ExprPath>()?;

        let last_segment = http_status_path
            .path
            .segments
            .last()
            .expect("Expected at least one segment in http StatusCode");

        let status = last_segment.ident.to_token_stream();

        if status.is_empty() {
            return Err(Error::new(first_span, MISSING_STATUS_ERROR));
        }

        Ok(Self { status })
    }
}

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

                let variant_attributes: VariantAttributes = variant.try_into().unwrap_or_else(|error: Error| abort!(
                    error.span(),
                    error.to_string(),
                ));

                let status = variant_attributes.response.status;

                match &variant.fields {
                    syn::Fields::Named(_) => abort!(
                        variant.ident,
                        "`IntoResponse` does not support named enum fields"
                    ),
                    syn::Fields::Unnamed(fields) => {
                        match fields.unnamed.iter().collect::<Vec<_>>().as_slice() {
                            [] => {
                                quote! {
                                    #ident::#variant_ident () => IntoResponse::into_response(axum::http::StatusCode::#status)
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
                                    #ident::#variant_ident (body) => IntoResponse::into_response((axum::http::StatusCode::#status, #get_body))
                                }
                            }
                            _ => abort!(
                                variant.ident,
                                "`IntoResponse` requires at most one unnamed field"
                            ),
                        }
                    }
                    syn::Fields::Unit => quote! {
                        #ident::#variant_ident => IntoResponse::into_response(axum::http::StatusCode::#status)
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
