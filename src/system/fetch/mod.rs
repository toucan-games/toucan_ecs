pub use impls::*;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::world::WorldRefs;

mod impls;
mod tuple;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;

    fn fetch(entities: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item>;
}
