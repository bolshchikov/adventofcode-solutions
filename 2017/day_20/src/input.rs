use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use particle;

const INPUT_FILE_NAME: &str = "input.txt";

pub fn get_file_input() -> Vec<particle::Particle> {
    let mut res: Vec<particle::Particle> = Vec::new();
    let f = File::open(INPUT_FILE_NAME).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        res.push(particle::create_particle(line.unwrap()));
    }

    res
}

pub fn _get_example_1_input() -> Vec<particle::Particle> {
    let p1 = particle::create_particle(String::from("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>"));
    let p2 = particle::create_particle(String::from("p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"));

    vec![p1, p2]
}

pub fn _get_example_2_input() -> Vec<particle::Particle> {
    let p1 = particle::create_particle(String::from("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>"));
    let p2 = particle::create_particle(String::from("p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>"));
    let p3 = particle::create_particle(String::from("p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>"));
    let p4 = particle::create_particle(String::from("p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"));

    vec![p1, p2, p3, p4]
}
