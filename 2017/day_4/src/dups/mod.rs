use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn has_dups(line: String) -> bool {
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
    return false;
}
