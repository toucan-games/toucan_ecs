//! Provides types which can be queried in the [world](crate::world::World).
//!
//! Such types are:
//! - [`Entity`](crate::entity::Entity) handle;
//! - immutable and mutable [references] of [components];
//! - immutable and mutable [references] of [components] wrapped in [`Option`];
//! - immutable and mutable [references] of [resources] (if enabled by `resource` feature);
//! - [marker](crate::component::marker) types of components;
//! - [marker](crate::resource::marker) types of resources (if enabled by `resource` feature);
//! - and [tuples] of types listed above.
//!
//! [references]: prim@reference
//! [tuples]: prim@tuple
//! [components]: crate::component::Component
//! [resources]: crate::resource::Resource

use std::any::TypeId;
use std::marker::PhantomData;

use multimap::MultiMap;

pub(crate) use private::QuerySealed;
use soundness_check::SoundnessCheck as QueryMutSealed;
pub(crate) use soundness_check::{SoundnessCheck, SoundnessChecked};

use super::fetch::{Fetch, FetchMut};

mod soundness_check;
mod tuple;

pub(crate) type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

/// Type which can be queried by [view](crate::world::View)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait Query<'data>: 'data + QuerySealed {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}

pub(crate) mod private {
    pub trait QuerySealed {}
}

pub(crate) type QueryMutItem<'data, Q> = <<Q as QueryMut<'data>>::Fetch as FetchMut<'data>>::Item;

/// Type which can be queried by **mutable** [view](crate::world::ViewMut)
/// of the [world](crate::world::World).
///
/// This trait is **sealed** and cannot be implemented for types outside of `toucan_ecs`.
pub trait QueryMut<'data>: 'data + QueryMutSealed {
    #[doc(hidden)]
    type Fetch: FetchMut<'data>;
}

pub(crate) struct CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    _checked: SoundnessChecked<Q>,
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> CheckedQuery<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(super) fn new() -> Self {
        Self {
            _ph: PhantomData,
            _checked: SoundnessChecked::default(),
        }
    }
}
