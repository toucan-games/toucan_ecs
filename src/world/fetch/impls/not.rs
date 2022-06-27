use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, Storage, StorageImpl};
use crate::world::{Fetch, FetchError, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<&'data StorageImpl<C>>,
}

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    fn new(data: WorldData<'data>) -> Result<Self, FetchError> {
        let storage = data.components().get_storage();
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
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
