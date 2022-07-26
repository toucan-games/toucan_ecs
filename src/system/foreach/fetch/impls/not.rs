use std::collections::HashSet;

use atomicell::Ref;

use crate::component::marker::Not;
use crate::component::storage::Storage;
use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldData;

#[repr(transparent)]
pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<Ref<'data, C::Storage>>,
}

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

    fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let storage = data.components().get_storage_guarded::<C>();
        Ok(Self { storage })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        match self.storage.as_ref() {
            None => Ok(Not::new()),
            Some(storage) => match storage.attached(entity) {
                false => Ok(Not::new()),
                true => Err(FetchError),
            },
        }
    }

    // noinspection DuplicatedCode
    fn fetch_iter(
        &'data mut self,
        strategy: FetchStrategy<'data>,
    ) -> FetchResult<Option<(Entity, Self::Item)>> {
        match strategy {
            FetchStrategy::Optimized => Err(FetchError),
            FetchStrategy::All(entities) => {
                let entity = match entities.next() {
                    None => return Ok(None),
                    Some(entity) => entity,
                };
                let item = self.fetch_entity(entity)?;
                Ok(Some((entity, item)))
            }
        }
    }
}
