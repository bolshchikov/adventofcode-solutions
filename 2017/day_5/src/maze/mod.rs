#[cfg(test)]
mod tests;

fn generic_escape(input: &Vec<i32>, mutate_fn: &Fn(i32) -> i32) -> u32 {
    let mut current_index = 0;
    let mut steps: u32 = 0;
    let mut numbers: Vec<i32> = input.clone();

    loop {
        let num = numbers.get_mut(current_index);
        match num {
            None => break,
            Some(item) => {
                current_index = (current_index as i32 + *item) as usize;
                steps += 1;
                *item = mutate_fn(*item);
            }
        }
    }
    steps
}

pub fn escape(input: &Vec<i32>) -> u32 {
    let increase_by_one = |num| num + 1;
    generic_escape(input, &increase_by_one)
}

pub fn escape2(input: &Vec<i32>) -> u32 {
    let mutation_rule = |num| if num >= 3 { num - 1 } else { num + 1 };
    generic_escape(input, &mutation_rule)
}
