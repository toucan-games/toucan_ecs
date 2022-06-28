use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::fetch::*;
use crate::world::query;
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};

use super::Query;

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<C>;
}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<C>;
}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<C>;
}

impl<'data, C> Query<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<C>;
}

impl<'data, C> Query<'data> for ViewOne<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOne<C>;
}

impl<'data, C> Query<'data> for ViewOneMut<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOneMut<C>;
}

impl<'data, Q> Query<'data> for View<'data, Q>
where
    Q: query::Query<'data>,
{
    type Fetch = FetchView<'data, Q>;
}

impl<'data, Q> Query<'data> for ViewMut<'data, Q>
where
    Q: query::QueryMut<'data>,
{
    type Fetch = FetchViewMut<'data, Q>;
}

cfg_resource! {
    impl<'data, R> Query<'data> for marker::Resource<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceRead<R>;
    }
}

cfg_resource! {
    impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceWrite<R>;
    }
}
