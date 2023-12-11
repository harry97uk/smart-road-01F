// intersection/lane.rs

use rand::Rng;
use sdl2::keyboard::Keycode;

use crate::{
    vehicle::{ Vehicle, VEHICLE_WIDTH, VEHICLE_HEIGHT },
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    physics::{ will_vehicles_collide },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Intersection {
    // Define any necessary fields for the intersection
    // For example, a collection of vehicles currently in the intersection
    pub lanes: [Lane; 12],
    pub vehicles: Vec<Vehicle>,
}

impl Intersection {
    pub fn new() -> Self {
        let lanes = [
            Lane::new(Direction::North, Direction::South),
            Lane::new(Direction::North, Direction::East),
            Lane::new(Direction::North, Direction::West),
            Lane::new(Direction::South, Direction::North),
            Lane::new(Direction::South, Direction::East),
            Lane::new(Direction::South, Direction::West),
            Lane::new(Direction::East, Direction::North),
            Lane::new(Direction::East, Direction::South),
            Lane::new(Direction::East, Direction::West),
            Lane::new(Direction::West, Direction::North),
            Lane::new(Direction::West, Direction::South),
            Lane::new(Direction::West, Direction::East),
        ];
        Self {
            lanes,
            vehicles: vec![],
        }
    }

    pub fn update(&mut self) {
        // Update the state of the intersection
        // For example, check for collisions and update vehicle positions
        // Implement the smart intersection strategy here
        let nc = self.vehicles.clone();
        for (_, car) in self.vehicles.iter_mut().enumerate() {
            let mut new_velocity = 4.0;
            let mut cars_after: Vec<Vehicle> = nc.clone();

            //give priority to cars closer to finishing
            cars_after.retain(
                |&c|
                    !c.is_in_end_lane() &&
                    c.position != car.position &&
                    c.get_distance_to_finish() < car.get_distance_to_finish()
            );

            if !car.is_in_end_lane() {
                for other_car in &mut cars_after {
                    while will_vehicles_collide(car, other_car) {
                        // Reduce velocity by 0.5, with a minimum of 1.0
                        new_velocity = (car.velocity - 1.0).max(1.0);

                        car.set_velocity(new_velocity);

                        // Check again for collision with the updated velocity
                        if !will_vehicles_collide(car, other_car) || new_velocity == 1.0 {
                            break;
                        }
                    }
                }
            }

            car.set_velocity(new_velocity);
            car.update(1.0 / 0.06);
        }

        //remove vehicles from intersection if out of bounds
        self.vehicles.retain(
            |v|
                v.position.x <= (WINDOW_WIDTH as i32) &&
                v.position.x >= 0 - (VEHICLE_WIDTH as i32) &&
                v.position.y <= (WINDOW_HEIGHT as i32) &&
                v.position.y >= 0 - (VEHICLE_HEIGHT as i32)
        );
    }

    pub fn add_directed_vehicle(&mut self, keycode: Keycode) {
        let mut origin = Direction::South;
        let mut directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West
        ];

        match keycode {
            Keycode::Up => {
                origin = Direction::South;
            }
            Keycode::Down => {
                origin = Direction::North;
            }
            Keycode::Left => {
                origin = Direction::East;
            }
            Keycode::Right => {
                origin = Direction::West;
            }
            _ => (),
        }

        directions.retain(|&d| d != origin);
        let random_index = rand::thread_rng().gen_range(0..directions.len());
        let direction = directions[random_index];

        self.add_vehicle(origin, direction);
    }

    pub fn add_random_vehicle(&mut self) {
        //get random direction and set origin
        let mut directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West
        ];
        let mut random_index = rand::thread_rng().gen_range(0..directions.len());
        let origin = directions[random_index];

        //remove origin direction from choosable directions and set output direction
        directions.retain(|&d| d != origin);
        random_index = rand::thread_rng().gen_range(0..directions.len());
        let direction = directions[random_index];

        self.add_vehicle(direction, origin)
    }

    fn add_vehicle(&mut self, origin: Direction, direction: Direction) {
        let vehicle = Vehicle::new(origin, direction);
        let lane: &mut Lane = self.lanes
            .iter_mut()
            .find(|l| l.direction == direction && l.origin == origin)
            .expect("could not find correct lane");
        lane.add_vehicle(vehicle);
        self.vehicles.push(vehicle);
    }

    // Add more methods as needed for intersection behavior
}

pub struct Lane {
    pub origin: Direction,
    pub direction: Direction,
    pub vehicles: Vec<Vehicle>,
}

impl Lane {
    pub fn new(origin: Direction, direction: Direction) -> Self {
        Self {
            origin,
            direction,
            vehicles: Vec::new(),
        }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    // Add more methods as needed for lane-specific behavior
}
