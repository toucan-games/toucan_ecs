//! Contains macros for [`toucan_ecs`] crate.
//!
//! [`toucan_ecs`]: https://crates.io/crates/toucan_ecs

use proc_macro::TokenStream;

use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Error, Ident, Token};

mod derive;
mod proc;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::component(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn fetch_tuple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input with Punctuated::<Ident, Token![,]>::parse_terminated)
        .into_iter()
        .collect();
    proc::fetch(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
