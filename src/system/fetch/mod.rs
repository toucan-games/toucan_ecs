pub use impls::*;

use crate::World;

mod impls;
mod tuple;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;

    unsafe fn fetch(world: &'data mut World) -> Self::Item;
}
