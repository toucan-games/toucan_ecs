use crate::component::Component;
use crate::error::{FetchError, FetchResult};
use crate::fetch::{FetchResourceWrite, FetchWrite};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;
use crate::Entity;

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Self::new(data).ok_or(FetchError)
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.entities()?;
        let iter = Box::new(iter);
        Some(iter)
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        self.fetch_mut(entity).ok_or(FetchError)
    }
}

cfg_resource! {
    impl<'data, R> Fetch<'data> for FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        type Item = marker::ResourceMut<'data, R>;

        unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
            Self::new(data).ok_or(FetchError)
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            Ok(self.fetch_mut())
        }
    }
}
