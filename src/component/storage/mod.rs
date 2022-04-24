pub use holder::StorageHolder;
pub(super) use impls::ComponentKey;
pub use impls::StorageImpl;
pub use traits::Storage;

mod holder;
mod impls;
mod traits;
