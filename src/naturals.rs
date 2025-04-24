use std::cmp::Ordering;
use Naturals::{Small, Big};
pub enum Naturals {
    Small(usize),
    Big(Vec<usize>),
}

impl Naturals {
    pub fn new(n: impl Into<Naturals>) -> Self {
        n.into()
    }
    fn trim(self) -> Self {
        if let Big(mut inner) = self {
            while inner.pop_if(|x| 0usize.eq(x)).is_some() {}
            if inner.len() > 1 {
                Big(inner)
            } else if inner.len() == 1 {
                // TODO Check if this code is really save!
                // This code should be safe because we just checked the bounds.
                // Unsafety could come from dereferencing.
                // Depending if the pointed to usize gets copied or not.
                // If not then the pointed to memory is maybe unallocated in case inner drops.
                unsafe { Small(*inner.get_unchecked(0)) }
            } else {
                panic!("Calculations should always provide non zero Naturals")
            }
        } else {
            self
        }
    }
}

impl PartialEq<Self> for Naturals {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Small(lhs), Small(rhs)) => lhs.eq(rhs),
            (Big(lhs), Big(rhs)) => {
                lhs.iter().zip(rhs).all(|(l, r)| l == r) &&  lhs.len() == rhs.len()
            },
            (Small(_lhs), Big(_rhs)) => unimplemented!(),  // It's assumed that this case does not happen.
            (Big(_lhs), Small(_rhs)) => unimplemented!(),  // If Natural fits into usize then it's also stored accordingly.
        }
    }
}

impl Eq for Naturals {}

impl Ord for Naturals {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Small(lhs), Small(rhs)) => lhs.cmp(rhs),
            (Big(lhs), Big(rhs)) => {
                todo!()
            },
            (Small(_lhs), Big(_rhs)) => unimplemented!(),  // It's assumed that this case does not happen.
            (Big(_lhs), Small(_rhs)) => unimplemented!(),  // If Natural fits into usize then it's also stored accordingly.
        }
    }
}

impl PartialOrd<Self> for Naturals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_from_small_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Naturals {
            fn from(value: $t) -> Self {
                Small(value as usize)
            }
        }
    )*)
}

macro_rules! impl_from_unsigned_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Naturals {
            fn from(value: $t) -> Self {
                if value <= 0 {
                    panic!("Cannot convert negative value into natural number.");
                }
                value.unsigned_abs().into()
            }
        }
    )*)
}

#[cfg(target_pointer_width = "64")]
macro_rules! impl_from_big_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Naturals {
            fn from(value: $t) -> Self {
                if value <= usize::MAX as $t {
                    Small(value as usize)
                } else {
                    let mut part: usize = value.rem_euclid(0x10000000000000000) as usize;
                    let mut remaining: $t = value >> 64;
                    let mut parts: Vec<usize> = vec![part];
                    while remaining != 0 {
                        part = remaining.rem_euclid(0x10000000000000000) as usize;
                        remaining = remaining >> 64;
                        parts.push(part);
                    }
                    Big(parts)
                }
            }
        }
    )*)
}

#[cfg(target_pointer_width = "32")]
macro_rules! impl_from_big_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Naturals {
            fn from(value: $t) -> Self {
                if value <= usize::MAX as $t {
                    Small(value as usize)
                } else {
                    let mut part: usize = value.rem_euclid(0x100000000) as usize;
                    let mut remaining: $t = value >> 32;
                    let mut parts: Vec<usize> = vec![part];
                    while remaining != 0 {
                        part = remaining.rem_euclid(0x100000000) as usize;
                        remaining = remaining >> 32;
                        parts.push(part);
                    }
                    Big(parts)
                }
            }
        }
    )*)
}

#[cfg(target_pointer_width = "16")]
macro_rules! impl_from_big_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Naturals {
            fn from(value: $t) -> Self {
                if value <= usize::MAX as $t {
                    Small(value as usize)
                } else {
                    let mut part: usize = value.rem_euclid(0x10000) as usize;
                    let mut remaining: $t = value >> 16;
                    let mut parts: Vec<usize> = vec![part];
                    while remaining != 0 {
                        part = remaining.rem_euclid(0x10000) as usize;
                        remaining = remaining >> 16;
                        parts.push(part);
                    }
                    Big(parts)
                }
            }
        }
    )*)
}

#[cfg(target_pointer_width = "64")]
macro_rules! impl_from_primitive {
    () => {
        impl_from_small_primitive! { usize u8 u16 u32 u64 }
        impl_from_big_primitive! { u128 }

    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! impl_from_primitive {
    () => {
        impl_from_small_primitive! { usize u8 u16 u32 }
        impl_from_big_primitive! {  u64 u128 }
    };
}

#[cfg(target_pointer_width = "16")]
macro_rules! impl_from_primitive {
    () => {
        impl_from_small_primitive! { usize u8 u16 }
        impl_from_big_primitive! {  u32 u64 u128 }
    };
}

impl_from_primitive! {}
impl_from_unsigned_primitive! { isize i8 i16 i32 i64 i128 }
