use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Provides different kinds of reference:
/// [immutable](RefKind::Ref) or [mutable](RefKind::Mut) one.
pub enum RefKind<'data, T>
where
    T: ?Sized,
{
    /// Immutable kind of reference.
    Ref(&'data T),
    /// Mutable kind of reference.
    Mut(&'data mut T),
}

impl<'data, T> From<&'data T> for RefKind<'data, T>
where
    T: ?Sized,
{
    fn from(r#ref: &'data T) -> Self {
        Self::Ref(r#ref)
    }
}

impl<'data, T> From<&'data mut T> for RefKind<'data, T>
where
    T: ?Sized,
{
    fn from(r#mut: &'data mut T) -> Self {
        Self::Mut(r#mut)
    }
}

/// Storage for different kinds of reference.
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving it out of the storage to preserve specified lifetime of the owner.
#[repr(transparent)]
pub struct RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher,
{
    map: HashMap<K, Option<RefKind<'data, V>>, S>,
}

impl<'data, K, V, S> RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher,
{
    /// Returns an immutable reference of the value without
    /// preserving lifetime of the owner, so without changing this storage.
    ///
    /// ## Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the storage.
    pub fn get_ref(&self, key: &K) -> Option<&V> {
        let option = self.map.get(key)?.as_ref();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#ref = match ref_kind {
            RefKind::Ref(r#ref) => *r#ref,
            RefKind::Mut(r#ref) => &**r#ref,
        };
        Some(r#ref)
    }

    /// Moves an immutable reference of the value out of this storage.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this storage.
    ///
    /// ## Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the storage.
    pub fn move_ref(&mut self, key: K) -> Option<&'data V> {
        let option = self.map.get(&key)?.as_ref();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#ref = match ref_kind {
            RefKind::Ref(r#ref) => *r#ref,
            RefKind::Mut(_) => {
                let option = self.map.remove(&key)?;
                let ref_kind = option.expect(BORROWED_MUTABLY);
                match ref_kind {
                    RefKind::Ref(_) => unreachable!(),
                    RefKind::Mut(r#mut) => {
                        let r#ref = &*r#mut;
                        let ref_kind = Some(RefKind::Ref(r#ref));
                        self.map.insert(key, ref_kind);
                        r#ref
                    }
                }
            }
        };
        Some(r#ref)
    }

    /// Moves a mutable reference of the value out of this storage.
    ///
    /// ## Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the storage
    /// or the value was already borrowed as immutable.
    pub fn move_mut(&mut self, key: K) -> Option<&'data mut V> {
        let option = self.map.remove(&key)?;
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#mut = match ref_kind {
            RefKind::Ref(r#ref) => {
                let ref_kind = Some(RefKind::Ref(r#ref));
                self.map.insert(key, ref_kind);
                move_mut_failed()
            }
            RefKind::Mut(r#mut) => {
                self.map.insert(key, None);
                r#mut
            }
        };
        Some(r#mut)
    }
}

impl<K, V, S> Default for RefKindStorage<'_, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    /// Constructs an empty storage, with the [Default] value for the hasher.
    fn default() -> Self {
        let map = HashMap::default();
        Self { map }
    }
}

impl<'data, K, V, S> FromIterator<(K, &'data V)> for RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'data V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Ref(v))))
            .collect();
        Self { map }
    }
}

impl<'data, K, V, S> FromIterator<(K, &'data mut V)> for RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'data mut V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Mut(v))))
            .collect();
        Self { map }
    }
}

impl<'data, K, V, S> Extend<(K, &'data V)> for RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn extend<T: IntoIterator<Item = (K, &'data V)>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|(k, v)| (k, Some(RefKind::Ref(v))));
        self.map.extend(iter)
    }
}

impl<'data, K, V, S> Extend<(K, &'data mut V)> for RefKindStorage<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn extend<T: IntoIterator<Item = (K, &'data mut V)>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|(k, v)| (k, Some(RefKind::Mut(v))));
        self.map.extend(iter)
    }
}

const BORROWED_MUTABLY: &str = "reference was already borrowed mutably";
const BORROWED_IMMUTABLY: &str = "reference was already borrowed immutably";

#[cold]
#[track_caller]
fn move_mut_failed() -> ! {
    panic!("{}", BORROWED_IMMUTABLY)
}
