use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILE_NAME: &str = "input.txt";

fn input_to_int() -> Vec<i32> {
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
    let mut current_index = 0;
    let mut steps: u32 = 0;
    let mut numbers: Vec<i32> = input_to_int();

    loop {
        let num = numbers.get_mut(current_index);
        match num {
            None => break,
            Some(item) => {
                current_index = (current_index as i32 + *item) as usize;
                steps += 1;
                *item += 1;
            }
        }
    }
    println!("{}", steps);
}
