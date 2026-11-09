use std::ops::Add;

pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    pub const fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { beg, end, step }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        }
        let current = self.beg;
        self.beg = self.beg + self.step;
        Some(current)
    }
}