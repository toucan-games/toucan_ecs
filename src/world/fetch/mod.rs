pub use error::FetchError;
pub use impls::*;

use crate::Entity;

use super::{WorldData, WorldDataMut};

mod error;
mod impls;
mod tuple;

pub trait Fetch<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    fn new(data: WorldData<'data>) -> Result<Self, FetchError>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError>;
}

pub trait FetchMut<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    /// # Safety
    ///
    /// This function should be called if and only if mutability soundness was checked.
    unsafe fn new(data: WorldDataMut<'data>) -> Result<Self, FetchError>;

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError>;
}
