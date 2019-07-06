

pub struct SimpleRNG {
    x: u32,
}

impl Default for SimpleRNG {
    fn default() -> Self {
        Self { x: 34 }
    }
}

impl snake::RandomNumberGenerator for SimpleRNG {
    fn next(&mut self) -> u32 {
        self.x = (7 * self.x) % 11;
        self.x
    }
}

