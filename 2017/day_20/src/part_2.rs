use input;
use particle;

fn remove_collission(particles: &mut Vec<particle::Particle>) -> usize {
  let mut to_be_removed: Vec<usize> = Vec::new();

  for i in 0..particles.len() {
    for j in i + 1..particles.len() {
      let a = &particles[i];
      let b = &particles[j];
      if a.eq(&b) {
        to_be_removed.push(i);
        to_be_removed.push(j);
      }
    }
  }

  to_be_removed.sort();
  to_be_removed.dedup();
  to_be_removed.reverse();

  for i in &to_be_removed {
    &particles.remove(*i);
  }

  to_be_removed.len()
}

pub fn get_survived_particles() {
  let mut particles: Vec<particle::Particle> = input::get_file_input();

  for i in 0..50 {
    particles = particles.iter().map(|p| p.tick()).collect();
    let amount_of_collisions = remove_collission(&mut particles);
    println!("{}", particles.len());
  }
}
