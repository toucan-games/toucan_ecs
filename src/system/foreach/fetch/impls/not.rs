use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchNot;
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(Self::new(data.into()))
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        FetchNot::fetch(self, entity)
    }
}
