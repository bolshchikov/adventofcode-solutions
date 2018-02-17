use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILE_NAME: &str = "input.txt";

pub fn get_input() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        res.push(line.unwrap());
    }

    res
}
pub fn get_example_input() -> Vec<String> {
    return vec![
        String::from("b inc 5 if a > 1"),
        String::from("a inc 1 if b < 5"),
        String::from("c dec -10 if a >= 1"),
        String::from("c inc -20 if c == 10"),
    ];
}
