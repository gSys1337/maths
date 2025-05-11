use crate::naturals::Natural;
use crate::naturals::Natural::{Big, Small};

pub struct BitIter {
    hunks: Vec<usize>,
    idx: Option<usize>,
    mask: usize,
}

impl BitIter {
    fn new(n: Natural) -> Self {
        let hunks = match n.trim() {
            Small(hunk) => vec![hunk],
            Big(hunks) => hunks,
        };
        let idx = Some(hunks.len() - 1);
        #[allow(clippy::unnecessary_literal_unwrap)]
        let mask = 1usize<<hunks[idx.unwrap()].ilog2();
        Self { hunks, idx, mask }
    }
}

impl From<Natural> for BitIter {
    fn from(value: Natural) -> Self {
        BitIter::new(value)
    }
}

impl Iterator for BitIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.idx {
            let result = self.hunks[idx] & self.mask != 0;
            self.mask = match self.mask {
                1 => {
                    self.idx = idx.checked_sub(1);
                    isize::MAX.unsigned_abs() + 1
                }
                _ => self.mask >> 1,
            };
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::naturals::iter::BitIter;
    use crate::naturals::Natural::Small;

    #[test]
    fn test_bit_iter() {
        let bits = BitIter::from(Small(100)).map(|b| if b { 1 } else { 0 }).collect::<Vec<_>>();
        assert_eq!(bits, vec![1,1,0,0,1,0,0]);
    }
}
