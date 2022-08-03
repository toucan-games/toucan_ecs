use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result};

// todo make it hygienic (use full paths to the structs and traits)
pub fn fetch(input: Vec<Ident>) -> Result<TokenStream> {
    let optimized_strategy = input.iter().enumerate().map(|(idx, ident)| {
        let fetch_entities = input
            .iter()
            .enumerate()
            .filter_map(|(idx_inner, ident)| (idx_inner != idx).then_some(ident));
        quote! {
            if #ident.is_iter() {
                return match #ident.fetch_iter(strategy)? {
                    None => Ok(None),
                    Some((entity, #ident)) => {
                        #(
                            let #fetch_entities = #fetch_entities.fetch_entity(entity)?;
                        )*
                        let item = (#( #input, )*);
                        Ok(Some((entity, item)))
                    },
                }
            }
        }
    });
    let output = quote! {
        impl<'data, #( #input, )*> Fetch<'data> for (#( #input, )*)
        where #( #input: Fetch<'data>, )*
        {
            type Item = (#( #input::Item, )*);

            fn push_fetch_data(world: &WorldRefs<'data>, fetch_data: &mut Vec<FetchData>) {
                #( #input::push_fetch_data(world, fetch_data); )*
            }

            #[allow(non_snake_case)]
            fn new(data: &mut WorldRefs<'data>, mut optimal: Option<ComponentTypeId>) -> FetchResult<Self> {
                #(
                let #input = #input::new(data, optimal)?;
                // make sure that iterable fetch is unique
                if #input.is_iter() {
                    optimal = None;
                }
                )*
                Ok((#( #input, )*))
            }

            #[allow(non_snake_case)]
            fn is_iter(&self) -> bool {
                let (#( #input, )*) = self;
                #( #input.is_iter() )||*
            }

            #[allow(non_snake_case)]
            fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
                let (#( #input, )*) = self;
                #( let #input = #input.fetch_entity(entity)?; )*
                Ok((#( #input, )*))
            }

            #[allow(non_snake_case)]
            fn fetch_iter(
                &'data mut self,
                strategy: FetchStrategy<'data>,
            ) -> FetchResult<Option<(Entity, Self::Item)>> {
                let (#( #input, )*) = self;
                match strategy {
                    FetchStrategy::All(entities) => {
                        let entity = match entities.next() {
                            None => return Ok(None),
                            Some(entity) => entity,
                        };
                        #( let #input = #input.fetch_entity(entity)?; )*
                        let item = (#( #input, )*);
                        Ok(Some((entity, item)))
                    }
                    FetchStrategy::Optimized => {
                        #( #optimized_strategy )*
                        Err(FetchError)
                    }
                }
            }
        }
    };
    Ok(output)
}
