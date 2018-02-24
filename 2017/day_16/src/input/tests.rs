use input;

#[test]
fn should_return_example_input() {
    let res = input::get_example_line();
    assert_eq!(res, String::from("abcde"));
}

#[test]
fn should_return_parameters_for_spin_move() {
  let res = input::get_spin_params(&String::from("s1"));
  assert_eq!(res, 1);
}

#[test]
fn should_return_parameters_for_exchange_move() {
  let res = input::get_exchange_params(&String::from("x3/4"));
  assert_eq!(res, (3, 4));
}

#[test]
fn should_return_parameters_for_partner_move() {
  let res = input::get_partner_params(&String::from("pe/b"));
  assert_eq!(res, ('e', 'b'));
}