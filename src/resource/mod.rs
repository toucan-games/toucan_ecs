//! Provides trait for resources and borrow types for them.

use erased::ErasedResourceHolder;
pub(crate) use registry::Registry;
pub(crate) use registry_refs::RegistryRefs;
pub(crate) use set::ResourceSet;
/// Derive macro that implements [`Resource`] trait.
///
/// # Examples
///
/// ```
/// use toucan_ecs::prelude::*;
///
/// #[derive(Resource)]
/// pub struct Time {
///     start: std::time::Instant,
/// }
/// ```
pub use toucan_ecs_macro::Resource;
pub(crate) use type_id::ResourceTypeId;

mod erased;
mod registry;
mod registry_refs;
mod set;
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
///
/// ## How can I implement `Resource`?
///
/// You can implement this trait with derive macro:
///
/// ```
/// use toucan_ecs::prelude::*;
///
/// #[derive(Resource)]
/// pub struct Time {
///     start: std::time::Instant,
/// }
/// ```
///
/// or manually (equivalent to the derive macro usage above):
///
/// ```
/// use toucan_ecs::prelude::*;
///
/// pub struct Time {
///     start: std::time::Instant,
/// }
///
/// impl Resource for Time {}
/// ```
pub trait Resource: Send + Sync + 'static {}
