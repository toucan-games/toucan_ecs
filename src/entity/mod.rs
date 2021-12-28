mod builder;
pub mod registry;

slotmap::new_key_type! {
    pub struct Entity;
}
