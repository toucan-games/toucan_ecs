use std::collections::HashSet;

use atomicell::RefMut;

use crate::component::{ComponentTypeId, Registry};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldData;

#[repr(transparent)]
pub struct FetchResourceOptionWrite<'data, R>
where
    R: Resource,
{
    resource: Option<RefMut<'data, R>>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionWrite<'data, R>
where
    R: Resource,
{
    type Item = Option<marker::ResourceMut<'data, R>>;

    fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

    fn register(_: &mut Registry) {}

    fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let resource = data.resources().get_mut_guarded();
        Ok(Self { resource })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
        let resource = self.resource.as_deref_mut().map(marker::ResourceMut::new);
        Ok(resource)
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
