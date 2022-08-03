use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::WorldRefs;

impl<'data> Fetch<'data> for () {
    type Item = ();

    fn fetch(_: &Iter<'data>, _: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        Ok(())
    }
}
