use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
use crate::fetch::*;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::query::{Query, QueryMut};

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data> QueryMut<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> QueryMut<'data> for &'data C
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

impl<'data, C> QueryMut<'data> for Option<&'data C>
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

impl<'data, C> QueryMut<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> QueryMut<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> QueryMut<'data> for Option<&'data mut C>
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

    impl<'data, R> QueryMut<'data> for marker::Resource<'data, R>
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

    impl<'data, R> QueryMut<'data> for Option<marker::Resource<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionRead<'data, R>;
    }

    impl<'data, R> QueryMut<'data> for marker::ResourceMut<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceWrite<'data, R>;
    }

    impl<'data, R> QueryMut<'data> for Option<marker::ResourceMut<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionWrite<'data, R>;
    }
}
