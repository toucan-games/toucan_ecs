use std::any::TypeId;
use std::marker::PhantomData;

use multimap::MultiMap;

pub use soundness_check::{check_soundness, SoundnessCheck, SoundnessChecked};

use super::fetch::{Fetch, FetchMut};

mod soundness_check;
mod tuple;

pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Query<'data>: 'data {
    type Fetch: Fetch<'data>;
}

pub type QueryMutItem<'data, Q> = <<Q as QueryMut<'data>>::Fetch as FetchMut<'data>>::Item;

pub trait QueryMut<'data>: 'data + SoundnessCheck {
    type Fetch: FetchMut<'data>;
}

pub struct CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    _checked: SoundnessChecked<Q>,
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(super) fn new() -> Self {
        Self {
            _ph: PhantomData,
            _checked: Default::default(),
        }
    }
}
