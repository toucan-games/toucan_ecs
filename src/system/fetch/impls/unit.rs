use crate::system::fetch::Fetch;
use crate::World;

impl<'data> Fetch<'data> for () {
    type Item = ();

    unsafe fn fetch(_: &'data mut World) -> Self::Item {}
}
