use crate::entity::Entity;
use crate::system::fetch::Fetch;

pub struct FetchEntity;

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;
}
