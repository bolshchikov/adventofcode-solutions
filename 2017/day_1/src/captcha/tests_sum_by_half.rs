use captcha::*;

#[test]
fn should_be_6() {
    let numbers: Vec<u32> = vec![1, 2, 1, 2];
    assert_eq!(6, sum_by_half(&numbers));
}

#[test]
fn should_be_0() {
    let numbers: Vec<u32> = vec![1, 2, 2, 1];
    assert_eq!(0, sum_by_half(&numbers));
}

#[test]
fn should_be_4() {
    let numbers: Vec<u32> = vec![1, 2, 3, 4, 2, 5];
    assert_eq!(4, sum_by_half(&numbers));
}
