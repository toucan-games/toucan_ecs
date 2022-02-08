use crate::entity::Entity;
use crate::world::{SharedViewable, Viewable};

use super::fetch::FetchEntity;

impl<'data> Viewable<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data> SharedViewable<'data> for Entity {}
