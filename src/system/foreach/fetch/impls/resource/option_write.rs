use crate::component::ComponentTypeId;
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::marker::ResMut;
use crate::resource::Resource;
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchResourceOptionWrite<'data, R>
where
    R: Resource,
{
    resource: Option<&'data mut R>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionWrite<'data, R>
where
    R: Resource,
{
    type Item = Option<ResMut<'data, R>>;

    fn push_fetch_data(_: &WorldRefs<'data>, _: &mut Vec<FetchData>) {}

    fn new(data: &mut WorldRefs<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let resource = data.move_resource_mut();
        Ok(Self { resource })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
        let resource = self.resource.as_deref_mut().map(ResMut);
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
