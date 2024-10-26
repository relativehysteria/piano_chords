pub struct Rng(usize);

impl Rng {
    pub fn new(seed: usize) -> Self {
        let mut rng = Self(seed);

        // Couple initial runs to get around shitty seeds
        (0..32).for_each(|_| { rng.next(); });

        rng
    }

    pub fn next(&mut self) -> usize {
        let ret = self.0;
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 43;
        ret
    }
}
