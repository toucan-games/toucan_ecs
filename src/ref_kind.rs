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
