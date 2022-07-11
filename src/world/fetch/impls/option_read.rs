use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchOptionRead;
#[cfg(feature = "resource")]
use crate::fetch::FetchResourceOptionRead;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    fn new(data: WorldData<'data>) -> FetchResult<Self> {
        Ok(Self::new(data))
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(self.fetch(entity))
    }
}

impl<'data, C> FetchMut<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = <Self as Fetch<'data>>::Item;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Fetch::new(data.into())
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        Fetch::entities(self)
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        Fetch::fetch(self, entity)
    }
}

cfg_resource! {
    impl<'data, R> Fetch<'data> for FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::Resource<'data, R>>;

        fn new(data: WorldData<'data>) -> FetchResult<Self> {
            Ok(Self::new(data))
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&self, _: Entity) -> FetchResult<Self::Item> {
            Ok(self.fetch())
        }
    }

    impl<'data, R> FetchMut<'data> for FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        type Item = <Self as Fetch<'data>>::Item;

        unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
            Fetch::new(data.into())
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
            Fetch::entities(self)
        }

        fn fetch_mut(&mut self, entity: Entity) -> FetchResult<Self::Item> {
            Fetch::fetch(self, entity)
        }
    }
}
