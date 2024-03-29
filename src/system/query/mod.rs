//! Provides types which can be queried by the [system](crate::system::System).
//!
//! Such types are:
//! - [`Entity`](crate::entity::Entity) handle;
//! - immutable and mutable [references] of [components];
//! - immutable and mutable [references] of [components] wrapped in [`Option`];
//! - immutable and mutable [references] of [resources] via [markers](crate::marker)
//! (if enabled by `resource` feature);
//! - [`Not`](crate::marker::Not) marker type of components;
//! - world views, such as [`ViewOne`][view_one], [`ViewOneMut`][view_one_mut],
//! [`View`][view] and [`ViewMut`][view_mut];
//! - and [tuples] of arity 12 or less of types listed above.
//!
//! [references]: prim@reference
//! [tuples]: prim@tuple
//! [components]: crate::component::Component
//! [resources]: crate::resource::Resource
//! [view]: crate::world::view::View
//! [view_mut]: crate::world::view::ViewMut
//! [view_one]: crate::world::view::ViewOne
//! [view_one_mut]: crate::world::view::ViewOneMut

pub(crate) use checked::CheckedQuery;

use crate::mutability_check::MutabilityCheck as Sealed;
use crate::system::fetch::Fetch;

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
pub trait Query<'data>: Sealed + From<QueryItem<'data, Self>> + 'data {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}
