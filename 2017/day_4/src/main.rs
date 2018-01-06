use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod passphrase;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let mut dups: Vec<u32> = Vec::new();
    let mut anagrams: Vec<u32> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        dups.push(!passphrase::has_dups(&l) as u32);
        anagrams.push(!passphrase::has_anagram(&l) as u32);
    }

    let sum: u32 = dups.iter().sum();
    let sum2: u32 = anagrams.iter().sum();

    println!("{}", sum);
    println!("{}", sum2);
}
