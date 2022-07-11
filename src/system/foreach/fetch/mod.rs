pub use impls::*;

use crate::entity::Entity;
use crate::error::FetchResult;
use crate::world::WorldDataMut;

mod impls;
mod tuple;

pub trait Fetch<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    /// # Safety
    ///
    /// This function should be called if and only if mutability soundness was checked.
    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self>;

    // fixme move to associated type when GATs are stabilized
    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>>;

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item>;
}
