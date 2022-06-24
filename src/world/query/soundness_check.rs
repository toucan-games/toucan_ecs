use std::any::TypeId;
use std::marker::PhantomData;

use multimap::MultiMap;

pub trait SoundnessCheck {
    const MUTABLE: bool;
    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>);
}

pub struct SoundnessChecked<T>
where
    T: SoundnessCheck,
{
    _ph: PhantomData<*const T>,
}

impl<T> Default for SoundnessChecked<T>
where
    T: SoundnessCheck,
{
    fn default() -> Self {
        check_soundness::<T>();
        Self { _ph: PhantomData }
    }
}

fn check_soundness<T>()
where
    T: SoundnessCheck,
{
    let mut multimap = MultiMap::new();
    T::extend_before_check(&mut multimap);

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

    use super::*;

    #[derive(Copy, Clone)]
    struct Position(f32, f32, f32);

    #[derive(Copy, Clone)]
    struct Velocity(f32, f32, f32);

    #[derive(Copy, Clone)]
    struct Mass(f32);

    #[test]
    fn one_type() {
        check_soundness::<&Position>();
        check_soundness::<(&Position,)>();
        check_soundness::<(&Position, Not<&Position>)>();
        check_soundness::<(Not<&Position>, &Position, Option<&Position>)>();

        check_soundness::<&mut Position>();
        check_soundness::<(&mut Position,)>();
    }

    #[test]
    #[should_panic(expected = "mutable borrow occurs while other immutable occurrences was found")]
    fn one_type_mutable_borrow() {
        check_soundness::<(&mut Position, Option<&Position>)>();
    }

    #[test]
    #[should_panic(expected = "multiple mutable borrows occur")]
    fn one_type_mutable_borrows() {
        check_soundness::<(Option<&mut Position>, Not<&Position>, &mut Position)>();
    }

    #[test]
    fn multiple_types() {
        check_soundness::<&Velocity>();
        check_soundness::<(&Velocity,)>();
        check_soundness::<(&Position, &Velocity)>();
        check_soundness::<(&Position, Not<&Velocity>, Option<&Position>)>();
        check_soundness::<(
            &Position,
            &Velocity,
            Option<&Mass>,
            &Position,
            &Mass,
            &Velocity,
        )>();

        check_soundness::<&mut Velocity>();
        check_soundness::<(&mut Mass,)>();
    }

    #[test]
    #[should_panic(expected = "mutable borrow occurs while other immutable occurrences was found")]
    fn multiple_types_mutable_borrow() {
        check_soundness::<(&mut Mass, &mut Velocity, &Mass, &Position)>();
    }

    #[test]
    #[should_panic(expected = "multiple mutable borrows occur")]
    fn multiple_types_mutable_borrows() {
        check_soundness::<(&Velocity, &mut Position, &Mass, &mut Position)>();
    }
}
