use crate::component::Component;
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
use crate::fetch::{FetchRead, FetchResourceRead};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::Fetch;
use crate::world::{FetchMut, WorldDataMut};

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Self::new(data.into()).ok_or(FetchError)
    }

    // noinspection DuplicatedCode
    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.entities()?;
        let iter = Box::new(iter);
        Some(iter)
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        self.fetch_mut(entity)
    }
}

cfg_resource! {
    impl<'data, R> Fetch<'data> for FetchResourceRead<'data, R>
    where
        R: Resource,
    {
        type Item = marker::Resource<'data, R>;

        unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
            Self::new(data.into()).ok_or(FetchError)
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            Ok(FetchResourceRead::fetch(self))
        }
    }
}
