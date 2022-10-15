use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours + (minutes / 60);

        Self {
            hours: hours.rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> &mut Self {
        let new_mins = self.minutes + minutes;
        let hours = (new_mins / 60).rem_euclid(24);
        let mins = new_mins.rem_euclid(60);

        self.hours += hours;
        self.minutes += mins;

        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = format!("{:0>2}", self.hours).replace("24", "00");
        let minutes = format!("{:0>2}", self.minutes);

        write!(f, "{}:{}", hours, minutes)
    }
}
