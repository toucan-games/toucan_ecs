use crate::mutability_check::MutabilityCheck as Sealed;

use super::fetch::Fetch;

mod impls;
mod tuple;

#[cfg(doc)]
#[doc(hidden)]
pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

#[cfg(not(doc))]
type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

/// Special type of query which can be queried
/// by the [system](crate::system::foreach::ForeachSystem) **multiple** times.
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: 'data + Sealed + From<QueryItem<'data, Self>> {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}
