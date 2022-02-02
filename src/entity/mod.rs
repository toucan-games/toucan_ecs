pub mod entry;
pub mod refs;
pub mod registry;
pub mod view;

slotmap::new_key_type! {
    pub struct Entity;
}
