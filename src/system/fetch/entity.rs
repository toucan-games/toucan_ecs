use crate::entity::Entity;

use super::Fetch;

pub struct FetchEntity;

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;
}
