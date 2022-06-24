use std::any::TypeId;

use multimap::MultiMap;

use crate::world::query::{Query, QueryMut, QuerySealed, SoundnessCheck};
use crate::Entity;

use super::fetch::FetchEntity;

impl QuerySealed for Entity {}

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl SoundnessCheck for Entity {
    const MUTABLE: bool = false;

    fn extend_before_check(_: &mut MultiMap<TypeId, bool>) {}
}

impl<'data> QueryMut<'data> for Entity {
    type Fetch = FetchEntity;
}
