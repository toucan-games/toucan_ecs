use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::{FetchError, FetchResult};
use crate::marker::ResMut;
use crate::resource::Resource;
use crate::system::fetch::Fetch;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchResourceWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceWrite<R>
where
    R: Resource,
{
    type Item = ResMut<'data, R>;

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let resource = data.move_resource_mut().ok_or(FetchError)?;
        let resource = ResMut(resource);
        Ok(resource)
    }
}
