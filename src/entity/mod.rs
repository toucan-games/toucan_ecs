//! Provides entity handle, entry for the specific entity in the world.

mod fetch;
mod query;
pub(crate) mod registry;

slotmap::new_key_type! {
    /// Unique handle of the entity in ECS.
    ///
    /// Similarly as in arenas, you can store it anywhere
    /// to obtain components attached to the entity.
    pub struct Entity;
}
