use std::marker::PhantomData;

use crate::mutability_check::MutabilityChecked;
use crate::system::query::Query;

pub struct CheckedQuery<'data, Q>
where
    Q: Query<'data>,
{
    _checked: MutabilityChecked<Q>,
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> CheckedQuery<'data, Q>
where
    Q: Query<'data>,
{
    pub(in crate::system) fn new() -> Self {
        Self {
            _ph: PhantomData,
            _checked: MutabilityChecked::default(),
        }
    }
}
