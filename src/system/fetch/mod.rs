pub use impls::*;

use crate::error::FetchResult;
use crate::world::World;

mod impls;
mod tuple;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;

    fn fetch(world: &'data World) -> FetchResult<Self::Item>;
}
