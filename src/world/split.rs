use crate::world::components::{Components, ComponentsMut};
use crate::world::resources::{Resources, ResourcesMut};

/// **Immutable** borrowed type of the [world](crate::world::World) that contains
/// all the data of this world.
pub struct Split<'data> {
    components: Components<'data>,
    resources: Resources<'data>,
}

impl<'data> Split<'data> {
    pub(super) fn new(components: Components<'data>, resources: Resources<'data>) -> Self {
        Self {
            components,
            resources,
        }
    }

    /// Divides world's borrow into separate
    /// **immutable** borrows of components and resources.
    pub fn destruct(self) -> (Components<'data>, Resources<'data>) {
        let components = self.components;
        let resources = self.resources;
        (components, resources)
    }
}

/// **Mutable** borrowed type of the [world](crate::world::World) that contains
/// all the data of this world.
pub struct SplitMut<'data> {
    components: ComponentsMut<'data>,
    resources: ResourcesMut<'data>,
}

impl<'data> SplitMut<'data> {
    pub(super) fn new(components: ComponentsMut<'data>, resources: ResourcesMut<'data>) -> Self {
        Self {
            components,
            resources,
        }
    }

    /// Divides world's borrow into separate
    /// **mutable** borrows of components and resources.
    pub fn destruct(self) -> (ComponentsMut<'data>, ResourcesMut<'data>) {
        let components = self.components;
        let resources = self.resources;
        (components, resources)
    }
}
