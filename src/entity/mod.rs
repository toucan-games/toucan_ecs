pub use entry::Entry;
pub use registry::Registry;

pub mod view;

mod entry;
mod fetch;
mod registry;
mod viewable;

slotmap::new_key_type! {
    /// Unique handle of the entity in ECS.
    ///
    /// Similarly as in arenas, can be stored anywhere
    /// to obtain components attached to the entity later.
    pub struct Entity;
}
