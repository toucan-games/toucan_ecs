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

impl<'data, C> TryFrom<WorldDataMut<'data>> for FetchNotMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: WorldDataMut<'data>) -> Result<Self, Self::Error> {
        // SAFETY: must be checked by the caller.
        let storage = unsafe { world.components() }.get_storage();
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchNotMut<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    // noinspection DuplicatedCode
    unsafe fn fetch_mut(&mut self, entity: Entity) -> Result<Self::Item, FetchError> {
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
