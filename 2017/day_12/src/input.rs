use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILE_NAME: &str = "input.txt";

pub fn parse_line(line: &str) -> Vec<i32> {
    line.split(|c| c == '<' || c == '>' || c == '-' || c == ',' || c == ' ')
        .filter_map(|c| c.parse::<i32>().ok())
        .collect()
}

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
        String::from("0 <-> 2"),
        String::from("1 <-> 1"),
        String::from("2 <-> 0, 3, 4"),
        String::from("3 <-> 2, 4"),
        String::from("4 <-> 2, 3, 6"),
        String::from("5 <-> 6"),
        String::from("6 <-> 4, 5"),
    ];
}

#[test]
fn parse_line_test() {
    let line = "2 <-> 0, 3, 4";
    let res = parse_line(line);
    assert_eq!(res, [2, 0, 3, 4]);
}
