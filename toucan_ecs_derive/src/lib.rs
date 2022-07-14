use proc_macro::TokenStream;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(component), forward_attrs(allow, doc, cfg))]
struct Options {
    #[darling(multiple)]
    storage: Vec<Path>,
}

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let Options { storage } = Options::from_derive_input(&input).expect("Wrong storage format");
    let DeriveInput { ident, .. } = input;

    let storage = match &storage[..] {
        &[] => quote! { DefaultStorage<Self> },
        storage => quote! { #( #storage )* },
    };
    let output = quote! {
        impl Component for #ident {
            type Storage = #storage;
        }
    };
    output.into()
}
