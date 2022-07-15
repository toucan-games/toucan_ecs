use std::marker::PhantomData;

use super::{MultiMap, MutabilityCheck};

pub struct MutabilityChecked<T>(PhantomData<*const T>)
where
    T: MutabilityCheck;

impl<T> Default for MutabilityChecked<T>
where
    T: MutabilityCheck,
{
    fn default() -> Self {
        check::<T>();
        Self(PhantomData)
    }
}

fn check<T>()
where
    T: MutabilityCheck,
{
    let multimap = {
        let mut multimap = MultiMap::new();
        T::extend_before_check(&mut multimap);
        multimap
    };

    for (_, vec) in multimap {
        // all type occurrences are immutable, this is sound
        if vec.iter().all(|&it| !it) {
            continue;
        }
        // exactly one mutable type occurrence, this is sound too
        if vec.len() == 1 {
            continue;
        }
        // one mutable borrow with some immutable ones, this is unsound
        if vec.iter().filter(|&&it| it).count() == 1 {
            panic!("mutable borrow occurs while other immutable occurrences was found")
        }
        // multiple mutable borrows, this is unsound
        panic!("multiple mutable borrows occur")
    }
}

#[cfg(test)]
mod tests {
    use crate::component::marker::Not;
    use crate::component::storage::DefaultStorage;
    use crate::component::Component;

    use super::*;

    #[derive(Copy, Clone)]
    struct Position(f32, f32, f32);

    impl Component for Position {
        type Storage = DefaultStorage<Self>;
    }

    #[derive(Copy, Clone)]
    struct Velocity(f32, f32, f32);

    impl Component for Velocity {
        type Storage = DefaultStorage<Self>;
    }

    #[derive(Copy, Clone)]
    struct Mass(f32);

    impl Component for Mass {
        type Storage = DefaultStorage<Self>;
    }

    #[test]
    fn one_type() {
        check::<&Position>();
        check::<(&Position,)>();
        check::<(&Position, Not<Position>)>();
        check::<(Not<Position>, &Position, Option<&Position>)>();

        check::<&mut Position>();
        check::<(&mut Position,)>();
    }

    #[test]
    #[should_panic(expected = "mutable borrow occurs while other immutable occurrences was found")]
    fn one_type_mutable_borrow() {
        check::<(&mut Position, Option<&Position>)>();
    }

    #[test]
    #[should_panic(expected = "multiple mutable borrows occur")]
    fn one_type_mutable_borrows() {
        check::<(Option<&mut Position>, Not<Position>, &mut Position)>();
    }

    #[test]
    fn multiple_types() {
        check::<&Velocity>();
        check::<(&Velocity,)>();
        check::<(&Position, &Velocity)>();
        check::<(&Position, Not<Velocity>, Option<&Position>)>();
        check::<(
            &Position,
            &Velocity,
            Option<&Mass>,
            &Position,
            &Mass,
            &Velocity,
        )>();

        check::<&mut Velocity>();
        check::<(&mut Mass,)>();
    }

    #[test]
    #[should_panic(expected = "mutable borrow occurs while other immutable occurrences was found")]
    fn multiple_types_mutable_borrow() {
        check::<(&mut Mass, &mut Velocity, &Mass, &Position)>();
    }

    #[test]
    #[should_panic(expected = "multiple mutable borrows occur")]
    fn multiple_types_mutable_borrows() {
        check::<(&Velocity, &mut Position, &Mass, &mut Position)>();
    }
}
