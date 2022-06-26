//! Provides types which can be queried by views in the [world](crate::world::World).
//!
//! Such types are:
//! - [`Entity`](crate::entity::Entity) handle;
//! - immutable and mutable [references] of [components];
//! - immutable and mutable [references] of [components] wrapped in [`Option`];
//! - immutable and mutable [references] of [resources] via [markers](crate::resource::marker)
//! (if enabled by `resource` feature);
//! - [`Not`](crate::component::marker::Not) marker type of components;
//! - and [tuples] of arity 12 or less of types listed above.
//!
//! [references]: prim@reference
//! [tuples]: prim@tuple
//! [components]: crate::component::Component
//! [resources]: crate::resource::Resource

pub(crate) use checked::CheckedQuery;

use crate::mutability_check::{MutabilityCheck as QuerySealed, MutabilityCheck as QueryMutSealed};

use super::fetch::{Fetch, FetchMut};

mod checked;
mod tuple;

#[cfg(doc)]
#[doc(hidden)]
pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

#[cfg(not(doc))]
type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

/// Type which can be queried by **shared** [view](crate::world::View)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: 'data + QuerySealed + From<QueryItem<'data, Self>> {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}

#[cfg(doc)]
#[doc(hidden)]
pub type QueryMutItem<'data, Q> = <<Q as QueryMut<'data>>::Fetch as FetchMut<'data>>::Item;

#[cfg(not(doc))]
type QueryMutItem<'data, Q> = <<Q as QueryMut<'data>>::Fetch as FetchMut<'data>>::Item;

/// Type which can be queried by **mutable** [view](crate::world::ViewMut)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait QueryMut<'data>: 'data + QueryMutSealed + From<QueryMutItem<'data, Self>> {
    #[doc(hidden)]
    type Fetch: FetchMut<'data>;
}
