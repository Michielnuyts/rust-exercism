use std::fmt::Display;

pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: minutes + (hours * 60),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> &mut Self {
        self.minutes += minutes;
        self
    }

    fn convert(&self) -> (i32, i32) {
        let hours = (self.minutes / 60).rem_euclid(24);
        let minutes = self.minutes.rem_euclid(60);

        (hours, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hours, minutes) = self.convert();

        let hours = format!("{:0>2}", hours).replace("24", "00");
        let minutes = format!("{:0>2}", minutes);

        write!(f, "{}:{}", hours, minutes)
    }
}
