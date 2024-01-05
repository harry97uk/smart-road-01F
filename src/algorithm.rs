use crate::{ vehicle::Vehicle, physics::will_vehicles_collide };

pub fn determine_velocity(car: &mut Vehicle, mut all_cars: Vec<Vehicle>) -> f32 {
    let mut new_velocity: f32 = 3.0;
    car.colliding = false;

    //give priority to cars closer to finishing
    all_cars.retain(
        |c|
            !c.is_in_end_lane() &&
            c.position != car.position &&
            c.get_distance_to_finish() <= car.get_distance_to_finish()
    );

    let mut cars_after = all_cars;

    if !car.is_in_end_lane() {
        for other_car in &mut cars_after {
            while will_vehicles_collide(car, other_car) {
                // Reduce velocity by 0.5, with a minimum of 1.0
                new_velocity = (car.velocity - 1.0).max(1.0);

                car.set_velocity(new_velocity);

                // Check again for collision with the updated velocity
                if new_velocity == 1.0 {
                    if will_vehicles_collide(car, other_car) {
                        new_velocity = 4.0;
                        car.set_velocity(new_velocity);
                        if will_vehicles_collide(car, other_car) {
                            new_velocity = 1.0;
                        }
                        car.colliding = true;
                        other_car.colliding = true;
                    }
                    return new_velocity;
                }
            }
        }
    }
    new_velocity
}
