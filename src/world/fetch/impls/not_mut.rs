use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchNotMut<'data, C>
where
    C: Component,
{
    storage: Option<&'data StorageImpl<C>>,
}

impl<'data, C> FetchMut<'data> for FetchNotMut<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    // noinspection DuplicatedCode
    unsafe fn new(data: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let storage = data.components().get_storage();
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn fetch_mut(&mut self, entity: Entity) -> Result<Self::Item, FetchError> {
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
