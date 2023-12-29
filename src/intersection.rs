// intersection/lane.rs

use rand::Rng;
use sdl2::keyboard::Keycode;

use crate::{
    vehicle::{ Vehicle, VEHICLE_WIDTH, VEHICLE_HEIGHT },
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    algorithm::determine_velocity,
    statistics::Statistics,
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
    pub stats: Statistics,
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
        let stats = Statistics::new();
        Self {
            lanes,
            vehicles: vec![],
            stats,
        }
    }

    pub fn update(&mut self) {
        let nc = self.vehicles.clone();

        for (_, car) in self.vehicles.iter_mut().enumerate() {
            let cars_after: Vec<Vehicle> = nc.clone();

            let new_velocity = determine_velocity(car, cars_after);

            if new_velocity > self.stats.max_velocity {
                self.stats.set_max_velocity(new_velocity);
            }
            if new_velocity < self.stats.min_velocity {
                self.stats.set_min_velocity(new_velocity);
            }

            car.set_velocity(new_velocity);
            car.update(1.0 / 0.06);

            if !car.is_in_entire_intersection() {
                if car.time > self.stats.max_time {
                    self.stats.set_max_time(car.time);
                }
                if car.time < self.stats.min_time || self.stats.min_time < 0.0 {
                    self.stats.set_min_time(car.time);
                }
            }
        }

        let cars_before = self.vehicles.len().clone();

        //remove vehicles from intersection if out of bounds
        self.vehicles.retain(|v| v.is_in_entire_intersection());

        let cars_after = self.vehicles.len().clone();

        self.stats.add_to_total_vehicles((cars_before as u32) - (cars_after as u32));
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

    pub fn add_remaining_finished_vehicles(&mut self) {
        self.stats.add_to_total_vehicles(
            self.vehicles
                .iter()
                .filter(|v| v.is_in_end_lane())
                .count() as u32
        );
    }

    pub fn find_min_max_times(&mut self) {
        self.stats.set_max_time(
            self.vehicles
                .iter()
                .max_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
                .unwrap().time
        );
        self.stats.set_min_time(
            self.vehicles
                .iter()
                .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
                .unwrap().time
        )
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
