//! Contains derive macro for [`toucan_ecs`] crate.
//!
//! [`toucan_ecs`]: https://crates.io/crates/toucan_ecs

use proc_macro::TokenStream;

use darling::FromDeriveInput;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Path, Result};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(component), forward_attrs(allow, doc, cfg))]
struct Options {
    #[darling(multiple)]
    storage: Vec<Path>,
}

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput) -> Result<TokenStream2> {
    let Options { storage } = Options::from_derive_input(&input)?;
    let DeriveInput {
        ident, generics, ..
    } = input;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let trait_ident = quote! { toucan_ecs::component::Component };
    let storage = match &storage[..] {
        &[] => quote! { toucan_ecs::component::storage::DefaultStorage<Self> },
        storage => quote! { #( #storage )* },
    };
    let output = quote! {
        impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
            type Storage = #storage;
        }
    };
    Ok(output)
}
