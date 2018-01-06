use passphrase::*;

#[test]
fn should_have_dups() {
    let arg = String::from("tes tes");
    assert!(has_dups(&arg));
}

#[test]
fn should_not_have_dups() {
    let arg = String::from("test_1 test_2");
    assert!(!has_dups(&arg));
}

#[test]
fn should_have_anagram() {
    let arg = String::from("tes set");
    assert!(has_anagram(&arg));
}

#[test]
fn should_not_have_anagram() {
    let arg = String::from("tes_1 set");
    assert!(!has_anagram(&arg));
}

