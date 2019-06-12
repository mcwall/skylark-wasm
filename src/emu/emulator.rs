use std::fmt;
use std::collections::LinkedList;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const PIXELS: usize = 2048;

pub struct Cpu {
    pc: u16,
    i: u8,
    v: [u8; 16],
    stack: LinkedList<u8>
}

impl Cpu {
    pub fn new() -> Cpu {
        let pc = 0x200;
        let i = 0;
        let v = [0; 16];
        let stack = LinkedList::new();

        Cpu {
            pc,
            i,
            v,
            stack
        }
    }
}

pub struct DisplayFrame {
    pixels: [bool; PIXELS]
}

impl DisplayFrame {
    pub fn new() -> DisplayFrame {
        let pixels = [false; PIXELS];

        DisplayFrame {
            pixels
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x * WIDTH + y) as usize
    }

    pub fn set(&mut self, x: u32, y: u32, val: bool) {
        self.pixels[self.get_index(x, y)] = val;
    }
}

impl fmt::Display for DisplayFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pixels[..].chunks(WIDTH as usize) {
            for &pixel in row {
                let symbol = if pixel { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub struct Emulator {
    rom: Vec<u8>,
    ram: Vec<u8>,
    cpu: Cpu
}

impl Emulator {
    pub fn new() -> Emulator {
        let rom = vec![];
        let ram = vec![];
        let cpu = Cpu::new();

        Emulator {
            rom,
            ram,
            cpu
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }
}
