use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn div_mod(a: i32, b: i32) -> (i32, i32) {
    let mut q = a / b;
    let mut rem = a - q * b;
    if rem < 0 {
        rem += b;
        q -= 1;
    }
    (q, rem)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h_carry, m) = div_mod(minutes, 60);
        let (_, h) = div_mod(hours + h_carry, 24);
        Clock{hours: h, minutes: m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}