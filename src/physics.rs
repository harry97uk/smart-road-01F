use sdl2::rect::Point;

use crate::{ vehicle::{ Vehicle, VEHICLE_HEIGHT, VEHICLE_WIDTH }, WINDOW_WIDTH, WINDOW_HEIGHT };

const SAFETY_GAP: i32 = 4;

pub fn will_vehicles_collide(vehicle_a: &Vehicle, vehicle_b: &Vehicle) -> bool {
    // Define the number of time steps
    let num_time_steps = 1000; // Replace with the desired number of time steps

    // Initial positions
    let mut position_a = vehicle_a.position;
    let mut position_b = vehicle_b.position;

    // Check for collisions at each time step
    for _ in 0..num_time_steps {
        let future_position_a = vehicle_a.get_future_position(&position_a);
        let future_position_b = vehicle_b.get_future_position(&position_b);
        // Calculate future positions after the current time step
        let future_position_a_x = future_position_a.x;
        let future_position_a_y = future_position_a.y;
        let future_position_b_x = future_position_b.x;
        let future_position_b_y = future_position_b.y;

        if
            (future_position_a_x - future_position_b_x).abs() <=
                (vehicle_a.width.max(vehicle_a.width) as i32) + SAFETY_GAP &&
            (future_position_a_y - future_position_b_y).abs() <=
                (vehicle_a.height.max(vehicle_a.width) as i32) + SAFETY_GAP
        {
            // Future collision with safety gap
            return true;
        }

        // Update positions for the next time step
        position_a = future_position_a;
        position_b = future_position_b;
    }
    false
}

pub fn is_closer_to_center(point_a: Point, point_b: Point) -> bool {
    let center_x = (WINDOW_WIDTH as f32) / 2.0;
    let center_y = (WINDOW_HEIGHT as f32) / 2.0;
    let dist_a: f32 = f32
        ::sqrt(((point_a.x as f32) - center_x).powi(2) + ((point_a.y as f32) - center_y).powi(2))
        .into();
    let dist_b: f32 = f32
        ::sqrt(((point_b.x as f32) - center_x).powi(2) + ((point_b.y as f32) - center_y).powi(2))
        .into();

    dist_a <= dist_b
}

pub fn get_close_calls_for_vehicle(car: &mut Vehicle, other_cars: &Vec<Vehicle>) {
    for other_car in other_cars {
        if
            car.id != other_car.id &&
            car.close_calls
                .iter()
                .filter(|cc| cc.id == other_car.id)
                .count() < 1 &&
            (car.position.x - other_car.position.x).abs() <=
                (car.width.max(car.width) as i32) + SAFETY_GAP &&
            (car.position.y - other_car.position.y).abs() <=
                (car.height.max(car.width) as i32) + SAFETY_GAP
        {
            car.close_calls.push(other_car.clone());
        }
    }
}
