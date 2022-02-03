pub use option_read::FetchOptionRead;
pub use option_write::FetchOptionWrite;
pub use read::FetchRead;
pub use write::FetchWrite;

use crate::{Entity, Registry};

mod option_read;
mod option_write;
mod read;
mod tuple;
mod write;

pub trait Fetch<'data>: TryFrom<&'data Registry, Error = ()> {
    type Item: Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()>;
}
