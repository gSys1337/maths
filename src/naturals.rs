use Natural::{Big, Small};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};

#[derive(Debug, Clone)]
pub enum Natural {
    Small(usize),
    Big(Vec<usize>),
    // TODO add lazy Naturals when converting FromIterator
    // Lazy(Vec<usize>, Box<dyn Iterator<Item=usize>>),
}

// TODO
pub enum NonZeroNaturals {}

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
            (Small(_lhs), Big(_rhs)) => Ordering::Less,
            (Big(_lhs), Small(_rhs)) => Ordering::Greater,
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

impl PartialOrd<Self> for Natural {
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

impl Sum<Self> for Natural {
    fn sum<I: Iterator<Item = Self>>(_iter: I) -> Self {
        todo!()
    }
}

impl Product<Self> for Natural {
    fn product<I: Iterator<Item = Self>>(_iter: I) -> Self {
        todo!()
    }
}

impl Default for Natural {
    fn default() -> Self {
        Small(0usize)
    }
}

impl Hash for Natural {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

// TODO add Traits from ops module
