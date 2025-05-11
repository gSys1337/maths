use crate::naturals::Natural;
use crate::naturals::Natural::{Big, Small};
use std::str::FromStr;

macro_rules! impl_from_small_primitive {
    ($($t:ty)*) => ($(
        impl From<$t> for Natural {
            fn from(value: $t) -> Self {
                Small(value as usize)
            }
        }
    )*)
}

macro_rules! impl_from_unsigned_primitive {
    ($($t:ty)*) => ($(
        impl TryFrom<$t> for Natural {
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
        impl From<$t> for Natural {
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
        impl From<$t> for Natural {
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
        impl From<$t> for Natural {
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

impl FromIterator<usize> for Natural {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        Big(iter.into_iter().collect()).trim()
    }
}

impl FromStr for Natural {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(target_pointer_width = "16")]
        let save_decimal_digits = 4usize;
        #[cfg(target_pointer_width = "32")]
        let save_decimal_digits = 9usize;
        #[cfg(target_pointer_width = "64")]
        let save_decimal_digits = 19usize;
        let shift = s.len() % save_decimal_digits;
        let (mut prefix, mut remainder) = s.split_at_checked(shift).ok_or(())?;
        let mut n = Small(prefix.parse::<usize>().map_err(|_| ())?);
        while !remainder.is_empty() {
            (prefix, remainder) = remainder.split_at_checked(save_decimal_digits).ok_or(())?;
            n = n * Small(10).pow(Small(save_decimal_digits));
            n = n + Small(prefix.parse::<usize>().map_err(|_| ())?);
        }
        Ok(n)
    }
}

impl TryInto<usize> for Natural {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Small(inner) => Ok(inner),
            Big(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn from_str_test() {
        use crate::naturals::Natural;
        assert_eq!(Natural::from_str("100"), Natural::try_from(100));
        assert_eq!(Natural::from_str("100000000000000000000"), Ok(Natural::from(100000000000000000000u128)));
        assert_eq!(Natural::from_str("1000000000000000000900"), Ok(Natural::from(1000000000000000000900u128)));
        assert_eq!(Natural::from_str("9345623510000000000234500000000900"), Natural::try_from(9345623510000000000234500000000900i128));
        assert_eq!(Natural::from_str(&u128::MAX.to_string()), Ok(Natural::Big(vec![usize::MAX, usize::MAX])));
        assert_eq!(Natural::from_str("340282366920938463463374607431768211456"), Ok(Natural::Big(vec![0, 0, 1])));
    }
}
