use std::any::{Any, TypeId};
use std::collections::HashMap;

use slotmap::SlotMap;

use crate::component::pool::ComponentPool;
use crate::{Component, Entity};

pub struct Registry {
    entities: SlotMap<Entity, ()>,
    pools: HashMap<TypeId, Box<dyn Any>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
            pools: HashMap::new(),
        }
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        if let None = self.get_pool::<C>() {
            let pool = ComponentPool::<C>::new();
            self.pools.insert(type_id, Box::new(pool));
        }
    }

    pub fn create(&mut self) -> Entity {
        self.entities.insert(())
    }

    pub fn create_with<S>(&mut self, components: S) -> Entity
    where
        S: AddSet,
    {
        let entity = self.create();
        self.add_set(entity, components);
        entity
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        self.entities.remove(entity).is_some()
    }

    pub fn add<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        let pool = self
            .get_pool_mut::<C>()
            .expect("component must be registered to be used");
        pool.save(entity, component);
    }

    pub fn add_set<S>(&mut self, entity: Entity, components: S)
    where
        S: AddSet,
    {
        components.add_set(self, entity)
    }

    pub fn view<C>(&self) -> impl Iterator<Item = (Entity, &C)>
    where
        C: Component,
    {
        // todo get view on set of components from tuple (by macros 😨)
        let pool = self
            .get_pool::<C>()
            .expect("component must be registered to be used");
        self.entities.iter().map(|(entity, _)| {
            let component = &pool[entity];
            (entity, component)
        })
    }

    fn get_pool<C>(&self) -> Option<&ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let pool = self.pools.get(&type_id)?;
        let pool = pool.as_ref().downcast_ref().expect("downcast error");
        Some(pool)
    }

    fn get_pool_mut<C>(&mut self) -> Option<&mut ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let pool = self.pools.get_mut(&type_id)?;
        let pool = pool.as_mut().downcast_mut().expect("downcast error");
        Some(pool)
    }
}

pub trait AddSet {
    fn add_set(self, registry: &mut Registry, entity: Entity);
}

mod impls {
    use super::*;

    macro_rules! add_set_impl {
        ($($arg:ident),* | $($count:tt),*) => {
            impl<$($arg),*> AddSet for ($($arg,)*)
                where
                    $($arg: Component,)*
            {
                fn add_set(self, registry: &mut Registry, entity: Entity) {
                    $(registry.add(entity, self.$count);)*
                }
            }
        };
    }

    add_set_impl!(A | 0);
    add_set_impl!(A, B | 0, 1);
    add_set_impl!(A, B, C | 0, 1, 2);
    add_set_impl!(A, B, C, D | 0, 1, 2, 3);
    add_set_impl!(A, B, C, D, E | 0, 1, 2, 3, 4);
    add_set_impl!(A, B, C, D, E, F | 0, 1, 2, 3, 4, 5);
    add_set_impl!(A, B, C, D, E, F, G | 0, 1, 2, 3, 4, 5, 6);
    add_set_impl!(A, B, C, D, E, F, G, H | 0, 1, 2, 3, 4, 5, 6, 7);
    add_set_impl!(A, B, C, D, E, F, G, H, I | 0, 1, 2, 3, 4, 5, 6, 7, 8);
    add_set_impl!(A, B, C, D, E, F, G, H, I, J | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    #[rustfmt::skip]
    add_set_impl!(A, B, C, D, E, F, G, H, I, J, K | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    #[rustfmt::skip]
    add_set_impl!(A, B, C, D, E, F, G, H, I, J, K, L | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
}
