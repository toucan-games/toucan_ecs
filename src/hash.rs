use std::hash::Hasher;

#[derive(Default)]
pub struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        debug_assert_eq!(bytes.len(), 8);
        let _ = bytes
            .try_into()
            .map(|array| self.0 = u64::from_ne_bytes(array));
    }
}
