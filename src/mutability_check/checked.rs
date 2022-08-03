use std::marker::PhantomData;

use super::{MultiMap, MutabilityCheck};

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
    let mut multimap = MultiMap::new();
    T::extend_before_check(&mut multimap);

    for (_, vec) in multimap {
        // all type occurrences are immutable, this is sound
        if vec.iter().all(|&it| !it) {
            continue;
        }
        // exactly one mutable type occurrence, this is sound too
        if vec.len() == 1 {
            continue;
        }
        // one mutable borrow with some immutable ones, this is unsound
        if vec.iter().filter(|&&it| it).count() == 1 {
            panic!("mutable borrow occurs while other immutable occurrences was found")
        }
        // multiple mutable borrows, this is unsound
        panic!("multiple mutable borrows are not allowed")
    }
}
