use crate::world::Viewable;
use crate::Entity;

use super::fetch::FetchEntity;

impl<'data> Viewable<'data> for Entity {
    type Fetch = FetchEntity;
}
