use multimap::MultiMap;

pub use checked::MutabilityChecked;

use crate::type_id::DataTypeId;

mod checked;
mod impls;
mod tuple;

pub trait MutabilityCheck: Send + Sync {
    const MUTABLE: bool;
    fn extend_before_check(multimap: &mut MultiMap<DataTypeId, bool>);
}
