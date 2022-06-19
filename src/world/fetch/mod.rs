pub use error::FetchError;

use crate::{Entity, World};

mod error;
mod tuple;

pub trait Fetch<'data>: TryFrom<&'data World, Error = FetchError> {
    type Item: Send + Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError>;
}

pub trait FetchMut<'data>: TryFrom<&'data mut World, Error = FetchError> {
    type Item: Send + Sync + 'data;

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError>;
}
