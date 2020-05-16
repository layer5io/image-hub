
pub struct RateLimiter {
    RPM: u32,
    Min: i32,
    pub Count: u32, 
}

impl RateLimiter {
    pub const fn new(RPM: u32) -> Self {
        Self {
            RPM: RPM,
            Min: -1,
            Count: 0,
        }
    }
    pub fn update(&mut self, time: i32) -> bool {
        if self.Min != time {
            self.Min = time;
            self.Count = 0;
        }
        self.Count += 1;
        if self.Count > self.RPM {
            return false
        }
        true
    }
}