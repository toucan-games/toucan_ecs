//! Provides types which can be queried by the [system](crate::system::System).
//!
//! Such types are:
//! - [`Entity`](crate::entity::Entity) handle;
//! - immutable and mutable [references] of [components];
//! - immutable and mutable [references] of [components] wrapped in [`Option`];
//! - immutable and mutable [references] of [resources] via [markers](crate::resource::marker)
//! (if enabled by `resource` feature);
//! - [`Not`](crate::component::marker::Not) marker type of components;
//! - world views, such as [`ViewOne`](crate::component::ViewOne),
//! [`ViewOneMut`](crate::component::ViewOneMut), [`View`](crate::world::View) and
//! [`ViewMut`](crate::world::ViewMut);
//! - and [tuples] of arity 12 or less of types listed above.
//!
//! [references]: prim@reference
//! [tuples]: prim@tuple
//! [components]: crate::component::Component
//! [resources]: crate::resource::Resource

pub(crate) use checked::CheckedQuery;

use crate::mutability_check::MutabilityCheck as QuerySealed;

use super::Fetch;

mod checked;
mod impls;
mod tuple;

#[cfg(doc)]
#[doc(hidden)]
pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

#[cfg(not(doc))]
type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

/// Type which can be queried by the [system](crate::system::System).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: 'data + QuerySealed + From<QueryItem<'data, Self>> {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}
