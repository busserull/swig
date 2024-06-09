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

    pub fn url_safe(&self) -> String {
        let mut url = String::with_capacity(30);

        for (i, nibble) in hex::encode(&self.0).chars().enumerate() {
            if i % 2 == 0 {
                url.push_str("%");
            }

            url.push(nibble.to_ascii_uppercase());
        }

        url
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

impl PartialEq for Sha1 {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}
