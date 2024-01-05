// intersection/lane.rs

use rand::Rng;
use sdl2::keyboard::Keycode;

use crate::{
    vehicle::{ Vehicle },
    algorithm::determine_velocity,
    statistics::Statistics,
    physics::{ get_close_calls_for_vehicle },
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
        let mut close_call_count = 0;

        for (_, car) in self.vehicles.iter_mut().enumerate() {
            let all_cars: Vec<Vehicle> = nc.clone();

            get_close_calls_for_vehicle(car, &all_cars);

            let new_velocity = determine_velocity(car, all_cars);

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

        //divide close call count by two because count happens for both vehicles
        close_call_count = close_call_count / 2;
        self.stats.add_close_call(close_call_count);

        let cars_before = self.vehicles.len().clone();

        for veh in &self.vehicles {
            if !veh.is_in_entire_intersection() {
                self.stats.add_close_call(veh.close_calls.len() as u32);
            }
        }

        //remove vehicles from intersection if out of bounds
        self.vehicles.retain(|v| v.is_in_entire_intersection());

        let cars_after = self.vehicles.len().clone();

        self.stats.add_to_total_vehicles((cars_before as u32) - (cars_after as u32));
    }

    pub fn add_directed_vehicle(&mut self, keycode: Keycode, id: u32) {
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

        self.add_vehicle(origin, direction, id);
    }

    pub fn add_random_vehicle(&mut self, id: u32) {
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

        self.add_vehicle(direction, origin, id)
    }

    fn add_vehicle(&mut self, origin: Direction, direction: Direction, id: u32) {
        let vehicle = Vehicle::new(origin, direction, id);
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
