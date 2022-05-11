use crate::world::{Query, QueryMut};
use crate::Entity;

use super::fetch::FetchEntity;

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data> QueryMut<'data> for Entity {
    type Fetch = FetchEntity;
}
