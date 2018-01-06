use std::collections::HashMap;
use std::iter::FromIterator;

#[cfg(test)]
mod tests;

pub fn has_dups(line: &str) -> bool {
    let mut occurences = HashMap::new();
    for word in line.split_whitespace() {
        let count = occurences.entry(word).or_insert(0);
        *count += 1;
    }

    for occ in occurences.values() {
        if *occ != 1 {
            return true;
        }
    }
    false
}


pub fn has_anagram(line: &str) -> bool {
    let mut occurences = HashMap::new();
    for word in line.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        
        let sorted_word = String::from_iter(chars);
        let count = occurences.entry(sorted_word).or_insert(0);
        *count += 1;
    }
    for occ in occurences.values() {
        if *occ != 1 {
            return true;
        }
    }
    false
}
