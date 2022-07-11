use crate::entity::Entity;
use crate::error::FetchResult;
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;

impl<'data> Fetch<'data> for () {
    type Item = ();

    unsafe fn new(_: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(())
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
        Ok(())
    }
}
