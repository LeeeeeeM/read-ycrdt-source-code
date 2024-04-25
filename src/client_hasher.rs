use core::hash::Hasher;


#[derive(Default)]
pub struct ClientHasher {
    prefix: u64
}

impl Hasher for ClientHasher {
    fn finish(&self) -> u64 {
        self.prefix
    }
    fn write(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 8);
        // we only expect a single value to be written
        debug_assert!(self.prefix == 0);
        self.prefix = if bytes.len() == 4 {
            u32::from_ne_bytes(<[u8; 4]>::try_from(bytes).unwrap()) as u64
        } else {
            u64::from_ne_bytes(<[u8; 8]>::try_from(bytes).unwrap())
        };
    }
}