const BASE_RATE: f64 = 221.0;

fn get_success_rate_by_speed(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        (1..=4) => 1.0,
        (5..=8) => 0.9,
        (9..=u8::MAX) => 0.77,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    BASE_RATE * (speed as f64) * get_success_rate_by_speed(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour = production_rate_per_hour(speed);

    (per_hour / 60.0) as u32
}
