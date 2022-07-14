pub use impls::*;

use crate::error::FetchResult;
use crate::world::World;

mod impls;
mod tuple;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item>;
}
