use std::any::Any;

pub use storage::ResourceStorage;
pub use type_id::ResourceTypeId;

mod storage;
mod type_id;

/// Trait for data that can be stored as singleton in ECS.
///
/// This trait is implemented for all the types which implement [`Send`], [`Sync`] traits
/// and contain no non-static references.
///
/// Unlike [components][`crate::Component`], resources does not need to be [copyable][`Copy`]
/// because they are used to share some state across entities and manage some resources.
///
/// Storing and accessing resources can be useful to access unique data in systems.
pub trait Resource: Send + Sync + 'static {
    #[doc(hidden)]
    fn as_any_ref(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Resource for T
where
    T: Send + Sync + 'static,
{
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
