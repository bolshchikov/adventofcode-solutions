use dups::*;

#[test]
fn should_have_dups() {
    let arg = String::from("tes tes");
    assert!(has_dups(arg));
}

#[test]
fn should_not_have_dups() {
    let arg = String::from("test_1 test_2");
    assert!(!has_dups(arg));
}
