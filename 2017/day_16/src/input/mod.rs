use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests;

const INPUT_FILE_NAME: &str = "input.txt";

pub fn get_example_line() -> String {
    String::from("abcde")
}

pub fn get_line() -> String {
    String::from("abcdefghijklmnop")
}

pub fn get_example_moves() -> Vec<String> {
    vec![
        String::from("s1"),
        String::from("x3/4"),
        String::from("pe/b"),
    ]
}

pub fn get_moves() -> Vec<String> {
    let mut f = File::open(INPUT_FILE_NAME).expect("file not found");
    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read input");
    let moves: Vec<String> = input.split(",").map(|s| s.to_string()).collect();

    moves

}

pub fn get_spin_params(spin_move: &String) -> i32 {
    spin_move[1..].parse::<i32>().unwrap()
}

pub fn get_exchange_params(exchange_move: &String) -> (i32, i32) {
    let params: Vec<&str> = exchange_move[1..].split("/").collect();
    (
        params[0].parse::<i32>().unwrap(),
        params[1].parse::<i32>().unwrap(),
    )
}

pub fn get_partner_params(partner_move: &String) -> (char, char) {
    let params: Vec<_> = partner_move[1..].split("/").collect();
    let char_1: Vec<char> = params[0].chars().collect();
    let char_2: Vec<char> = params[1].chars().collect();
    (char_1[0], char_2[0])
}
