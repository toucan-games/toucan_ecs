use std::any::Any;

use crate::Entity;

pub trait Pool: 'static {
    fn remove(&mut self, entity: Entity);

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
