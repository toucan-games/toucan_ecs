//! Provides trait for components and borrow types for them.

pub(crate) use registry::Registry;
pub(crate) use registry_refs::RegistryRefs;
pub(crate) use set::ComponentSet;
/// Derive macro that implements [`Component`] trait.
///
/// # Examples
///
/// Use default storage type:
///
/// ```
/// use toucan_ecs::prelude::*;
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
/// ```
/// use toucan_ecs::prelude::*;
///
/// #[derive(Copy, Clone, Component)]
/// #[component(storage = "CustomStorage")]
/// pub struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Default)]
/// pub struct CustomStorage;
///
/// impl Storage for CustomStorage {
///     type Item = Position;
///
///     /* ... */
/// #    fn attach(&mut self, entity: Entity, component: Self::Item) { unimplemented!() }
/// #    fn attached(&self, entity: Entity) -> bool { unimplemented!() }
/// #    fn get(&self, entity: Entity) -> Option<&Self::Item> { unimplemented!() }
/// #    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> { unimplemented!() }
/// #    fn remove(&mut self, entity: Entity) { unimplemented!() }
/// #    fn clear(&mut self) { unimplemented!() }
/// #    fn iter(&self) -> Box<DynIter<Self::Item>> { unimplemented!() }
/// #    fn iter_mut(&mut self) -> Box<DynIterMut<Self::Item>> { unimplemented!() }
/// }
/// ```
pub use toucan_ecs_macro::Component;
pub(crate) use type_id::ComponentTypeId;

mod registry;
mod registry_refs;
mod set;
mod type_id;

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
/// use toucan_ecs::prelude::*;
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
/// use toucan_ecs::prelude::*;
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
