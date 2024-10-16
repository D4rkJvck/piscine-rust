#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.v = match self.v {
            0 | 1 => 0,
            v if v % 2 == 0 => v / 2,
            v => 3 * v + 1,
        };

        match self.v {
            0 => None,
            _ => Some(Self::new(self.v)),
        }
    }
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Self { v }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut count = 0;
    let mut collatz = Collatz::new(n);

    while let Some(_) = collatz.next() {
        count += 1;
    }

    count
}
