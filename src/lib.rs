pub mod real;
pub mod integer;
pub mod naturals;
#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod tests;

#[test]
#[cfg(target_pointer_width = "32")]
fn integer_from_u64() {
    use crate::integer::Integer;
    assert_eq!(Integer::new(u64::MIN).parts, vec![0usize, 0usize]);
    assert_eq!(Integer::new(u64::MAX).parts, vec![usize::MAX, usize::MAX]);
    assert_eq!(Integer::new(42u64).parts, vec![42usize, 0usize]);
}


#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Sign {
    Negative,
    Positive,
}
