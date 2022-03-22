pub use error::FetchError;

use crate::{Entity, World};

mod error;
mod tuple;

pub trait Fetch<'data>: TryFrom<&'data World, Error = FetchError> {
    type Item: Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError>;
}
