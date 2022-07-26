pub(super) use sealed::Sealed;

use crate::system::foreach::fetch::Fetch;

mod impls;
mod tuple;

#[cfg(doc)]
#[doc(hidden)]
pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

#[cfg(not(doc))]
pub(crate) type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

/// Special type of query which can be queried
/// by the [system](crate::system::foreach::ForeachSystem) **multiple** times.
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: Sealed + From<QueryItem<'data, Self>> + 'data {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}

mod sealed {
    pub trait Sealed {}
}
