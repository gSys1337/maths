#[test]
fn integer_from_u8() {
    use crate::integer::Integer;
    assert_eq!(Integer::new(u8::MIN).parts, vec![u8::MIN as usize]);
    assert_eq!(Integer::new(u8::MAX).parts, vec![u8::MAX as usize]);
    assert_eq!(Integer::new(42u8).parts, vec![42u8 as usize]);
}
