#[test]
fn integer_from_u8() {
    use crate::integer::Integer;
    assert_eq!(Integer::new(u8::MIN).parts, vec![u8::MIN as usize]);
    assert_eq!(Integer::new(u8::MAX).parts, vec![u8::MAX as usize]);
    assert_eq!(Integer::new(42u8).parts, vec![42u8 as usize]);
}

#[test]
#[cfg(target_pointer_width = "64")]
fn integer_from_u64() {
    use crate::integer::Integer;
    assert_eq!(Integer::new(u64::MIN).parts, vec![usize::MIN]);
    assert_eq!(Integer::new(u64::MAX).parts, vec![usize::MAX]);
    assert_eq!(Integer::new(42u64).parts, vec![42usize]);
}

#[test]
#[cfg(target_pointer_width = "32")]
fn integer_from_u64() {
    use crate::integer::Integer;
    assert_eq!(Integer::new(u64::MIN).parts, vec![0usize, 0usize]);
    assert_eq!(Integer::new(u64::MAX).parts, vec![usize::MAX, usize::MAX]);
    assert_eq!(Integer::new(42u64).parts, vec![42usize, 0usize]);
}

#[test]
#[cfg(target_pointer_width = "64")]
fn integer_from_i128() {
    use crate::Sign;
    use crate::integer::Integer;
    dbg!(usize::MAX);
    assert_eq!(Integer::new(i128::MIN).parts, vec![usize::MAX, usize::MAX]);
    assert_eq!(Integer::new(i128::MIN).sign, Some(Sign::Negative));
    assert_eq!(Integer::new(i128::MAX).parts, vec![usize::MAX - 1, usize::MAX]);
    assert_eq!(Integer::new(i128::MAX).sign, Some(Sign::Negative));
    assert_eq!(Integer::new(42i128).parts, vec![42usize, 0usize]);
}
