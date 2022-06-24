use std::any::TypeId;

use multimap::MultiMap;

use crate::world::query::{Query, QueryMut, QuerySealed, SoundnessCheck};

use super::fetch::{FetchRead, FetchReadMut, FetchWriteMut};
use super::marker;
use super::Resource;

impl<'data, R> QuerySealed for marker::Resource<'data, R> where R: Resource {}

impl<'data, R> Query<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> SoundnessCheck for marker::Resource<'data, R>
where
    R: Resource,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(TypeId::of::<R>(), Self::MUTABLE)
    }
}

impl<'data, R> QueryMut<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchReadMut<'data, R>;
}

impl<'data, R> SoundnessCheck for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(TypeId::of::<R>(), Self::MUTABLE)
    }
}

impl<'data, R> QueryMut<'data> for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchWriteMut<'data, R>;
}
