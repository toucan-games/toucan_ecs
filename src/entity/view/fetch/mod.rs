pub use read::FetchRead;
pub use write::FetchWrite;

use crate::{Entity, Registry};

mod read;
mod tuple;
mod write;

pub trait Fetch<'data>: TryFrom<&'data Registry, Error = ()> {
    type Item: Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()>;
}
