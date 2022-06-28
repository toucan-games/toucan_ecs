use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::World;

impl<'data> Fetch<'data> for () {
    type Item = ();

    unsafe fn fetch(_: *mut World) -> FetchResult<Self::Item> {
        Ok(())
    }
}
