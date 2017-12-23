use std::fs::File;
use std::io::prelude::*;

mod captcha;

const INPUT_FILE_NAME: &str = "input.txt";

fn input_to_arr() -> Vec<u32> {
    let mut f = File::open(INPUT_FILE_NAME).expect("file not found");
    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read input");
    let mut numbers: Vec<u32> = Vec::new();

    let input = input.trim();

    for c in input.chars() {
        let num = c.to_digit(10).expect("Failed to parse");
        numbers.push(num);
    }

    numbers
}

fn main() {
    let numbers = input_to_arr();
    let res = captcha::sum(&numbers);
    let res2 = captcha::sum_by_half(&numbers);

    println!("{}", res);
    println!("{}", res2);
}
