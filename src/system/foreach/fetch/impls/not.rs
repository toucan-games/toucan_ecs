use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, StorageHolder};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;

#[repr(transparent)]
pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        let storage = data.components().get_storage();
        Ok(Self { storage })
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    // noinspection DuplicatedCode
    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        match self.storage {
            None => Ok(Not(PhantomData)),
            Some(storage) => {
                let component = storage.get(entity);
                match component {
                    None => Ok(Not(PhantomData)),
                    Some(_) => Err(FetchError),
                }
            }
        }
    }
}
