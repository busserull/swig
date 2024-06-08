use sha1;
use sha1::Digest;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Sha1([u8; 20]);

impl Sha1 {
    pub fn new_raw(sha: &[u8]) -> Self {
        let mut buffer = [0; 20];
        buffer[..].copy_from_slice(&sha[0..20]);

        Self(buffer)
    }

    pub fn digest(message: &[u8]) -> Self {
        let mut hasher = sha1::Sha1::new();
        hasher.update(message);

        Self::new_raw(&hasher.finalize())
    }
}

impl AsRef<[u8]> for Sha1 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Sha1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sha1({})", hex::encode(self.0))
    }
}
