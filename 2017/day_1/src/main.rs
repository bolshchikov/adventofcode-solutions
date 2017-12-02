use std::fs::File;
use std::io::prelude::*;

const INPUT_FILE_NAME: &str = "input.txt";

fn str_to_int(input: String) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let input = input.trim();

    for c in input.chars() {
        let num = c.to_digit(10).expect("Failed to parse");
        numbers.push(num);
    }

    numbers
}

fn main() {

    let mut f = File::open(INPUT_FILE_NAME).expect("file not found");
    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read input");

    let numbers = str_to_int(input);
    let mut res: Vec<u32> = Vec::new();
    let mut prev = &numbers[numbers.len() - 1];

    for num in &numbers {
        if num == prev {
            res.push(*num);
        }
        prev = num;
    }

    let res: u32 = res.iter().sum();
    println!("{}", res);
}
