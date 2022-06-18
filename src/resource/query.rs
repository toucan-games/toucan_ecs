use std::any::TypeId;

use multimap::MultiMap;

use crate::world::{Query, QueryMut, SoundnessChecked};

use super::fetch::{FetchRead, FetchReadMut, FetchWriteMut};
use super::marker::Resource as ResourceMarker;
use super::Resource;

impl<'data, R> Query<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> SoundnessChecked for ResourceMarker<&'data R>
where
    R: Resource,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(TypeId::of::<R>(), Self::MUTABLE)
    }
}

impl<'data, R> QueryMut<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchReadMut<'data, R>;
}

impl<'data, R> SoundnessChecked for ResourceMarker<&'data mut R>
where
    R: Resource,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(TypeId::of::<R>(), Self::MUTABLE)
    }
}

impl<'data, R> QueryMut<'data> for ResourceMarker<&'data mut R>
where
    R: Resource,
{
    type Fetch = FetchWriteMut<'data, R>;
}
