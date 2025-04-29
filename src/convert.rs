use crate::naturals::Naturals;
use crate::naturals::Naturals::{Big, Small};
use std::str::FromStr;

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

impl FromIterator<usize> for Naturals {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        Big(iter.into_iter().collect()).trim()
    }
}

impl FromStr for Naturals {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        let mut r;
        let mut n = Naturals::try_from(1)?;
        let mut exp = 0;
        while s.len() != 0 {
            let delta = s.len().checked_sub(39usize);
            match delta {
                None => (s, r) = ("", s),
                Some(delta) => (s, r) = s.split_at_checked(delta).ok_or(())?,
            }
            let mut r = Naturals::new(r.parse::<u128>().map_err(|_| ())?);
            r = r.pow(39usize.pow(exp));
            n *= &Naturals::new(r);
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_str() {
        use crate::naturals::Naturals;
        assert_eq!("100".parse::<Naturals>().unwrap(), Naturals::try_from(100).unwrap());
        assert_eq!("100000000000000000000".parse::<Naturals>().unwrap(), Naturals::try_from(100000000000000000000i128).unwrap());
    }
}
