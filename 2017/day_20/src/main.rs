use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod particle;

const INPUT_FILE_NAME: &str = "input.txt";

fn input_to_arr() -> Vec<particle::Particle> {
    let mut res: Vec<particle::Particle> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        res.push(particle::create_particle(line.unwrap()));
    }

    res
}

fn get_input() -> Vec<particle::Particle> {
    let p1 = particle::create_particle(String::from("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>"));
    let p2 = particle::create_particle(String::from("p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"));

    vec![p1, p2]
}

fn main() {
    let mut particles: Vec<particle::Particle> = input_to_arr();
    let mut index: i32 = -1;
    let mut stable_iterations: Vec<i32> = Vec::new();

    for i in 0..particles.len() + 1 {
        stable_iterations.push(i as i32);
    }

    let is_stable = |arr: &Vec<i32>| -> bool {
        let mut cloned = arr.clone();
        cloned.dedup();
        cloned.len() == 1
    };

    let min_index = |arr: Vec<i32>| -> i32 {
        let mut min: i32 = <i32>::max_value();
        let mut index: usize = <usize>::max_value();
        for (idx, item) in arr.iter().enumerate() {
            if *item < min {
                min = *item;
                index = idx;
            }
        }

        index as i32
    };

    while !is_stable(&stable_iterations) {
        index += 1;
        particles = particles.iter().map(|p| p.tick()).collect();

        let distances: Vec<i32> = particles
            .iter()
            .map(|p| p.distance())
            .collect();

        let attempt = index as usize % stable_iterations.len();
        stable_iterations[attempt] = min_index(distances);
    }
    println!("{:?}", stable_iterations[0]);
    println!("{:?}", index);
}
