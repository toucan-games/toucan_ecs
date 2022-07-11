use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchOptionRead;
#[cfg(feature = "resource")]
use crate::fetch::FetchResourceOptionRead;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(Self::new(data.into()))
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(FetchOptionRead::fetch(self, entity))
    }
}

cfg_resource! {
    impl<'data, R> Fetch<'data> for FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::Resource<'data, R>>;

        unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
            Ok(Self::new(data.into()))
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            Ok(FetchResourceOptionRead::fetch(self))
        }
    }
}
