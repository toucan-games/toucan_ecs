use crate::component::Component;
use crate::error::{FetchError, FetchResult};
use crate::fetch::FetchRead;
#[cfg(feature = "resource")]
use crate::fetch::FetchResourceRead;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};
use crate::Entity;

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    fn new(data: WorldData<'data>) -> FetchResult<Self> {
        Self::new(data).ok_or(FetchError)
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.entities()?;
        let iter = Box::new(iter);
        Some(iter)
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        self.fetch(entity).ok_or(FetchError)
    }
}

impl<'data, C> FetchMut<'data> for FetchRead<'data, C>
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

    fn fetch_mut(&mut self, entity: Entity) -> FetchResult<Self::Item> {
        Fetch::fetch(self, entity)
    }
}

cfg_resource! {
    impl<'data, R> Fetch<'data> for FetchResourceRead<'data, R>
    where
        R: Resource,
    {
        type Item = marker::Resource<'data, R>;

        fn new(data: WorldData<'data>) -> FetchResult<Self> {
            Self::new(data).ok_or(FetchError)
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&self, _: Entity) -> FetchResult<Self::Item> {
            Ok(self.fetch())
        }
    }

    impl<'data, R> FetchMut<'data> for FetchResourceRead<'data, R>
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
