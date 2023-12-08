// intersection/lane.rs

use rand::Rng;
use sdl2::keyboard::Keycode;

use crate::{ vehicle::{ Vehicle, VEHICLE_WIDTH, VEHICLE_HEIGHT }, WINDOW_WIDTH, WINDOW_HEIGHT };

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
        for car in &mut self.vehicles {
            if car.is_car_in_front(nc.clone()) {
                car.set_velocity(1);
            } else {
                car.set_velocity(3);
            }

            car.update(1000 / 60);
        }
        self.vehicles.retain(
            |v|
                v.position.x <= (WINDOW_WIDTH as i32) &&
                v.position.x >= 0 - (VEHICLE_WIDTH as i32) &&
                v.position.y <= (WINDOW_HEIGHT as i32) &&
                v.position.y >= 0 - (VEHICLE_HEIGHT as i32)
        );
    }

    pub fn add_directed_vehicle(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => self.add_vehicle(Direction::South, Direction::North),
            Keycode::Down => self.add_vehicle(Direction::North, Direction::South),
            Keycode::Left => self.add_vehicle(Direction::East, Direction::West),
            Keycode::Right => self.add_vehicle(Direction::West, Direction::East),
            _ => (),
        }
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
