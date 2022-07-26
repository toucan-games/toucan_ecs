use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Path, Result};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(component), forward_attrs(allow, doc, cfg))]
struct Options {
    #[darling(multiple)]
    storage: Vec<Path>,
}

pub fn component(input: DeriveInput) -> Result<TokenStream> {
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
