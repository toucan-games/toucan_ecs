use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, StorageHolder};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::world::WorldData;

#[repr(transparent)]
pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> FetchNot<'data, C>
where
    C: Component,
{
    pub fn new(data: WorldData<'data>) -> Self {
        let storage = data.components().get_storage();
        Self { storage }
    }

    pub fn fetch(&self, entity: Entity) -> FetchResult<Not<C>> {
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
