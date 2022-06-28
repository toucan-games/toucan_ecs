pub use impls::*;

use crate::error::FetchResult;
use crate::Entity;

use super::{WorldData, WorldDataMut};

mod impls;
mod tuple;

pub trait Fetch<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    fn new(data: WorldData<'data>) -> FetchResult<Self>;

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item>;
}

pub trait FetchMut<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    /// # Safety
    ///
    /// This function should be called if and only if mutability soundness was checked.
    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self>;

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item>;
}
