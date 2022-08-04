use crate::component::Component;
use crate::entity::Entity;
use crate::marker::Not;
use crate::world::query::{Query, QueryMut};

impl<'data> Query<'data> for Entity {}

impl<'data> QueryMut<'data> for Entity {}

impl<'data, C> Query<'data> for &'data C where C: Component {}

impl<'data, C> QueryMut<'data> for &'data C where C: Component {}

impl<'data, C> Query<'data> for Option<&'data C> where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for &'data mut C where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data mut C> where C: Component {}
