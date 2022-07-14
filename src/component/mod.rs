//! Provides trait for components and borrow types for them.

pub(crate) use registry::Registry;
pub(crate) use set::ComponentSet;
pub(crate) use type_id::ComponentTypeId;

/// Derive macro that implements [`Component`] trait.
///
/// # Examples
///
/// Use default storage type:
///
/// ```
/// use toucan_ecs::component::Component;
/// use toucan_ecs::component::storage::DefaultStorage;
///
/// #[derive(Copy, Clone, Component)]
/// pub struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// Use custom storage type:
///
/// ```text
/// // TODO: write an example
/// ```
pub use toucan_ecs_derive::Component;

mod registry;
mod set;
mod type_id;

pub mod marker;
pub mod storage;

/// Trait for data that can be attached to the entity.
///
/// This trait must be implemented for the types which implement
/// [`Copy`], [`Send`], [`Sync`] traits and contain no non-static references.
///
/// It implements [`Copy`] trait to ensure that type does not manage some resource
/// because copyable types cannot implement [`Drop`].
///
/// ## How can I implement `Component`?
///
/// You can implement this trait with derive macro:
///
/// ```
/// use toucan_ecs::component::Component;
/// use toucan_ecs::component::storage::DefaultStorage;
///
/// #[derive(Copy, Clone, Component)]
/// pub struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// or manually (equivalent to the derive macro usage above):
///
/// ```
/// use toucan_ecs::component::Component;
/// use toucan_ecs::component::storage::DefaultStorage;
///
/// #[derive(Copy, Clone)]
/// pub struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// impl Component for Position {
///     type Storage = DefaultStorage<Self>;
/// }
/// ```
pub trait Component: Copy + Send + Sync + 'static {
    /// Type of storage which will be used by the crate
    /// to store this type of component.
    type Storage: storage::Storage<Item = Self>;
}
