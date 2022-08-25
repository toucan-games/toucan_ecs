use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::{FetchError, FetchResult};
use crate::marker::Res;
use crate::resource::Resource;
use crate::system::fetch::Fetch;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchResourceRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceRead<R>
where
    R: Resource,
{
    type Item = Res<'data, R>;

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let resource = data.move_resource_ref().ok_or(FetchError)?;
        let resource = Res(resource);
        Ok(resource)
    }
}
