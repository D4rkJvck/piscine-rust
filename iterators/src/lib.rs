#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(Self::new(self.v));

        self.v = match self.v {
            0 | 1 => return None,
            n if n % 2 == 0 => n / 2,
            n => 3 * n + 1,
        };

        next
    }
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Self { v }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}
