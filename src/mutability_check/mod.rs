use std::hash::BuildHasherDefault;

pub use checked::MutabilityChecked;

use hashbrown::HashMap;
use mutability::Mutability;

use crate::hash::TypeIdHasher;
use crate::type_id::DataTypeId;

mod checked;
mod impls;
mod mutability;
mod tuple;

type CheckMap = HashMap<DataTypeId, Mutability, BuildHasherDefault<TypeIdHasher>>;

pub trait MutabilityCheck: Send + Sync {
    const LENGTH: usize;

    fn check(check_map: &mut CheckMap);
}
