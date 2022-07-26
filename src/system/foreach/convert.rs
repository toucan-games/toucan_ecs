use std::marker::PhantomData;

use crate::system::foreach::{ForeachHolder, ForeachSystem, Query};
use crate::system::System;

/// Allows to convert [`ForeachSystem`](ForeachSystem) trait into the type
/// that implements [`System`](System) trait.
pub struct FromForeachSystem<'data, S, Q>
where
    S: ForeachSystem<'data, Q>,
    Q: Query<'data>,
{
    system: S,
    _ph: PhantomData<&'data Q>,
}

impl<'data, S, Q> From<S> for FromForeachSystem<'data, S, Q>
where
    S: ForeachSystem<'data, Q>,
    Q: Query<'data>,
{
    fn from(system: S) -> Self {
        Self {
            system,
            _ph: PhantomData,
        }
    }
}

impl<'data, S, Q> System<'data, ForeachHolder<'data, Q>> for FromForeachSystem<'data, S, Q>
where
    S: ForeachSystem<'data, Q>,
    Q: Query<'data>,
{
    fn run(&mut self, holder: ForeachHolder<'data, Q>) {
        for args in holder {
            self.system.run(args)
        }
    }
}
