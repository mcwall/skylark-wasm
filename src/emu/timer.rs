pub struct Timer {
    val: u8
}

impl Timer {
    pub fn new() -> Timer {
        let val = 0;

        Timer {
            val
        }
    }

    pub fn get(&self) -> u8 {
        self.val
    }

    pub fn set(&mut self, val: u8) {
        self.val = val;
    }

    pub fn decrement(&mut self) {
        if self.val > 0 {
            self.val -= 1;
        }
    }
}
