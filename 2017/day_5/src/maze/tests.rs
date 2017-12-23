use maze::*;

#[test]
fn should_escape_in_5_steps() {
    let numbers: Vec<i32> = vec![0, 3, 0, 1, -3];
    assert_eq!(5, escape(&numbers));
}

#[test]
fn should_escape_in_10_steps() {
    let numbers: Vec<i32> = vec![0, 3, 0, 1, -3];
    assert_eq!(10, escape2(&numbers));
}
