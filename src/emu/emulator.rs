extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use super::{ display, cpu, keyboard, timer };

// TODO: Maybe usize? Also, these probably shouln't be public
pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
pub const PRG_OFFSET: usize = 0x200;
pub const RAM_SIZE: usize = 0x1000;
pub const REG_SIZE: usize = 0x10;
pub const FONT_OFFSET: usize = 0x0;
pub const FONT_WIDTH: usize = 5;

#[wasm_bindgen]
pub struct Emulator {
    ram: Vec<u8>,
    cpu: cpu::Cpu,
    display: display::DisplayFrame,
    keyboard: keyboard::Keyboard,
    timer: timer::Timer
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        let ram = vec![0; RAM_SIZE];
        let cpu = cpu::Cpu::new();
        let display = display::DisplayFrame::new();
        let keyboard = keyboard::Keyboard::new();
        let timer = timer::Timer::new();

        Emulator {
            ram,
            cpu,
            display,
            keyboard,
            timer
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.load_fonts();

        for (i, rom_byte) in rom.iter().enumerate() {
            self.ram[PRG_OFFSET + i] = *rom_byte;
        }
    }

    pub fn display_out(&self) -> String {
        self.display.to_string()
    }

    pub fn pixels(&self) -> *const bool {
        self.display.pixels()
    }

    pub fn tick(&mut self) {//, system_time: u64) {
        self.cpu.tick(&mut self.ram, &self.keyboard, &mut self.display, &mut self.timer, 0)//system_time)
    }

    pub fn key_change(&mut self, key: usize, pressed: bool) {
        self.keyboard.key_change(key, pressed)
    }

    pub fn width(&self) -> u32 {
        WIDTH
    }

    pub fn height(&self) -> u32 {
        HEIGHT
    }

    fn load_fonts(&mut self) {
        // TODO: move this data to a proper data file

        // 0
        let mut addr = 0;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0x90; // 1001
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 1
        addr += 5;
        self.ram[addr + 0] = 0x20; // 0010
        self.ram[addr + 1] = 0x60; // 0110
        self.ram[addr + 2] = 0x20; // 0010
        self.ram[addr + 3] = 0x20; // 0010
        self.ram[addr + 4] = 0x70; // 0111

        // 2
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // 3
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // 4
        addr += 5;
        self.ram[addr + 0] = 0x90; // 1001
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0x10; // 0001

        // 5
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // 6
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 7
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0x20; // 0010
        self.ram[addr + 3] = 0x40; // 0100
        self.ram[addr + 4] = 0x40; // 0100

        // 8
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 9
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // A
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0x90; // 1001

        // B
        addr += 5;
        self.ram[addr + 0] = 0xe0; // 1110
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xe0; // 1110
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xe0; // 1110

        // C
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0x80; // 1000
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // D
        addr += 5;
        self.ram[addr + 0] = 0xe0; // 1110
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0x90; // 1001
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xe0; // 1110

        // E
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // F
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0x80; // 1000
    }
}
