use crate::entity::Entity;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::World;

pub struct FetchEntity;

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    unsafe fn fetch(_world: *mut World) -> FetchResult<Self::Item> {
        todo!()
    }
}
