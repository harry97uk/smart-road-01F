pub struct Statistics {
    pub num_vehicles: u32,
    pub max_velocity: f32,
    pub min_velocity: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub close_calls: u32,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            num_vehicles: 0,
            max_velocity: 3.0,
            min_velocity: 3.0,
            max_time: 0.0,
            min_time: -1.0,
            close_calls: 0,
        }
    }

    pub fn add_to_total_vehicles(&mut self, amount_to_add: u32) {
        self.num_vehicles += amount_to_add;
    }

    pub fn set_max_velocity(&mut self, new_velocity: f32) {
        self.max_velocity = new_velocity;
    }

    pub fn set_min_velocity(&mut self, new_velocity: f32) {
        self.min_velocity = new_velocity;
    }

    pub fn set_max_time(&mut self, new_time: f32) {
        self.max_time = new_time;
    }

    pub fn set_min_time(&mut self, new_time: f32) {
        self.min_time = new_time;
    }

    pub fn add_close_call(&mut self, amount_to_add: u32) {
        self.close_calls += amount_to_add;
    }
}
