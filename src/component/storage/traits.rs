use as_any::AsAny;

use crate::entity::Entity;

pub trait Storage: Send + Sync + 'static + AsAny {
    fn remove(&mut self, entity: Entity);

    fn attached(&self, entity: Entity) -> bool;

    fn clear(&mut self);
}
