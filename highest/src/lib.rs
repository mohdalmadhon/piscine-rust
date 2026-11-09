#[derive(Debug)]
pub struct Numbers<const N: usize> {
    numbers: [u32; N],
}

impl<const N: usize> Numbers<N> {
    pub const fn new(numbers: [u32; N]) -> Self {
        Numbers { numbers }
    }

    pub fn inner(&self) -> &[u32] {
        &self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Option<[u32; 3]> {
        if N < 3 {
            return None;
        }
        let mut sorted = self.numbers;
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        Some([sorted[0], sorted[1], sorted[2]])
    }
}