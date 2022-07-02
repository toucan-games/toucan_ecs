use crate::entity::{Entity, Iter};

pub enum Entities<'data> {
    All(Iter<'data>),
    Optimized(Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>),
}

impl<'data> Iterator for Entities<'data> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Entities::All(iter) => iter.next(),
            Entities::Optimized(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'data> ExactSizeIterator for Entities<'data> {
    fn len(&self) -> usize {
        match self {
            Entities::All(iter) => iter.len(),
            Entities::Optimized(iter) => iter.len(),
        }
    }
}
