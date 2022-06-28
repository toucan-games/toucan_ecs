//! Provides trait for components and borrow types for them.

pub(crate) use registry::Registry;
pub(crate) use set::ComponentSet;
pub(crate) use storage::{Iter, IterMut, Storage, StorageImpl};
pub(crate) use type_id::ComponentTypeId;

mod registry;
mod set;
mod storage;
mod type_id;

pub mod marker;

/// Trait for data that can be attached to the entity.
///
/// This trait is implemented for all the types which implement [`Copy`], [`Send`], [`Sync`] traits
/// and contain no non-static references.
///
/// It implements [`Copy`] trait to ensure that type does not manage some resource
/// because copyable types cannot implement [`Drop`].
pub trait Component: Copy + Send + Sync + 'static {}

impl<T> Component for T where T: Copy + Send + Sync + 'static {}
