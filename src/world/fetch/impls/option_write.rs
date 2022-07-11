use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::{FetchOptionWrite, FetchResourceOptionWrite};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchMut, WorldDataMut};

impl<'data, C> FetchMut<'data> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(Self::new(data))
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        self.fetch_mut(entity)
    }
}

cfg_resource! {
    impl<'data, R> FetchMut<'data> for FetchResourceOptionWrite<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::ResourceMut<'data, R>>;

        unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
            Ok(Self::new(data))
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch_mut(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            Ok(self.fetch_mut())
        }
    }
}
