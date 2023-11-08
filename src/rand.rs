use sha3::{Digest, Sha3_256};

/// Infinite generator of pseudo-random bytes
#[derive(Clone)]
pub struct ByteGenerator {
    seed: [u8; 32],
    stock: [u8; 32],
    idx: usize,
    itr: usize,
}

impl ByteGenerator {
    pub fn new(seed: &[u8]) -> Self {
        let mut bytes = [0u8; 32];

        bytes.iter_mut()
            .zip(seed.iter())
            .for_each(|(dst, src)| {
                *dst = *src;
            });

        Self { seed: bytes, stock: [0u8; 32], idx: 0, itr: 0 }
    }
}

impl Iterator for ByteGenerator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx {
            0 => {
                let mut payload = [0u8; 256];

                self.seed.iter()
                    .chain(self.itr.to_be_bytes().iter())
                    .zip(payload.iter_mut())
                    .for_each(|(src, dst)| *dst = *src);

                let mut hasher = Sha3_256::new();
                hasher.update(payload);
                let data: [u8; 32] = hasher.finalize().into();

                data.iter().zip(self.stock.iter_mut()).for_each(|(data, stock)| {
                    *stock = *data;
                });

                self.idx = 31;
                Some(self.stock[31])
            },
            _ => {
                self.idx -= 1;
                let value = self.stock[self.idx];
                Some(value)
            }
        }
    }
}

/// Fills a 2D slice with random bytes
pub fn fill<const N: usize>(dst: &mut [[u8; N]], seed: &[u8]) {
    let gen = ByteGenerator::new(seed);

    for i in 0..dst.len() {
        dst[i].iter_mut()
            .zip(gen.clone())
            .for_each(|(dst, src)| *dst = src);
    }
}
