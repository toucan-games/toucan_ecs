use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::component::Component;

/// Marker zero-sized type for retrieving entities without component of generic type.
/// It must be used in query to be retrieved.
#[derive(Clone, Copy)]
pub struct Not<C>(PhantomData<C>)
where
    C: Component;

impl<C> Default for Not<C>
where
    C: Component,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C> Debug for Not<C>
where
    C: Component,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = core::any::type_name::<C>();
        write!(f, "Not<{}>", name)
    }
}

impl<C> PartialEq for Not<C>
where
    C: Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<C> Eq for Not<C> where C: Component {}

impl<C> PartialOrd for Not<C>
where
    C: Component,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<C> Ord for Not<C>
where
    C: Component,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<C> Hash for Not<C>
where
    C: Component,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
