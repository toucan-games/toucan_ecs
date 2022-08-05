use crate::component::Component;
use crate::entity::Entity;
use crate::marker;
use crate::world::query::{Query, QueryMut};
#[cfg(feature = "resource")]
use crate::{
    resource::Resource,
    world::query::{ResourceQuery, ResourceQueryMut},
};

impl<'data> Query<'data> for Entity {}

impl<'data> QueryMut<'data> for Entity {}

impl<'data, C> Query<'data> for &'data C where C: Component {}

impl<'data, C> QueryMut<'data> for &'data C where C: Component {}

impl<'data, C> Query<'data> for Option<&'data C> where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for marker::Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for marker::Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for &'data mut C where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data mut C> where C: Component {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQuery<'data> for marker::Resource<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQueryMut<'data> for marker::Resource<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQuery<'data> for Option<marker::Resource<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQueryMut<'data> for Option<marker::Resource<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQueryMut<'data> for marker::ResourceMut<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> ResourceQueryMut<'data> for Option<marker::ResourceMut<'data, R>> where R: Resource {}
