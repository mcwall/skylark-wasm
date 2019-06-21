const RATE: f64 = 60.0;

pub struct Timer {
    start_time: u64,
    start_val: u8
}

impl Timer {
    pub fn new() -> Timer {
        let start_time = 0;
        let start_val = 0;

        Timer {
            start_time,
            start_val
        }
    }

    pub fn get(&mut self, current_time: u64) -> u8 {
        if self.start_val == 0 || self.start_time == 0 {
            return 0;
        };
        
        let ellapsed = (current_time - self.start_time) as f64;
        let new_val = (self.start_val as f64 - (ellapsed / RATE)) as u8;

        if new_val <= 0 || new_val > self.start_val {
            self.start_val = 0;
            self.start_time = 0;
        }

        new_val
    }

    pub fn set(&mut self, val: u8, current_time: u64) {
        self.start_val = val;
        self.start_time = current_time;
    }
}
