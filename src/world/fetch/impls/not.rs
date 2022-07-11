use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, StorageHolder};
use crate::error::{FetchError, FetchResult};
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};
use crate::Entity;

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

    fn new(data: WorldData<'data>) -> FetchResult<Self> {
        let storage = data.components().get_storage();
        Ok(Self { storage })
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
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

impl<'data, C> FetchMut<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = <Self as Fetch<'data>>::Item;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Fetch::new(data.into())
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        Fetch::entities(self)
    }

    fn fetch_mut(&mut self, entity: Entity) -> FetchResult<Self::Item> {
        Fetch::fetch(self, entity)
    }
}
