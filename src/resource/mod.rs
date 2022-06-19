//! Provides trait for resources and borrow types for them.

use holder::ErasedResourceHolder;
pub(crate) use holder::ResourceHolder;
pub(crate) use registry::Registry;
use type_id::ResourceTypeId;

mod fetch;
mod holder;
pub mod marker;
mod query;
mod registry;
mod type_id;

/// Trait for data that can be stored as singleton in ECS.
///
/// This trait is implemented for all the types which implement [`Send`], [`Sync`] traits
/// and doesn't contain any non-static references.
///
/// Unlike [components][`crate::component::Component`],
/// resources does not need to be [copyable][`Copy`] because they are used
/// to share some state across entities and manage some resources.
///
/// Storing and accessing resources can be useful to access unique data in systems.
pub trait Resource: Send + Sync + 'static {}

impl<T> Resource for T where T: Send + Sync + 'static {}
