use std::collections::HashSet;

use atomicell::Ref;

use crate::component::storage::Storage;
use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::{World, WorldData};

#[repr(transparent)]
pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<Ref<'data, C::Storage>>,
}

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

    fn register(world: &mut World) {
        world.register::<C>();
    }

    fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let storage = data.components().get_storage_guarded::<C>();
        Ok(Self { storage })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let storage = self.storage.as_ref();
        let item = storage.and_then(|it| it.get(entity));
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

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        resource: Option<Ref<'data, R>>,
    }

    impl<'data, R> Fetch<'data> for FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::Resource<'data, R>>;

        fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

        fn register(_: &mut World) {}

        fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
            let resource = data.resources().get_guarded();
            Ok(Self { resource })
        }

        fn is_iter(&self) -> bool {
            false
        }

        fn fetch_entity(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = self.resource.as_deref().map(marker::Resource::new);
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
                },
            }
        }
    }
}
