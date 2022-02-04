pub mod entry;
pub mod refs;
pub mod registry;
pub mod view;

slotmap::new_key_type! {
    /// Unique handle of the entity in ECS.
    ///
    /// Similarly as in arenas, can be stored anywhere
    /// to obtain components attached to the entity later.
    pub struct Entity;
}
