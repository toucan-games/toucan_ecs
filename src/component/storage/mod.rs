pub use holder::{ErasedStorageHolder, StorageHolder};
pub use impls::{Iter, IterMut, StorageImpl};
pub use traits::Storage;

mod holder;
mod impls;
mod traits;
