#[derive(Debug)]
pub struct Condition {
    pub register: String,
    pub op: String,
    pub value: i32,
}

#[derive(Debug)]
pub struct Instruction {
    pub register: String,
    pub action: String,
    pub value: i32,
    pub condition: Condition,
}

impl Instruction {
    pub fn new(line: &String) -> Instruction {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let cnd = Condition {
            register: String::from(tokens[4]),
            op: String::from(tokens[5]),
            value: tokens[6].parse::<i32>().unwrap(),
        };
        let inst = Instruction {
            register: String::from(tokens[0]),
            action: String::from(tokens[1]),
            value: tokens[2].parse::<i32>().unwrap(),
            condition: cnd,
        };

        inst
    }
}

#[test]
fn create_instruction_1() {
    let input = String::from("b inc 5 if a > 1");
    let instr = Instruction::new(&input);

    assert_eq!(instr.register, "b");
    assert_eq!(instr.action, "inc");
    assert_eq!(instr.value, 5);
    assert_eq!(instr.condition.register, "a");
    assert_eq!(instr.condition.value, 1);
}

#[test]
fn create_instruction_2() {
    let input = String::from("a inc 1 if b < 5");
    let instr = Instruction::new(&input);

    assert_eq!(instr.register, "a");
    assert_eq!(instr.action, "inc");
    assert_eq!(instr.value, 1);
    assert_eq!(instr.condition.register, "b");
    assert_eq!(instr.condition.value, 5);
}
