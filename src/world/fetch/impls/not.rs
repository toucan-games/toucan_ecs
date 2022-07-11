use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchNot;
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    fn new(data: WorldData<'data>) -> FetchResult<Self> {
        Ok(Self::new(data))
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        self.fetch(entity)
    }
}

impl<'data, C> FetchMut<'data> for FetchNot<'data, C>
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
