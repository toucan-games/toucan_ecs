pub enum Mutability {
    Immutable,
    Mutable,
}

impl Mutability {
    pub const fn is_immutable(&self) -> bool {
        matches!(self, Self::Immutable)
    }

    pub const fn is_mutable(&self) -> bool {
        matches!(self, Self::Mutable)
    }
}
