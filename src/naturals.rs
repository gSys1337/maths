pub mod iter;
use crate::naturals::iter::BitIter;
use Natural::{Big, Small};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Natural {
    Small(usize),
    Big(Vec<usize>),
}
impl Natural {
    pub fn new(n: impl Into<Natural>) -> Self {
        n.into()
    }
    pub fn is_small(&self) -> bool {
        matches!(self, Small(_))
    }
    pub fn is_big(&self) -> bool {
        matches!(self, Big(_))
    }
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
                Ordering::Less => Small(0),
            }
        } else {
            self
        }
    }
    pub fn last_hunk(&self) -> usize {
        match self {
            Small(inner) => *inner,
            Big(hunks) => *hunks.last().expect("Natural::Big(_) is never empty"),
        }
    }
    pub fn first_hunk(&self) -> usize {
        match self {
            Small(hunk) => *hunk,
            Big(hunks) => *hunks.first().expect("Big(_) is never empty"),
        }
    }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            Small(_) => 1,
            Big(hunks) => hunks.len(),
        }
    }
    pub const fn max_hunks() -> usize {
        (isize::MAX >> size_of::<usize>().ilog2()).unsigned_abs()
    }
    #[allow(non_snake_case)]
    pub fn BITS(&self) -> Natural {
        (Small(usize::BITS as usize) * Small(self.len())
            - Natural::new(self.last_hunk().leading_zeros()))
        .expect("This equation always equals 0 or higher")
    }
    pub fn bits(&self) -> BitIter {
        BitIter::from(self.clone())
    }
}
/// Constants
impl Natural {
    pub const ZERO: Natural = Small(0);
    pub const ONE: Natural = Small(1);
    pub const TWO: Natural = Small(2);
    pub const MAX_SMALL: Natural = Small(usize::MAX);
}

impl PartialEq<Self> for Natural {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Small(lhs), Small(rhs)) => lhs.eq(rhs),
            (Big(lhs), Big(rhs)) => {
                lhs.iter().zip(rhs).all(|(l, r)| l == r) && lhs.len() == rhs.len()
            }
            _ => false,
        }
    }
}

impl Eq for Natural {}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Small(lhs), Small(rhs)) => lhs.cmp(rhs),
            (Big(_lhs), Big(_rhs)) => {
                todo!()
            }
            (Small(_), Big(_)) => Ordering::Less,
            (Big(_), Small(_)) => Ordering::Greater,
        }
    }
    // fn max(self, _other: Self) -> Self
    // where
    //     Self: Sized,
    // {
    //     todo!()
    // }
    // fn min(self, _other: Self) -> Self
    // where
    //     Self: Sized,
    // {
    //     todo!()
    // }
    // fn clamp(self, _min: Self, _max: Self) -> Self
    // where
    //     Self: Sized,
    // {
    //     todo!()
    // }
}

impl PartialOrd<Self> for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    // fn lt(&self, _other: &Self) -> bool {
    //     todo!()
    // }
    // fn le(&self, _other: &Self) -> bool {
    //     todo!()
    // }
    // fn gt(&self, _other: &Self) -> bool {
    //     todo!()
    // }
    // fn ge(&self, _other: &Self) -> bool {
    //     todo!()
    // }
}

// impl Sum<Self> for Natural {
//     fn sum<I: Iterator<Item = Self>>(_iter: I) -> Self {
//         todo!()
//     }
// }

// impl Product<Self> for Natural {
//     fn product<I: Iterator<Item = Self>>(_iter: I) -> Self {
//         todo!()
//     }
// }

// impl Hash for Natural {
//     fn hash<H: Hasher>(&self, _state: &mut H) {
//         todo!()
//     }
// }

// TODO add Traits from ops module
