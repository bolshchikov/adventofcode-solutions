mod moves;
mod input;

fn main() {
    let mut input: String = input::get_line();
    let moves: Vec<String> = input::get_moves();

    for m in &moves {
        let s: Vec<_> = m.chars().collect();

        match s.get(0).unwrap() {
            &'s' => {
                input = moves::spin(input, input::get_spin_params(m));
            }
            &'x' => {
                let (pos_1, pos_2) = input::get_exchange_params(m);
                input = moves::exchange(input, pos_1, pos_2);
            }
            &'p' => {
                let (pos_1, pos_2) = input::get_partner_params(m);
                input = moves::partner(input, pos_1, pos_2);
            }
            _ => panic!("unknown move"),
        };
    }

    println!("{:?}", input);
}
