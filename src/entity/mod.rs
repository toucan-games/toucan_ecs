pub use entry::Entry;
pub use registry::Registry;
pub use view_one::{ViewOne, ViewOneMut};

mod entry;
mod fetch;
mod registry;
mod view_one;
mod viewable;

slotmap::new_key_type! {
    /// Unique handle of the entity in ECS.
    ///
    /// Similarly as in arenas, can be stored anywhere
    /// to obtain components attached to the entity later.
    pub struct Entity;
}
