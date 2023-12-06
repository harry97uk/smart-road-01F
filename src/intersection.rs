// intersection/lane.rs

use crate::vehicle::Vehicle;

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
        }
    }

    pub fn update(&mut self) {
        // Update the state of the intersection
        // For example, check for collisions and update vehicle positions
        // Implement the smart intersection strategy here
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
