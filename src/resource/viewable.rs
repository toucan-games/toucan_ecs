use crate::world::Viewable;

use super::fetch::FetchRead;
use super::marker::Resource as ResourceMarker;
use super::Resource;

impl<'data, R> Viewable<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}
