#[cfg(test)]
mod tests;

pub fn spin(input: String, num: i32) -> String {
    let string = String::from(input);
    let split_index = string.len() - (num as usize);
    let part_1 = string.get(..split_index).unwrap();
    let part_2 = string.get(split_index..).unwrap();

    [part_2, part_1].join("")
}

pub fn exchange(input: String, pos_1: i32, pos_2: i32) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    chars.swap(pos_1 as usize, pos_2 as usize);

    chars.into_iter().collect()
}

pub fn partner(input: String, char_1: char, char_2: char) -> String {
    let pos_1 = input.chars().position(|c| c == char_1).unwrap();
    let pos_2 = input.chars().position(|c| c == char_2).unwrap();

    exchange(input, pos_1 as i32, pos_2 as i32)
}
