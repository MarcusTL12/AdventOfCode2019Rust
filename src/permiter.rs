pub fn permutations<const N: usize>() -> Permutations<N> {
    let mut idxs = [0; N];
    for (i, x) in idxs.iter_mut().enumerate() {
        *x = i;
    }

    Permutations {
        idxs,
        swaps: [0; N],
        i: 0,
    }
}

pub struct Permutations<const N: usize> {
    idxs: [usize; N],
    swaps: [usize; N],
    i: usize,
}

impl<const N: usize> Iterator for Permutations<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs)
    }
}
