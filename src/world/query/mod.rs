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

use crate::system::foreach;

mod impls;
mod tuple;

/// Type which can be queried by **shared** [view](crate::world::view::View)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: QueryMut<'data> {}

/// Type which can be queried by **mutable** [view](crate::world::view::ViewMut)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait QueryMut<'data>: foreach::Query<'data> {}
