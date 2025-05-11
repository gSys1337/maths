use crate::naturals::Natural;
use crate::naturals::Natural::{Big, Small};
use crate::naturals::iter::BitIter;
use std::cmp::Ordering;
use std::iter;
use std::iter::repeat_n;
use std::ops::{Add, Div, Mul, Rem, Shl, Sub};

impl Add<Natural> for Natural {
    type Output = Natural;
    fn add(self, rhs: Natural) -> Natural {
        let mut lhs = match self {
            Small(hunk) => vec![hunk],
            Big(hunks) => hunks,
        };
        let mut rhs = match rhs {
            Small(hunk) => vec![hunk],
            Big(hunks) => hunks,
        };
        match lhs.len().cmp(&rhs.len()) {
            Ordering::Less => lhs.extend(repeat_n(0, rhs.len() - lhs.len())),
            Ordering::Greater => rhs.extend(repeat_n(0, lhs.len() - rhs.len())),
            Ordering::Equal => {}
        }
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
        Big(new).trim()
    }
}
#[cfg(test)]
mod add_test {
    use crate::naturals::Natural::{Big, Small};
    #[test]
    fn big_lhs_rhs_0() {
        let lhs = Big(vec![1, 1, 1]);
        let rhs = Big(vec![1, 1]);
        let expected = Big(vec![2, 2, 1]);
        assert_eq!(expected, lhs + rhs);
    }
    #[test]
    fn big_lhs_rhs_1() {
        let lhs = Big(vec![1, 1]);
        let rhs = Big(vec![1, 1, 2]);
        let expected = Big(vec![2, 2, 2]);
        assert_eq!(expected, lhs + rhs);
    }
    #[test]
    fn big_lhs_rhs_2() {
        let lhs = Big(vec![1, usize::MAX, 1, 1]);
        let rhs = Big(vec![1, 1]);
        let expected = Big(vec![2, 0, 2, 1]);
        assert_eq!(expected, lhs + rhs);
    }
    #[test]
    fn big_lhs_rhs_3() {
        let lhs = Small(0);
        let rhs = Small(0);
        let expected = Small(0);
        assert_eq!(expected, lhs + rhs);
    }
}

impl Sub<Natural> for Natural {
    type Output = Option<Natural>;
    fn sub(self, rhs: Natural) -> Self::Output {
        match (self, rhs) {
            (Small(lhs), Small(rhs)) => Some(Small(lhs.checked_sub(rhs)?)),
            (Small(_), Big(_)) => None,
            (Big(lhs), Big(rhs)) => {
                if lhs.len() < rhs.len() {
                    None
                } else {
                    let mut carry = 0usize;
                    let new: Vec<usize> = lhs
                        .iter()
                        .zip(rhs.iter().chain(iter::repeat(&0usize)))
                        .map(|(lhs, rhs)| {
                            let (diff, overflow0) = lhs.overflowing_sub(*rhs);
                            let (diff, overflow1) = diff.overflowing_sub(carry);
                            carry = if overflow0 | overflow1 { 1 } else { 0 };
                            diff
                        })
                        .collect();
                    if carry == 0 { Some(Big(new)) } else { None }
                }
            }
            (lhs, Small(rhs)) => lhs.sub(Big(vec![rhs])),
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
                #[allow(clippy::suspicious_arithmetic_impl)]
                prod_sum.push(n.shift_up(idx_l + idx_r));
            }
        }
        #[allow(clippy::suspicious_arithmetic_impl)]
        prod_sum
            .iter_mut()
            .reduce(|acc, next| {
                *acc = acc.clone() + next.to_owned();
                acc
            })
            .unwrap()
            .clone()
    }
}

impl Natural {
    fn shift_up(self, n: usize) -> Natural {
        if n == 0 {
            return self;
        } else if self.len().checked_add(n).is_none() {
            panic!("Shifting up would create an inner vec too big for indexing");
        }
        let mut shifted = [0usize].repeat(n);
        match self {
            Small(single) => shifted.push(single),
            Big(vec) => shifted.extend(vec),
        }
        Big(shifted)
    }
    /// Exponentiation with Horner's method.
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Horner%27s_method)
    /// or [here](https://www.brainkart.com/article/Horner---s-Rule-and-Binary-Exponentiation_8034/)
    /// for explanation.
    ///
    /// In particular the paragraph **Binary Exponentiation** with this image.
    /// ![Image](https://img.brainkart.com/imagebk9/F5M5pU5.jpg)
    pub fn pow(self, exp: Natural) -> Natural {
        BitIter::from(exp).fold(Natural::ONE, |acc, bit| {
            let acc = acc.clone() * acc;
            if bit { acc * self.clone() } else { acc }
        })
    }
}

#[cfg(test)]
mod pow_tests {
    use crate::naturals::Natural;
    use crate::naturals::Natural::Small;
    #[test]
    fn pow_0() {
        assert_eq!(
            Natural::TWO.pow(Small(100)),
            "1267650600228229401496703205376".parse().unwrap()
        );
    }
    #[test]
    fn pow_1() {
        assert_eq!(Small(5).pow(Small(5)), Small(3125));
    }
}

impl Div<Natural> for Natural {
    type Output = Natural;
    // TODO implement proper long division
    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs, Natural::ZERO, "division by zero");
        let mut remainder = self;
        let mut quotient = Natural::ZERO;
        while remainder >= rhs {
            remainder = (remainder - rhs.clone()).expect("It was tested remainder > rhs");
            quotient = quotient + Natural::ONE;
        }
        quotient
    }
}

impl Rem<Natural> for Natural {
    type Output = Natural;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Small(lhs), Small(rhs)) => Small(lhs.rem(rhs)),
            (Big(_lhs), Small(_rhs)) => todo!(),
            (Small(lhs), Big(_)) => Small(lhs),
            (Big(_lhs), Big(_rhs)) => todo!(),
        }
    }
}

impl Shl<Natural> for Natural {
    type Output = Natural;
    fn shl(mut self, rhs: Natural) -> Self::Output {
        let secure_shift = Natural::new((isize::MAX >> size_of::<usize>().ilog2()).unsigned_abs())
            * Natural::new(usize::BITS);
        assert!(
            rhs < secure_shift,
            "Shifting {secure_shift:?} or more times is impossible as it creates more then isize::MAX bytes"
        );
        let hunk_size = Small(usize::BITS as usize);
        if rhs >= hunk_size {
            let new_hunks = rhs.clone() / hunk_size.clone();
            assert!(new_hunks.is_small());
            assert!(
                new_hunks.first_hunk() + self.len() < Natural::max_hunks(),
                "Shifting {:?} times would create more then isize::MAX bytes",
                rhs.clone()
            );
            self = self.shift_up(new_hunks.first_hunk());
        }
        let rhs: usize = (rhs % hunk_size).first_hunk();
        if rhs == 0 {
            self
        } else {
            match self {
                Small(hunk) => Big(vec![hunk << rhs, hunk >> (usize::BITS as usize - rhs)]).trim(),
                Big(hunks) => {
                    let mut upper: Vec<usize> = hunks.iter().map(|x| x << rhs).collect();
                    upper.push(0);
                    let mut lower: Vec<usize> = vec![0];
                    lower.extend(hunks.iter().map(|x| x >> (usize::BITS - (rhs as u32))));
                    Big(upper
                        .iter()
                        .zip(lower)
                        .map(|(upper, lower)| upper + lower)
                        .collect())
                    .trim()
                }
            }
        }
    }
}
#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod shl_tests {
    use crate::naturals::Natural;
    use crate::naturals::Natural::Small;

    #[test]
    fn shl_inside_small() {
        let expected = Natural::Small(0);
        let output = expected.clone() << Small(1);
        assert_eq!(expected, output);

        let expected = Natural::Small(8);
        let output = Natural::Small(1) << Small(3);
        assert_eq!(expected, output);

        let expected = Natural::Small(0b1010001010000000);
        let output = Natural::Small(0b101000101) << Small(7);
        assert_eq!(expected, output);
    }
    #[test]
    fn shl_small_into_big() {
        let expected = Natural::Big(vec![0, 1]);
        let output = Natural::Small(1) << Small(64);
        assert_eq!(expected, output);

        let expected = Natural::Big(vec![0, 8]);
        let output = Natural::Small(1) << Small(64 + 3);
        assert_eq!(expected, output);

        let expected = Natural::Big(vec![0, 0b101000101]);
        let output = Natural::Small(0b101000101) << Small(64);
        assert_eq!(expected, output);

        let expected = Natural::Big(vec![0, 0b1010001010000000]);
        let output = Natural::Small(0b101000101) << Small(64 + 7);
        assert_eq!(expected, output);
    }
}
