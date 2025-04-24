use std::cmp::Ordering;
use Naturals::{Big, Small};

#[derive(Debug)]
pub enum Naturals {
    Small(usize),
    Big(Vec<usize>),
}

impl Naturals {
    pub fn new(n: impl Into<Naturals>) -> Self {
        n.into()
    }
    pub fn is_small(&self) -> bool {
        matches!(self, Small(_))
    }
    pub fn is_big(&self) -> bool {
        matches!(self, Big(_))
    }
    #[allow(unused)]
    pub(crate) fn trim(self) -> Self {
        if let Big(mut inner) = self {
            while inner.pop_if(|x| 0usize.eq(x)).is_some() {}
            match inner.len().cmp(&1) {
                Ordering::Greater => Big(inner),
                Ordering::Equal => {
                    // This code should be safe because we just checked the bounds.
                    // Unsafety could come from dereferencing.
                    // Depending on if the pointed to usize gets copied or not.
                    // If not then the pointed to memory is maybe unallocated in case inner drops.
                    // This is just to make trim(...) faster.
                    unsafe { Small(*inner.get_unchecked(0)) }
                }
                // Should they?
                Ordering::Less => panic!("Calculations should always provide non zero Big(...)"),
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
                lhs.iter().zip(rhs).all(|(l, r)| l == r) && lhs.len() == rhs.len()
            }
            (Small(_lhs), Big(_rhs)) => unimplemented!(), // It's assumed that this case does not happen.
            (Big(_lhs), Small(_rhs)) => unimplemented!(), // If Natural fits into usize then it's also stored accordingly.
        }
    }
}

impl Eq for Naturals {}

impl Ord for Naturals {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Small(lhs), Small(rhs)) => lhs.cmp(rhs),
            (Big(_lhs), Big(_rhs)) => {
                todo!()
            }
            (Small(_lhs), Big(_rhs)) => unimplemented!(), // It's assumed that this case does not happen.
            (Big(_lhs), Small(_rhs)) => unimplemented!(), // If Natural fits into usize then it's also stored accordingly.
        }
    }
    fn max(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    fn min(self, _other: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

impl PartialOrd<Self> for Naturals {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn le(&self, _other: &Self) -> bool {
        todo!()
    }
    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn ge(&self, _other: &Self) -> bool {
        todo!()
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
        impl TryFrom<$t> for Naturals {
            type Error = ();
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if value <= 0 {
                    Err(())
                } else {
                    Ok(value.unsigned_abs().into())
                }
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
                        remaining >>= 64;
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
                        remaining >>= 32;
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
                        remaining >>= 16;
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
