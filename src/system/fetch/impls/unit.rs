use crate::system::fetch::Fetch;

impl<'data> Fetch<'data> for () {
    type Item = ();
}
