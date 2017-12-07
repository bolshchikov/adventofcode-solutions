use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let mut difference: Vec<u32> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();

        let max: u32 = l.split_whitespace()
            .map(|s| s.parse().unwrap())
            .max()
            .expect("could not get max value");
        let min: u32 = l.split_whitespace()
            .map(|s| s.parse().unwrap())
            .min()
            .expect("could not get min value");

        difference.push(max - min);
    }

    let res: u32 = difference.iter().sum();
    println!("{}", res);
}
