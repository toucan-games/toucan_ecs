use std::collections::HashSet;

use crate::component::ComponentTypeId;
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::{World, WorldData};

impl<'data> Fetch<'data> for () {
    type Item = ();

    fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

    fn register(_: &mut World) {}

    fn new(_: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        Ok(())
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
        Ok(())
    }

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
                Ok(Some((entity, ())))
            }
        }
    }
}
