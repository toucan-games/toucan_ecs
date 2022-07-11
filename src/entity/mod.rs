//! Provides entity handle in the world.

pub use builder::EntityBuilder;
pub(crate) use entities::Entities;
pub(crate) use registry::{Iter, Registry};

mod builder;
mod entities;
mod registry;

slotmap::new_key_type! {
    /// Unique handle of the entity in ECS.
    ///
    /// Similarly as in arenas, you can store it anywhere
    /// to obtain components attached to the entity.
    pub struct Entity;
}
