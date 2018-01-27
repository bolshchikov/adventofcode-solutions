use input;
use particle;

fn is_stable(arr: &Vec<i32>) -> bool {
    let mut cloned = arr.clone();
    cloned.dedup();
    cloned.len() == 1
}

fn get_min_index(arr: Vec<i32>) -> i32 {
    let mut min: i32 = <i32>::max_value();
    let mut index: usize = <usize>::max_value();
    for (idx, item) in arr.iter().enumerate() {
        if *item < min {
            min = *item;
            index = idx;
        }
    }

    index as i32
}

pub fn get_closest_particle() -> i32 {
    let mut particles: Vec<particle::Particle> = input::get_file_input();
    let mut index: i32 = -1;
    let mut stable_iterations: Vec<i32> = Vec::new();

    for i in 0..particles.len() + 1 {
        stable_iterations.push(i as i32);
    }

    while !is_stable(&stable_iterations) {
        index += 1;
        particles = particles.iter().map(|p| p.tick()).collect();

        let distances: Vec<i32> = particles.iter().map(|p| p.distance()).collect();

        let attempt = index as usize % stable_iterations.len();
        stable_iterations[attempt] = get_min_index(distances);
    }

    stable_iterations[0]
}
