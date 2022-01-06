pub mod entry;
pub mod registry;
pub mod view;

slotmap::new_key_type! {
    pub struct Entity;
}
