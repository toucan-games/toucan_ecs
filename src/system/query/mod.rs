//! Provides types which can be queried by the [system](crate::system::System).
//!
//! Such types are:
//! - todo

use private::QuerySealed;

use super::Fetch;

mod impls;

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

mod private {
    pub trait QuerySealed {}
}
