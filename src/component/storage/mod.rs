//! Utilities for [storages](Storage) of components in ECS.

pub(crate) use erased::ErasedStorageHolder;
pub use impls::DefaultStorage;

use crate::component::Component;
use crate::entity::Entity;

mod erased;
mod impls;

/// Type of iterator over *immutable* data
/// which must be provided by each [`Storage`] implementation.
pub type DynIter<'data, C> = dyn ExactSizeIterator<Item = (Entity, &'data C)> + Send + Sync + 'data;

/// Type of iterator over *mutable* data
/// which must be provided by each [`Storage`] implementation.
pub type DynIterMut<'data, C> =
    dyn ExactSizeIterator<Item = (Entity, &'data mut C)> + Send + Sync + 'data;

/// Trait for storages of components in ECS.
///
/// By default, all the [components](Component) implemented by derive macro
/// use [`DefaultStorage`](DefaultStorage) type of storage:
///
/// ```
/// use toucan_ecs::component::Component;
/// use toucan_ecs::component::storage::DefaultStorage;
///
/// // Uses `DefaultStorage` type of storage
/// #[derive(Copy, Clone, Component)]
/// pub struct Velocity {
///     dx: f32,
///     dy: f32,
/// }
/// ```
///
/// This can be overridden by manual implementation of the [`Component`](Component) trait
/// or by `#[component(storage = "...")]` helper attribute:
///
/// ```
/// // TODO: write an example
/// ```
pub trait Storage: Default + Send + Sync + 'static {
    /// Type of component which is stored by this storage.
    type Item: Component;

    /// Attaches provided component to the entity.
    fn attach(&mut self, entity: Entity, component: Self::Item);

    /// Checks if entity has component of item type.
    fn attached(&self, entity: Entity) -> bool;

    /// Retrieves the shared borrow of the component of item type attached to the entity.
    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    /// Retrieves the unique borrow of the component of item type attached to the entity.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    /// Removes component of item type from the entity.
    fn remove(&mut self, entity: Entity);

    /// Clears this storage, destroying all components.
    fn clear(&mut self);

    /// Returns iterator over *immutable* data.
    // fixme move to associated type when GATs are stabilized
    fn iter(&self) -> Box<DynIter<Self::Item>>;

    /// Returns iterator over *mutable* data.
    // fixme move to associated type when GATs are stabilized
    fn iter_mut(&mut self) -> Box<DynIterMut<Self::Item>>;
}
