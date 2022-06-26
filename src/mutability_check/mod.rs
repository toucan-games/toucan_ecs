use multimap::MultiMap;

pub use checked::MutabilityChecked;
pub use type_id::TypeId;

mod checked;
mod impls;
mod tuple;
mod type_id;

pub trait MutabilityCheck: Send + Sync {
    const MUTABLE: bool;
    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>);
}
