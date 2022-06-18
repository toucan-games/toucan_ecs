use std::any::TypeId;

use multimap::MultiMap;

use crate::world::{Query, QueryMut, SoundnessChecked};
use crate::Entity;

use super::fetch::FetchEntity;

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl SoundnessChecked for Entity {
    const MUTABLE: bool = false;

    fn extend_before_check(_: &mut MultiMap<TypeId, bool>) {}
}

impl<'data> QueryMut<'data> for Entity {
    type Fetch = FetchEntity;
}
