use std::collections::HashSet;

use atomicell::RefMut;

use crate::component::storage::Storage;
use crate::component::{Component, ComponentTypeId, Registry};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldData;

#[repr(transparent)]
pub struct FetchOptionWrite<'data, C>
where
    C: Component,
{
    storage: Option<RefMut<'data, C::Storage>>,
}

impl<'data, C> Fetch<'data> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

    fn register(registry: &mut Registry) {
        registry.register::<C>();
    }

    fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
        let storage = data.components().get_storage_mut_guarded::<C>();
        Ok(Self { storage })
    }

    fn is_iter(&self) -> bool {
        false
    }

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let storage = self.storage.as_mut();
        let item = storage.and_then(|it| it.get_mut(entity));
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
}
