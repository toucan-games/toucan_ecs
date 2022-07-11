use crate::component::marker::Not;
use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::*;
use crate::Entity;

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
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> Query<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<'data, C>;
}

cfg_resource! {
    impl<'data, R> Query<'data> for marker::Resource<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceRead<'data, R>;
    }

    impl<'data, R> Query<'data> for Option<marker::Resource<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionRead<'data, R>;
    }

    impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceWrite<'data, R>;
    }

    impl<'data, R> Query<'data> for Option<marker::ResourceMut<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionWrite<'data, R>;
    }
}
