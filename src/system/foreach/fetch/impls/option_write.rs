use crate::component::storage::Storage;
use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchOptionWrite<'data, C>
where
    C: Component,
{
    storage: Option<&'data mut C::Storage>,
}

impl<'data, C> Fetch<'data> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    fn push_fetch_data(_: &WorldRefs<'data>, _: &mut Vec<FetchData>) {}

    fn new(data: &mut WorldRefs<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let storage = data.move_storage_ref_mut::<C>();
        Ok(Self { storage })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let storage = self.storage.as_deref_mut();
        let item = storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
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
