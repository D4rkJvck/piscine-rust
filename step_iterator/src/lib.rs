use std::{iter::Iterator, ops::AddAssign};

pub struct StepIterator<T> {
    beg: T,
    end: T,
    steps: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, steps: T) -> Self {
        StepIterator { beg, end, steps }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: AddAssign + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        };

        let next = self.beg;

        self.beg += self.steps;

        Some(next)
    }
}
