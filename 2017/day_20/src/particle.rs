struct Position(i32, i32, i32);
struct Velocity(i32, i32, i32);
struct Acceleration(i32, i32, i32);

pub struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl Particle {
    pub fn tick(&self) -> Particle {
        let new_acceleration = Acceleration(
            self.acceleration.0,
            self.acceleration.1,
            self.acceleration.2,
        );

        let new_velocity = Velocity(
            &self.velocity.0 + new_acceleration.0,
            &self.velocity.1 + new_acceleration.1,
            &self.velocity.2 + new_acceleration.2,
        );

        let new_position = Position(
            &self.position.0 + new_velocity.0,
            &self.position.1 + new_velocity.1,
            &self.position.2 + new_velocity.2,
        );

        Particle {
            position: new_position,
            velocity: new_velocity,
            acceleration: new_acceleration,
        }
    }

    pub fn distance(&self) -> i32 {
        &self.position.0.abs() + &self.position.1.abs() + &self.position.2.abs()
    }
}

pub fn create_particle(coordinates: String) -> Particle {
    let attributes: Vec<&str> = coordinates.split(|c| c == '<' || c == '>').collect();
    let position: Vec<i32> = attributes[1]
        .split(',')
        .map(|num| num.parse().expect("Failed to parse"))
        .collect();

    let velocity: Vec<i32> = attributes[3]
        .split(',')
        .map(|num| num.parse().expect("Failed to parse"))
        .collect();

    let acceleration: Vec<i32> = attributes[5]
        .split(',')
        .map(|num| num.parse().expect("Failed to parse"))
        .collect();


    Particle {
        position: Position(position[0], position[1], position[2]),
        velocity: Velocity(velocity[0], velocity[1], velocity[2]),
        acceleration: Acceleration(acceleration[0], acceleration[1], acceleration[2]),
    }
}
