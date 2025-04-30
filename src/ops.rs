use crate::naturals::Natural;
use crate::naturals::Natural::{Big, Small};
use std::ops::{Add, Mul};

impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(self, rhs: Natural) -> Natural {
        match (self, rhs) {
            (Small(lhs), Small(rhs)) => {
                let (sum, overflow) = lhs.overflowing_add(rhs);
                if overflow {
                    Big(vec![sum, 1usize])
                } else {
                    Small(sum)
                }
            }
            (Big(lhs), Small(rhs)) => {
                let mut carry = rhs;
                let mut new: Vec<usize> = lhs
                    .iter()
                    .map(|part| {
                        let (sum, c) = part.overflowing_add(carry);
                        carry = if c { 1 } else { 0 };
                        sum
                    })
                    .collect();
                if carry > 0 {
                    new.push(carry);
                }
                Big(new)
            }
            (Small(lhs), rhs) => rhs.add(Small(lhs)),
            (Big(lhs), Big(rhs)) => {
                let mut carry = 0usize;
                let mut new: Vec<usize> = lhs
                    .iter()
                    .zip(rhs)
                    .map(|(lhs, rhs)| {
                        let (sum, overflow) = lhs.overflowing_add(rhs);
                        let sum = sum + carry;
                        carry = if overflow { 1 } else { 0 };
                        sum
                    })
                    .collect();
                if carry > 0 {
                    new.push(carry);
                }
                Big(new)
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(self, rhs: Natural) -> Self::Output {
        let lhs = match self {
            Small(lhs) => vec![lhs],
            Big(lhs) => lhs,
        };
        let rhs = match rhs {
            Small(rhs) => vec![rhs],
            Big(rhs) => rhs,
        };
        let mut prod_sum: Vec<Natural> = Vec::new();
        for (idx_l, item_l) in lhs.iter().enumerate() {
            for (idx_r, item_r) in rhs.iter().enumerate() {
                let n = Natural::new((*item_l as u128).mul(*item_r as u128));
                prod_sum.push(n.shift_up(idx_l + idx_r));
            }
        }
        prod_sum.iter().fold(Natural::new(0usize), |acc, next| { acc + next.to_owned() })
    }
}

impl Natural {
    fn shift_up(self, n: usize) -> Natural {
        if n == 0 {
            return self;
        }
        let mut shifted = [0usize].repeat(n);
        match self {
            Small(single) => shifted.push(single),
            Big(vec) => shifted.extend(vec),
        }
        Big(shifted)
    }
}
