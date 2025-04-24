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
    assert_eq!(Integer::new(i128::MIN).parts, vec![usize::MAX, usize::MAX]);
    assert_eq!(Integer::new(i128::MIN).sign, Some(Sign::Negative));
    assert_eq!(
        Integer::new(i128::MAX).parts,
        vec![usize::MAX - 1, usize::MAX]
    );
    assert_eq!(Integer::new(i128::MAX).sign, Some(Sign::Negative));
    assert_eq!(Integer::new(42i128).parts, vec![42usize, 0usize]);
}

#[test]
#[cfg(target_pointer_width = "64")]
fn naturals_from_primitive() {
    use crate::naturals::Naturals;
    if let Naturals::Small(small) = Naturals::from(8u8) {
        assert_eq!(8usize, small);
    } else {
        assert!(false, "Type u8 is small enough to stay on Stack.");
    }
    if let Naturals::Small(_) = Naturals::from((u8::MAX as u16) << 3) {
        assert!(true);
    } else {
        assert!(false, "Type u16 is small enough to stay on Stack.");
    }
    if let Naturals::Small(small) = Naturals::from(2147483647u32) {
        assert_eq!(2147483647usize, small);
    } else {
        assert!(false, "Type u32 is small enough to stay on Stack.");
    }
}
#[test]
#[cfg(target_pointer_width = "64")]
fn naturals_from_u128() {
    use crate::naturals::Naturals;
    if let Naturals::Small(small) = Naturals::from(2147483647u128) {
        assert_eq!(2147483647usize, small);
    } else {
        assert!(false, "Given value of type u128 is still small enough to fit into usize and can therefore stay on Stack.");
    }
    if let Naturals::Big(parts) = Naturals::from(u128::MAX) {
        assert_eq!(parts, vec![usize::MAX, usize::MAX]);
    } else {
        assert!(false, "This value is not small enough to fit on the stack. Is is split into 2 parts.");
    }
    if let Naturals::Big(parts) = Naturals::from((usize::MAX as u128) << 32) {
        assert_eq!(parts, vec![0xFFFFFFFF00000000usize, 0xFFFFFFFFusize]);
    } else {
        assert!(false, "This value is not small enough to fit on the stack. Is is split into 2 parts.");
    }
}

// TODO write tests for conversion from signed primitives
