use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::marker::ResMut;
use crate::resource::Resource;
use crate::system::fetch::Fetch;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchResourceOptionWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionWrite<R>
where
    R: Resource,
{
    type Item = Option<ResMut<'data, R>>;

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let resource = data.move_resource_mut();
        let resource = resource.map(ResMut);
        Ok(resource)
    }
}
