use std::fmt;

#[derive(Clone, Copy)]
pub struct PeerId([u8; 20]);

impl PeerId {
    pub fn new() -> Self {
        let mut buffer = [0u8; 20];
        let fill = "Graphgear 250 Pentel";
        buffer.copy_from_slice(fill.as_bytes());

        Self(buffer)
    }
}

impl AsRef<[u8]> for PeerId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}
