extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const N_KEYS: usize = 16;

pub struct Keyboard {
    keys: Vec<bool>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let keys = vec![false; N_KEYS];

        Keyboard {
            keys
        }
    }

    pub fn key_change(&mut self, key: usize, pressed: bool) {
        log!("Key change: {:?}", self.keys);
        self.keys[key] = pressed;
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn current_key(&self) -> Option<usize> {
        self.keys.iter().position(|&k| k)
    }
}
