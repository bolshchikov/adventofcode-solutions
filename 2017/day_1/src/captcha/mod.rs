#[cfg(test)]
mod tests_sum;
mod tests_sum_by_half;

pub fn sum(numbers: &Vec<u32>) -> u32 {
    let mut res: Vec<u32> = Vec::new();
    let mut prev = &numbers[numbers.len() - 1];

    for num in numbers {
        if num == prev {
            res.push(*num);
        }
        prev = num;
    }

    res.iter().sum()
}

pub fn sum_by_half(numbers: &Vec<u32>) -> u32 {
    0
}