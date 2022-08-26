use std::marker::PhantomData;

use crate::mutability_check::{CheckMap, MutabilityCheck};

pub struct MutabilityChecked<T>(PhantomData<T>)
where
    T: MutabilityCheck;

impl<T> Default for MutabilityChecked<T>
where
    T: MutabilityCheck,
{
    fn default() -> Self {
        check::<T>();
        Self(PhantomData)
    }
}

fn check<T>()
where
    T: MutabilityCheck,
{
    let mut check_map = CheckMap::with_capacity_and_hasher(T::LENGTH, Default::default());
    T::check(&mut check_map);
}
