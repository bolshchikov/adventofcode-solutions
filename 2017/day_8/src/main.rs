use std::collections::HashMap;
mod input;
mod instruction;

fn main() {
    let mut registry: HashMap<String, i32> = HashMap::new();

    let instructions: Vec<instruction::Instruction> = input::get_input()
        .iter()
        .map(|line| instruction::Instruction::new(line))
        .collect();

    for i in &instructions {
        registry.insert(i.register.clone(), 0);
    }

    for j in &instructions {
        let condition = match j.condition.op.as_ref() {
            ">" => *registry.get(&j.condition.register).unwrap() > j.condition.value,
            "<" => *registry.get(&j.condition.register).unwrap() < j.condition.value,
            "<=" => *registry.get(&j.condition.register).unwrap() <= j.condition.value,
            ">=" => *registry.get(&j.condition.register).unwrap() >= j.condition.value,
            "==" => *registry.get(&j.condition.register).unwrap() == j.condition.value,
            "!=" => *registry.get(&j.condition.register).unwrap() != j.condition.value,
            _ => panic!("unknown operand"),
        };

        if condition {
            match j.action.as_ref() {
                "inc" => {
                    let new_val = *registry.get(&j.register).unwrap() + j.value;
                    registry.insert(j.register.clone(), new_val);
                }
                "dec" => {
                    let new_val = *registry.get(&j.register).unwrap() - j.value;
                    registry.insert(j.register.clone(), new_val);
                }
                _ => panic!("unknown operation"),
            }
        }
        // println!("{:?}", registry);
    }
    println!("{:?}", registry.values().max().unwrap());
}
