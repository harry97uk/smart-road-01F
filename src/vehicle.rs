// vehicles/vehicle.rs

use sdl2::rect::{ Rect, Point };

use crate::intersection::Direction;

#[derive(Debug)]
pub struct Vehicle {
    pub bounds: Rect,
    pub position: Point,
    pub time: f64,
    pub distance: f64,
    pub velocity: f64,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(origin: Direction, direction: Direction) -> Self {
        Self {
            bounds: Rect::new(0, 0, 100, 80),
            position: Point::new(0, 0),
            time: 0.0,
            distance: 0.0,
            velocity: 0.0,
            direction,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Update the vehicle's state based on physics rules
        self.time += delta_time;
        self.distance += self.velocity * delta_time;
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        // Set the velocity of the vehicle
        self.velocity = velocity;
    }

    // Add more methods as needed for vehicle-specific behavior
}
