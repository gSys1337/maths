use crate::Sign;
use std::ops::ShrAssign;

pub struct Integer {
    pub(crate) sign: Option<Sign>,
    pub(crate) parts: Vec<usize>,
}

impl Integer {
    pub fn new(n: impl Into<Integer>) -> Self {
        n.into()
    }
}

macro_rules! from_unsigned_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Integer {
            fn from(value: $t) -> Self {
                let sign =
                    if value == 0 { None }
                    else { Some(Sign::Positive) };
                if size_of::<$t>() <= size_of::<usize>() {
                    Self { sign, parts: vec![value as usize] }
                } else {
                    let mut value = value;
                    let parts: Vec<usize> = (0..<$t>::BITS.div_euclid(usize::BITS))
                        .map(|_| {
                        let part = value.rem_euclid(usize::BITS as $t);
                        value.shr_assign(usize::BITS as $t);
                        part as usize
                    }).collect();
                    Self { sign, parts }
                }
            }
        }
    )*)
}

from_unsigned_primitive! { u8 u16 u32 u64 u128 usize }

macro_rules! from_signed_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Integer {
            fn from(value: $t) -> Self {
                let sign =
                    if value == 0 { None }
                    else if value < 0 { Some(Sign::Negative) }
                    else { Some(Sign::Positive) };
                let value = value.unsigned_abs();
                if size_of::<$t>() <= size_of::<usize>() {
                    let mut value = value as usize;
                    if let Some(Sign::Negative) = sign {
                        value = value as usize ^ usize::MAX;
                    }
                    Self { sign, parts: vec![value as usize] }
                } else {
                    let mut value = value as usize;
                    if let Some(Sign::Negative) = sign {
                        value = value as usize ^ usize::MAX;
                    }
                    let parts: Vec<usize> = (0..<$t>::BITS.div_euclid(usize::BITS))
                        .map(|_| {
                        dbg!(value);
                        let part = dbg!(value.rem_euclid(usize::BITS.try_into().unwrap()));
                        value.shr_assign(usize::BITS as $t);
                        part as usize
                    }).collect();
                    Self { sign, parts }
                }
            }
        }
    )*)
}

from_signed_primitive! { i8 i16 i32 i64 i128 isize }

