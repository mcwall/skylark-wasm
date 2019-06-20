use std::fmt;
use super::emulator;

pub struct DisplayFrame {
    pixels: Vec<bool>
}

impl DisplayFrame {
    pub fn new() -> DisplayFrame {
        let pixels = vec![false; (emulator::WIDTH * emulator::HEIGHT) as usize];

        DisplayFrame {
            pixels
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        ((x % emulator::WIDTH) + (y % emulator::HEIGHT) * emulator::WIDTH) as usize
    }

    pub fn pixels(&self) -> *const bool {
        self.pixels.as_ptr()
    }

    pub fn clear(&mut self) {
        for idx in 0..self.pixels.len() {
            self.pixels[idx] = false;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut change = false;
        // println!("Drawing ({}, {}) h{}", x, y, sprite.len());
        let test: Vec<usize> = sprite.iter().map(|s| { *s as usize }).collect();
        for dy in 0..sprite.len() {
            change |= self.draw_sprite(x, y + dy as u8, sprite[dy as usize]);
        }

        change
    }

    // TODO: This can be improved by simply XORing the sprite with existing byte from ram
    fn draw_sprite(&mut self, x: u8, y: u8, sprite: u8) -> bool {
        let mut change = false;
        for i in 0 .. 8 {
            let index = self.get_index(x as u32 + i as u32, y as u32);
            let new_pixel = match sprite >> (7 - i) & 1 {
                0 => false,
                _ => true
            } ^ self.pixels[index];

            change |= self.pixels[index] && !new_pixel;
            self.pixels[index] = new_pixel;
        }

        change
    }
}

impl fmt::Display for DisplayFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pixels.as_slice().chunks(emulator::WIDTH as usize) {
            for &pixel in row {
                let symbol = if pixel { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
