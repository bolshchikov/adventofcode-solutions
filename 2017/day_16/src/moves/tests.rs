use moves;

#[test]
fn should_spin() {
    let input = String::from("abcde");
    let res = moves::spin(input, 3);
    assert_eq!(res, "cdeab");
}

#[test]
fn should_exchange() {
    let input = String::from("abcde");
    let res = moves::exchange(input, 0, 1);
    assert_eq!(res, "bacde");
}

#[test]
fn should_partner() {
    let input = String::from("abcde");
    let res = moves::partner(input, 'a', 'b');
    assert_eq!(res, "bacde");
}