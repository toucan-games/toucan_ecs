use super::Fetch;

impl<'data> Fetch<'data> for () {
    type Item = ();
}
