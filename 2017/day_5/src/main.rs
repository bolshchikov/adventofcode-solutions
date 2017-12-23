use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod maze;

const INPUT_FILE_NAME: &str = "input.txt";

fn input_to_arr() -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap().parse().expect("Failed to parse");
        res.push(l);
    }
    res
}

fn main() {
    let numbers: Vec<i32> = input_to_arr();
    let steps = maze::escape(&numbers);
    let steps2 = maze::escape2(&numbers);
    println!("{}", steps);
    println!("{}", steps2);
}
