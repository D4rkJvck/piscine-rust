pub mod roman;

pub use roman::*;

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current = (0..4000)
            .filter(|&r| self.0 == RomanNumber::from(r as u32).0)
            .collect::<Vec<u32>>()[0];

        Some(Self::from(current + 1))
    }
}
