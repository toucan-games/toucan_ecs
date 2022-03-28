use crate::world::Query;
use crate::Entity;

use super::fetch::FetchEntity;

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}
