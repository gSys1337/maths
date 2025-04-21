use crate::Sign;
use std::ops::ShrAssign;

pub struct Integer {
    pub(crate) sign: Sign,
    pub(crate) parts: Vec<usize>,
}

impl Integer {
    pub fn new(n: impl Into<Integer>) -> Self {
        n.into()
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        Self {
            sign: Sign::Positive,
            parts: vec![value],
        }
    }
}

impl From<u8> for Integer {
    fn from(value: u8) -> Self {
        if u8::BITS <= usize::BITS {
            Self {
                sign: Sign::Positive,
                parts: vec![value.into()],
            }
        } else {
            let mut value = value;
            let parts: Vec<usize> = (0..u8::BITS.div_euclid(usize::BITS))
                .map(|_| {
                    let part = value.rem_euclid(usize::BITS as u8);
                    value.shr_assign(usize::BITS as u8);
                    part as usize
                })
                .collect();
            Self {
                sign: Sign::Positive,
                parts,
            }
        }
    }
}
