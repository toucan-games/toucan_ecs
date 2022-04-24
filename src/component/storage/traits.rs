use crate::component::Component;
use crate::Entity;

pub trait Storage: Default + Send + Sync + 'static {
    type Item: Component;

    fn attach(&mut self, entity: Entity, component: Self::Item);

    fn attached(&self, entity: Entity) -> bool;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    fn remove(&mut self, entity: Entity);

    fn clear(&mut self);
}
