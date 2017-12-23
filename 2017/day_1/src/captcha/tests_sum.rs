use captcha::*;

#[test]
fn should_be_3() {
    let numbers: Vec<u32> = vec![1, 1, 2, 2];
    assert_eq!(3, sum(&numbers));
}

#[test]
fn should_be_4() {
    let numbers: Vec<u32> = vec![1, 1, 1, 1];
    assert_eq!(4, sum(&numbers));
}

#[test]
fn should_be_0() {
    let numbers: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(0, sum(&numbers));
}
