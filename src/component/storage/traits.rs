use std::any::Any;

use crate::entity::Entity;

pub trait Storage: Send + Sync + 'static {
    fn remove(&mut self, entity: Entity);

    fn attached(&self, entity: Entity) -> bool;

    fn clear(&mut self);

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
