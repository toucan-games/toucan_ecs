use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::marker::Res;
use crate::resource::Resource;
use crate::system::fetch::Fetch;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchResourceOptionRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionRead<R>
where
    R: Resource,
{
    type Item = Option<Res<'data, R>>;

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let resource = data.move_resource_ref();
        let resource = resource.map(Res);
        Ok(resource)
    }
}
