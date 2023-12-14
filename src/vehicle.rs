use sdl2::rect::Point;

use crate::{
    intersection::Direction,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    render::{ VERTICAL_LANE_WIDTH, HORIZONTAL_LANE_HEIGHT },
};

pub const VEHICLE_WIDTH: u32 = (VERTICAL_LANE_WIDTH * 3) / 2;
pub const VEHICLE_HEIGHT: u32 = VERTICAL_LANE_WIDTH;

#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
    pub position: Point,
    pub width: u32,
    pub height: u32,
    pub time: f32,
    pub distance: f32,
    pub velocity: f32,
    pub origin: Direction,
    pub direction: Direction,
    pub facing: Direction,
    pub colliding: bool,
}

impl Vehicle {
    pub fn new(origin: Direction, direction: Direction) -> Self {
        let (mut x, mut y) = (0, 0);
        let mut facing = Direction::North;

        match (origin, direction) {
            (Direction::South, Direction::North) => {
                y = WINDOW_HEIGHT - VEHICLE_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 2;
                facing = Direction::North;
            }
            (Direction::South, Direction::East) => {
                y = WINDOW_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH;
                facing = Direction::North;
            }
            (Direction::South, Direction::West) => {
                y = WINDOW_HEIGHT;
                x = (WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 3;
                facing = Direction::North;
            }
            (Direction::East, Direction::South) => {
                x = WINDOW_WIDTH;
                y = WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT * 2;
                facing = Direction::West;
            }
            (Direction::East, Direction::North) => {
                x = WINDOW_WIDTH;
                y = WINDOW_HEIGHT / 3;
                facing = Direction::West;
            }
            (Direction::East, Direction::West) => {
                x = WINDOW_WIDTH - VEHICLE_WIDTH;
                y = WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT;
                facing = Direction::West;
            }
            (Direction::North, Direction::South) => {
                x = WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH;
                facing = Direction::South;
            }
            (Direction::North, Direction::East) => {
                x = WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH * 2;
                facing = Direction::South;
            }
            (Direction::North, Direction::West) => {
                x = WINDOW_WIDTH / 3;
                facing = Direction::South;
            }
            (Direction::West, Direction::North) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 3;
                facing = Direction::East;
            }
            (Direction::West, Direction::East) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 2;
                facing = Direction::East;
            }
            (Direction::West, Direction::South) => {
                y = (WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT;
                facing = Direction::East;
            }
            _ => {}
        }

        Self {
            position: Point::new(x as i32, y as i32),
            width: VEHICLE_WIDTH,
            height: VEHICLE_HEIGHT,
            time: 0.0,
            distance: 0.0,
            velocity: 1.0,
            origin,
            direction,
            facing,
            colliding: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update the vehicle's state based on physics rules
        self.time += delta_time;
        self.distance += self.velocity * delta_time;

        //update position
        self.update_position();
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        // Set the velocity of the vehicle
        self.velocity = velocity;
    }

    pub fn is_in_end_lane(&self) -> bool {
        match self.direction {
            Direction::North => {
                return self.position.y <= (WINDOW_HEIGHT as i32) / 3;
            }
            Direction::South => {
                return self.position.y >= ((WINDOW_HEIGHT * 2) as i32) / 3;
            }
            Direction::East => {
                return self.position.x >= ((WINDOW_WIDTH * 2) as i32) / 3;
            }
            Direction::West => {
                return self.position.x <= (WINDOW_WIDTH as i32) / 3;
            }
        }
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
                    self.width = VEHICLE_HEIGHT;
                    self.height = VEHICLE_WIDTH;
                    self.position.y += self.velocity as i32;
                }
                Direction::South => {
                    self.width = VEHICLE_HEIGHT;
                    self.height = VEHICLE_WIDTH;
                    self.position.y -= self.velocity as i32;
                }
                Direction::East => {
                    self.width = VEHICLE_WIDTH;
                    self.height = VEHICLE_HEIGHT;
                    self.position.x -= self.velocity as i32;
                }
                Direction::West => {
                    self.width = VEHICLE_WIDTH;
                    self.height = VEHICLE_HEIGHT;
                    self.position.x += self.velocity as i32;
                }
            }
        } else {
            match self.direction {
                Direction::North => {
                    self.facing = Direction::North;
                    self.width = VEHICLE_HEIGHT;
                    self.height = VEHICLE_WIDTH;
                    self.position.y -= self.velocity as i32;
                }
                Direction::South => {
                    self.facing = Direction::South;
                    self.width = VEHICLE_HEIGHT;
                    self.height = VEHICLE_WIDTH;
                    self.position.y += self.velocity as i32;
                }
                Direction::East => {
                    self.facing = Direction::East;
                    self.width = VEHICLE_WIDTH;
                    self.height = VEHICLE_HEIGHT;
                    self.position.x += self.velocity as i32;
                }
                Direction::West => {
                    self.facing = Direction::West;
                    self.width = VEHICLE_WIDTH;
                    self.height = VEHICLE_HEIGHT;
                    self.position.x -= self.velocity as i32;
                }
            }
        }
    }

    pub fn get_future_position(&self, position: &Point) -> Point {
        let mut future_pos = Point::new(position.x, position.y);
        if !self.has_reached_future_turning_point(&future_pos) {
            match self.origin {
                Direction::North => {
                    future_pos.y = position.y + (self.velocity as i32);
                }
                Direction::South => {
                    future_pos.y = position.y - (self.velocity as i32);
                }
                Direction::East => {
                    future_pos.x = position.x - (self.velocity as i32);
                }
                Direction::West => {
                    future_pos.x = position.x + (self.velocity as i32);
                }
            }
        } else {
            match self.direction {
                Direction::North => {
                    future_pos.y = position.y - (self.velocity as i32);
                }
                Direction::South => {
                    future_pos.y = position.y + (self.velocity as i32);
                }
                Direction::East => {
                    future_pos.x = position.x + (self.velocity as i32);
                }
                Direction::West => {
                    future_pos.x = position.x - (self.velocity as i32);
                }
            }
        }
        future_pos
    }

    pub fn has_reached_turning_point(&self) -> bool {
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

    fn has_reached_future_turning_point(&self, position: &Point) -> bool {
        match (self.origin, self.direction) {
            (Direction::North, Direction::South) => true,
            (Direction::North, Direction::East) =>
                position.y >= (((WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT * 3) as i32),
            (Direction::North, Direction::West) => position.y >= ((WINDOW_HEIGHT / 3) as i32),
            (Direction::South, Direction::North) => true,
            (Direction::South, Direction::East) =>
                position.y <= (((WINDOW_HEIGHT * 2) / 3 - HORIZONTAL_LANE_HEIGHT) as i32),
            (Direction::South, Direction::West) =>
                position.y <= ((WINDOW_HEIGHT / 3 + HORIZONTAL_LANE_HEIGHT * 2) as i32),
            (Direction::East, Direction::North) =>
                position.x <= (((WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH) as i32),
            (Direction::East, Direction::South) =>
                position.x <= ((WINDOW_WIDTH / 3 + VERTICAL_LANE_WIDTH * 2) as i32),
            (Direction::East, Direction::West) => true,
            (Direction::West, Direction::North) =>
                position.x >= (((WINDOW_WIDTH * 2) / 3 - VERTICAL_LANE_WIDTH * 3) as i32),
            (Direction::West, Direction::South) => position.x >= ((WINDOW_WIDTH / 3) as i32),
            (Direction::West, Direction::East) => true,
            _ => true,
        }
    }

    pub fn get_distance_to_finish(&self) -> u32 {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        match (self.origin, self.direction) {
            (Direction::North, Direction::South) => {
                dy = ((WINDOW_HEIGHT as i32) * 2) / 3 - self.position.y;
            }
            (Direction::North, Direction::East) => {
                dx = ((WINDOW_WIDTH as i32) * 2) / 3 - self.position.x;
                dy =
                    ((WINDOW_HEIGHT as i32) * 2) / 3 -
                    (HORIZONTAL_LANE_HEIGHT as i32) * 3 -
                    self.position.y;
            }
            (Direction::North, Direction::West) => {
                dx = self.position.x - (WINDOW_WIDTH as i32) / 3;
                dy = self.position.y - (WINDOW_HEIGHT as i32) / 3;
            }
            (Direction::South, Direction::North) => {
                dy = self.position.y - (WINDOW_HEIGHT as i32) / 3;
            }
            (Direction::South, Direction::East) => {
                dx = ((WINDOW_WIDTH as i32) * 2) / 3 - self.position.x;
                dy =
                    self.position.y -
                    ((WINDOW_HEIGHT as i32) * 2) / 3 -
                    (HORIZONTAL_LANE_HEIGHT as i32);
            }
            (Direction::South, Direction::West) => {
                dx = self.position.x - (WINDOW_WIDTH as i32) / 3;
                dy =
                    self.position.y -
                    (WINDOW_HEIGHT as i32) / 3 +
                    (HORIZONTAL_LANE_HEIGHT as i32) * 2;
            }
            (Direction::East, Direction::North) => {
                dx =
                    self.position.x -
                    ((WINDOW_WIDTH as i32) * 2) / 3 -
                    (VERTICAL_LANE_WIDTH as i32);
                dy = self.position.y - (WINDOW_HEIGHT as i32) / 3;
            }
            (Direction::East, Direction::South) => {
                dx = self.position.x - (WINDOW_WIDTH as i32) / 3 + (VERTICAL_LANE_WIDTH as i32) * 2;
                dy = ((WINDOW_HEIGHT as i32) * 2) / 3 - self.position.y;
            }
            (Direction::East, Direction::West) => {
                dx = self.position.x - (WINDOW_WIDTH as i32) / 3;
            }
            (Direction::West, Direction::North) => {
                dx =
                    ((WINDOW_WIDTH as i32) * 2) / 3 -
                    (VERTICAL_LANE_WIDTH as i32) * 3 -
                    self.position.x;
                dy = self.position.y - (WINDOW_HEIGHT as i32) / 3;
            }
            (Direction::West, Direction::South) => {
                dx = (WINDOW_WIDTH as i32) / 3 - self.position.x;
                dy = ((WINDOW_HEIGHT as i32) * 2) / 3 - self.position.y;
            }
            (Direction::West, Direction::East) => {
                dx = ((WINDOW_WIDTH as i32) * 2) / 3 - self.position.x;
            }
            _ => (),
        }
        (dx.abs() + dy.abs()) as u32
    }

    // Add more methods as needed for vehicle-specific behavior
}
