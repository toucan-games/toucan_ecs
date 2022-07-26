use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::World;

impl<'data> Fetch<'data> for () {
    type Item = ();

    fn fetch(_: &'data World) -> FetchResult<Self::Item> {
        Ok(())
    }
}
