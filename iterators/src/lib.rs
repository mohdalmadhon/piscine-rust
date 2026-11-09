pub fn collatz(n: u64) -> Option<usize> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let last = std::iter::successors(Some(n), |&x| match x {
        1 => None,
        x if x % 2 == 0 => Some(x / 2),
        x => x.checked_mul(3)?.checked_add(1),
    })
    .inspect(|_| steps += 1)
    .last()?;

    (last == 1).then_some(steps - 1)
}