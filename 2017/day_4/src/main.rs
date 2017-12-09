use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

const INPUT_FILE_NAME: &str = "input.txt";

fn is_valid(line: String) -> bool {
    let mut occurences = HashMap::new();
    for word in line.split_whitespace() {
        let count = occurences.entry(word).or_insert(0);
        *count += 1;
    }

    for occ in occurences.values() {
        if *occ != 1 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut res: Vec<u32> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        res.push(is_valid(l) as u32);
    }

    let sum: u32 = res.iter().sum();

    println!("{}", sum);
}
