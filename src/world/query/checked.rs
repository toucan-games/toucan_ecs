use std::marker::PhantomData;

use crate::mutability_check::MutabilityChecked;

use super::QueryMut;

pub struct CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    _checked: MutabilityChecked<Q>,
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(in crate::world) fn new() -> Self {
        Self {
            _ph: PhantomData,
            _checked: MutabilityChecked::default(),
        }
    }
}
