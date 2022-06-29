pub use holder::{ErasedStorageHolder, StorageHolder, StorageHolderMut};
pub use impls::StorageImpl;

use crate::component::Component;
use crate::Entity;

mod holder;
mod impls;

pub type Iter<'data, C> = dyn Iterator<Item = (Entity, &'data C)> + Send + Sync + 'data;

pub type IterMut<'data, C> = dyn Iterator<Item = (Entity, &'data mut C)> + Send + Sync + 'data;

pub trait Storage: Send + Sync + 'static {
    type Item: Component;

    fn attach(&mut self, entity: Entity, component: Self::Item);

    fn attached(&self, entity: Entity) -> bool;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    fn remove(&mut self, entity: Entity);

    fn clear(&mut self);

    // fixme move to associated type when GATs are stabilized
    fn iter(&self) -> Box<Iter<Self::Item>>;

    // fixme move to associated type when GATs are stabilized
    fn iter_mut(&mut self) -> Box<IterMut<Self::Item>>;
}
