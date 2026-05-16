use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ezconfig::Config for #name {
            fn load() -> ::std::result::Result<Self, ezconfig::Error> {
                ezconfig::__internal::load::<Self>()
            }
        }
    };

    TokenStream::from(expanded)
}