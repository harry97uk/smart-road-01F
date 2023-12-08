use sdl2::rect::Point;

use crate::{
    intersection::{ Direction, Intersection },
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    render::{ VERTICAL_LANE_WIDTH, HORIZONTAL_LANE_HEIGHT },
};

pub const VEHICLE_WIDTH: u32 = VERTICAL_LANE_WIDTH;
pub const VEHICLE_HEIGHT: u32 = VERTICAL_LANE_WIDTH;

#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
    pub position: Point,
    pub time: i32,
    pub distance: i32,
    pub velocity: i32,
    pub origin: Direction,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(origin: Direction, direction: Direction) -> Self {
        let (mut x, mut y) = (0, 0);

        match (origin, direction) {
            (Direction::South, Direction::North) => {
                y = WINDOW_HEIGHT - VEHICLE_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 2;
            }
            (Direction::South, Direction::East) => {
                y = WINDOW_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH;
            }
            (Direction::South, Direction::West) => {
                y = WINDOW_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 3;
            }
            (Direction::East, Direction::South) => {
                x = WINDOW_WIDTH;
                y = WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT * 2;
            }
            (Direction::East, Direction::North) => {
                x = WINDOW_WIDTH;
                y = WINDOW_HEIGHT / 3;
            }
            (Direction::East, Direction::West) => {
                x = WINDOW_WIDTH - VEHICLE_WIDTH;
                y = WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT;
            }
            (Direction::North, Direction::South) => {
                x = WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH;
            }
            (Direction::North, Direction::East) => {
                x = WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH * 2;
            }
            (Direction::North, Direction::West) => {
                x = WINDOW_WIDTH / 3;
            }
            (Direction::West, Direction::North) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 3;
            }
            (Direction::West, Direction::East) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 2;
            }
            (Direction::West, Direction::South) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT;
            }
            _ => {}
        }

        Self {
            position: Point::new(x as i32, y as i32),
            time: 0,
            distance: 0,
            velocity: 1,
            origin,
            direction,
        }
    }

    pub fn update(&mut self, delta_time: i32) {
        // Update the vehicle's state based on physics rules
        self.time += delta_time;
        self.distance += self.velocity * delta_time;

        //update position
        self.update_position();
    }

    pub fn set_velocity(&mut self, velocity: i32) {
        // Set the velocity of the vehicle
        self.velocity = velocity;
    }

    pub fn is_in_intersection(&self) -> bool {
        self.position.x >= (WINDOW_WIDTH as i32) / 3 &&
            self.position.x <= ((WINDOW_WIDTH as i32) * 2) / 3 &&
            self.position.y >= (WINDOW_HEIGHT as i32) / 3 &&
            self.position.y <= ((WINDOW_HEIGHT as i32) * 2) / 3
    }

    pub fn is_car_in_front(&self, vehicles: Vec<Vehicle>) -> bool {
        match self.origin {
            Direction::North => {
                if
                    let None = vehicles
                        .iter()
                        .find(|c| c.position.x == self.position.x && c.position.y > self.position.y)
                {
                    return false;
                }
            }
            Direction::South => {
                if
                    let None = vehicles
                        .iter()
                        .find(|c| c.position.x == self.position.x && c.position.y < self.position.y)
                {
                    return false;
                }
            }
            Direction::East => {
                if
                    let None = vehicles
                        .iter()
                        .find(|c| c.position.y == self.position.y && c.position.x < self.position.x)
                {
                    return false;
                }
            }
            Direction::West => {
                if
                    let None = vehicles
                        .iter()
                        .find(|c| c.position.y == self.position.y && c.position.x > self.position.x)
                {
                    return false;
                }
            }
        }
        true
    }

    fn update_position(&mut self) {
        if !self.has_reached_turning_point() {
            match self.origin {
                Direction::North => {
                    self.position.y += self.velocity;
                }
                Direction::South => {
                    self.position.y -= self.velocity;
                }
                Direction::East => {
                    self.position.x -= self.velocity;
                }
                Direction::West => {
                    self.position.x += self.velocity;
                }
            }
        } else {
            match self.direction {
                Direction::North => {
                    self.position.y -= self.velocity;
                }
                Direction::South => {
                    self.position.y += self.velocity;
                }
                Direction::East => {
                    self.position.x += self.velocity;
                }
                Direction::West => {
                    self.position.x -= self.velocity;
                }
            }
        }
    }

    fn has_reached_turning_point(&self) -> bool {
        match (self.origin, self.direction) {
            (Direction::North, Direction::South) => true,
            (Direction::North, Direction::East) =>
                self.position.y >= (((WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 3) as i32),
            (Direction::North, Direction::West) => self.position.y >= ((WINDOW_HEIGHT / 3) as i32),
            (Direction::South, Direction::North) => true,
            (Direction::South, Direction::East) =>
                self.position.y <= (((WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT) as i32),
            (Direction::South, Direction::West) =>
                self.position.y <= ((WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT * 2) as i32),
            (Direction::East, Direction::North) =>
                self.position.x <= (((WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH) as i32),
            (Direction::East, Direction::South) =>
                self.position.x <= ((WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH * 2) as i32),
            (Direction::East, Direction::West) => true,
            (Direction::West, Direction::North) =>
                self.position.x >= (((WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 3) as i32),
            (Direction::West, Direction::South) => self.position.x >= ((WINDOW_WIDTH / 3) as i32),
            (Direction::West, Direction::East) => true,
            _ => true,
        }
    }

    // Add more methods as needed for vehicle-specific behavior
}
